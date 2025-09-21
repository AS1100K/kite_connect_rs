use kite_connect::quotes::{Instrument, Ohlc};
use kite_connect::ws::{KiteTicker, Req, Ticker};
use kite_connect::{AutoAuth, KiteConnect};
use ratatui::crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::CrosstermBackend;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::{Frame, Terminal};
use std::collections::HashMap;
use std::env;
use std::time::Duration;

enum Screen {
    WatchList,
    Search,
}

struct App {
    screen: Screen,
    should_quit: bool,
    all_instruments: Vec<Instrument>,
    watch_instruments: HashMap<u32, WatchInstrument>,
    search_input: String,
    search_results: Vec<Instrument>,
    search_cursor_position: usize,
    kt: KiteTicker,
}

impl App {
    fn update_search_results(&mut self) {
        if self.search_input.is_empty() {
            self.search_results.clear();
            return;
        }

        let query = self.search_input.to_uppercase();
        self.search_results = self
            .all_instruments
            .iter()
            .filter(|&instrument| instrument.name.starts_with(query.as_str()))
            .take(5)
            .cloned()
            .collect();

        self.search_cursor_position = 0;
    }
}

pub struct WatchInstrument {
    trading_symbol: String,
    ltp: f64,
    ohlc: Ohlc,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("API_KEY").expect("Please pass the API_KEY as env variable.");
    let api_secret = env::var("API_SECRET").expect("Please pass the API_SECRET as env variable.");
    let access_token = env::var("ACCESS_TOKEN");

    let kc = if let Ok(access_token) = access_token {
        let kc = KiteConnect::new(api_key, api_secret);
        kc.authenticate_with_access_token(access_token).unwrap()
    } else {
        let auto_auth = AutoAuth::new(api_key, api_secret);
        let kc = auto_auth.authenticate().await.unwrap();

        let access_token = kc.access_token();
        println!("Access Token: {access_token}");
        println!("🤫 Keep it safe. Waiting 5 seconds for you to save it.");

        std::thread::sleep(std::time::Duration::from_secs(5));

        kc
    };

    let all_instruments = kc
        .get_exhchange_instruments(kite_connect::orders::Exchange::NSE)
        .await?;

    let (kt, rx) = kc.web_socket().await?;

    let mut app = App {
        screen: Screen::Search,
        should_quit: false,
        all_instruments,
        watch_instruments: HashMap::new(),
        search_input: String::new(),
        search_results: Vec::with_capacity(5),
        search_cursor_position: 0,
        kt,
    };

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    while !app.should_quit {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(250))?
            && let Event::Key(key) = event::read()?
        {
            match app.screen {
                Screen::WatchList => handle_watchlist_input(&mut app, key.code),
                Screen::Search => handle_search_input(&mut app, key.code).await?,
            }
        }

        while let Ok(ticker) = rx.try_recv() {
            match ticker {
                Ticker::FullQuote(full_quote) => {
                    if let Some(instrument) = app
                        .watch_instruments
                        .get_mut(&full_quote.quote.instrument_token)
                    {
                        instrument.ltp = full_quote.quote.last_price;
                        instrument.ohlc = full_quote.quote.ohlc;
                    }
                }
                Ticker::PartialQuote(partial_quote) => {
                    if let Some(instrument) = app
                        .watch_instruments
                        .get_mut(&partial_quote.instrument_token)
                    {
                        instrument.ltp = partial_quote.last_price;
                        instrument.ohlc = partial_quote.ohlc;
                    }
                }
                Ticker::IndicesQuote(indices_quote) => {
                    if let Some(instrument) = app
                        .watch_instruments
                        .get_mut(&indices_quote.instrument_token)
                    {
                        instrument.ltp = indices_quote.last_price;
                        instrument.ohlc = indices_quote.ohlc;
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    match app.screen {
        Screen::Search => draw_search_ui(f, app),
        Screen::WatchList => draw_watchlist_ui(f, app),
    }
}

fn draw_watchlist_ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.area());

    // --- Watchlist Items ---
    let watchlist_items: Vec<_> = app
        .watch_instruments
        .iter()
        .map(|(instrument_token, instrument)| {
            let change = instrument.ltp - instrument.ohlc.close;
            let change_percentage = change * 100.0 / instrument.ohlc.close;

            let color = if change >= 0.0 {
                Color::Green
            } else {
                Color::Red
            };

            let sign = if change >= 0.0 { "▲" } else { "▼" };

            let content = Line::from(vec![
                Span::styled(
                    instrument.trading_symbol.as_str(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" ({instrument_token})"),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::from(" "),
                Span::styled(
                    format!("{:.2}", instrument.ltp),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
                Span::from(" "),
                Span::styled(
                    format!(
                        "{sign} {:.2} ({:.2}%)",
                        change.abs(),
                        change_percentage.abs()
                    ),
                    Style::default().fg(color),
                ),
            ]);

            // Block::from(ListItem::new(content).style(Style::default().fg(Color::White)))
            //     .borders(Borders::ALL)
            //     .title(instrument.trading_symbol.as_str())
            ListItem::new(content).style(Style::default().fg(Color::White))
        })
        .collect();

    let watchlist_list = List::new(watchlist_items)
        .block(Block::default().borders(Borders::NONE).title("Watchlist"));

    f.render_widget(watchlist_list, chunks[0]);

    // --- Footer ---
    let footer_text = "Press 'q' to quit, '/' to search and add instrument.";
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[1]);
}

fn draw_search_ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.area());

    // --- Search Input Box ---
    let input = Paragraph::new(app.search_input.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Search"));
    f.render_widget(input, chunks[0]);

    // Make cursor visible
    f.set_cursor_position((
        chunks[0].x + app.search_input.len() as u16 + 1,
        chunks[0].y + 1,
    ));

    // --- Search Results List ---
    let results: Vec<_> = app
        .search_results
        .iter()
        .map(|i| ListItem::new(format!("{} ({})", i.name, i.trading_symbol)))
        .collect();

    let result_list = List::new(results)
        .block(Block::default().borders(Borders::ALL).title("Results"))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Gray),
        )
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(app.search_cursor_position));

    f.render_stateful_widget(result_list, chunks[1], &mut list_state);

    // --- Footer ---
    let footer_text = "Use ↑/↓ to navigate, [Enter] to add, [Esc] to go back.";
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn handle_watchlist_input(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('/') => app.screen = Screen::Search,
        _ => {}
    }
}

async fn handle_search_input(
    app: &mut App,
    key_code: KeyCode,
) -> Result<(), Box<dyn std::error::Error>> {
    match key_code {
        KeyCode::Char(c) => {
            app.search_input.push(c);
            app.update_search_results();
        }
        KeyCode::Backspace => {
            app.search_input.pop();
            app.update_search_results();
        }
        KeyCode::Up => {
            if !app.search_results.is_empty() {
                app.search_cursor_position = app.search_cursor_position.saturating_sub(1);
            }
        }
        KeyCode::Down => {
            if !app.search_results.is_empty() {
                let max_pos = app.search_results.len() - 1;
                if app.search_cursor_position < max_pos {
                    app.search_cursor_position += 1;
                }
            }
        }
        KeyCode::Enter => {
            if let Some(selected) = app.search_results.get(app.search_cursor_position) {
                app.watch_instruments.insert(
                    selected.instrument_token,
                    WatchInstrument {
                        trading_symbol: selected.trading_symbol.clone(),
                        ltp: 0.0,
                        ohlc: Ohlc {
                            open: 0.0,
                            high: 0.0,
                            low: 0.0,
                            close: 0.0,
                        },
                    },
                );

                app.kt
                    .send(Req::Subscribe(&[selected.instrument_token]))
                    .await?;

                // Reset search and go back to watchlist
                app.search_input.clear();
                app.search_results.clear();
                app.screen = Screen::WatchList;
            }
        }
        KeyCode::Esc => {
            app.search_input.clear();
            app.search_results.clear();
            app.screen = Screen::WatchList;
        }
        _ => {}
    }

    Ok(())
}

use crate::quotes::{Depth, DepthBook, LtpQuote, Ohlc, OhlcQuote};
use byteorder::{BigEndian, ReadBytesExt};
use crossbeam_channel::{Receiver, Sender};
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Seek, SeekFrom};
use tokio::{net::TcpStream, task::JoinHandle};
use tokio_tungstenite::tungstenite::{Bytes, Message};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

use super::*;

pub const KITE_WEB_SOCKET_ENDPOINT: &str = "wss://ws.kite.trade/";

pub struct KiteTicker {
    handle: JoinHandle<()>,
    write_stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Ticker {
    ConnectionClosed,
    IndicesQuote(OhlcQuote),
    LtpQuote(LtpQuote),
    PartialQuote(PartialQuote),
    FullQuote(FullQuote),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PartialQuote {
    pub instrument_token: u32,
    pub last_price: f64,
    pub last_traded_quantity: u32,
    pub average_traded_price: f64,
    pub volume_traded: u32,
    pub total_buy_quantity: u32,
    pub total_sell_quantity: u32,
    pub ohlc: Ohlc,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FullQuote {
    pub quote: PartialQuote,
    pub last_trade_time: u32,
    pub oi: u32,
    pub oi_day_high: u32,
    pub oi_day_low: u32,
    pub exchange_timestamp: u32,
    pub depth: DepthBook,
}

pub enum Req<'a> {
    Subscribe(&'a [u32]),
    Unsubscribe(&'a [u32]),
    Mode {
        mode: ReqMode,
        instrument_tokens: &'a [u32],
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ReqMode {
    Ltp,
    Quote,
    Full,
}

impl KiteTicker {
    pub async fn send(&mut self, req: Req<'_>) -> Result<(), Error> {
        let msg = match req {
            Req::Subscribe(instrument_tokens) => Message::Text(
                serde_json::json!({
                    "a": "subscribe",
                    "v": instrument_tokens
                })
                .to_string()
                .into(),
            ),
            Req::Unsubscribe(instrument_token) => Message::Text(
                serde_json::json!({
                    "a": "unsubscribe",
                    "v": instrument_token
                })
                .to_string()
                .into(),
            ),
            Req::Mode {
                mode,
                instrument_tokens,
            } => Message::Text(
                serde_json::json!({
                    "a": "mode",
                    "v": [mode, instrument_tokens]
                })
                .to_string()
                .into(),
            ),
        };

        self.send_raw(msg).await
    }

    pub async fn send_raw(&mut self, req: Message) -> Result<(), Error> {
        self.write_stream.send(req).await?;
        Ok(())
    }

    pub async fn wait_handle(self) {
        let _ = self.handle.await;
    }
}

impl KiteConnect<Authenticated> {
    pub async fn web_socket(&self) -> Result<(KiteTicker, Receiver<Ticker>), Error> {
        let endpoint = format!(
            "{KITE_WEB_SOCKET_ENDPOINT}?api_key={}&access_token={}",
            self.api_key(),
            self.access_token()
        );

        let (socket, _) = connect_async(endpoint).await?;
        let (write, read) = socket.split();

        let (tx, rx) = crossbeam_channel::unbounded();

        let handle = tokio::spawn(async move { handle_read_stream(read, tx).await });

        Ok((
            KiteTicker {
                handle,
                write_stream: write,
            },
            rx,
        ))
    }
}

async fn handle_read_stream(
    mut read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    tx: Sender<Ticker>,
) {
    use tokio_tungstenite::tungstenite::Error;

    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) => match msg {
                Message::Binary(bytes) => decode_n_send_bytes(bytes, &tx),
                Message::Text(bytes) => println!("{bytes}"),
                Message::Ping(_) | Message::Pong(_) => { /* TODO: Verify if we need to send Ping-Pong manually */
                }
                Message::Close(_) => {
                    if let Err(e) = tx.send(Ticker::ConnectionClosed) {
                        eprintln!(
                            "Trying to send \"Connection Closed\" message to already closed channel: {e}"
                        )
                    }
                }
                _ => unreachable!(),
            },
            Err(err) => match err {
                Error::AlreadyClosed | Error::ConnectionClosed => {
                    if let Err(e) = tx.send(Ticker::ConnectionClosed) {
                        eprintln!(
                            "Trying to send \"Connection Closed\" message to already closed channel: {e}"
                        )
                    }
                    break;
                }
                _ => eprintln!("Error while sending message to channel: {err}"),
            },
        }
    }
}

// TODO: Support parallel decoding for multiple packets
fn decode_n_send_bytes(bytes: Bytes, tx: &Sender<Ticker>) {
    if bytes.len() < 2 {
        return;
    }

    println!("Packet Length: {}", bytes.len());

    let mut cursor = Cursor::new(bytes);

    // TODO: Should we unwrap this?
    let total_packets = cursor.read_u16::<BigEndian>().unwrap();

    for _ in 0..total_packets {
        let packet_len = cursor.read_u16::<BigEndian>().unwrap();

        match packet_len {
            8 => send_ltp_quote_packet(&mut cursor, tx),
            28 | 32 => send_indices_quote_packet(&mut cursor, packet_len, tx),
            44 | 184 => send_quote_n_full_packet(&mut cursor, packet_len, tx),
            _ => {
                eprintln!("Got unsupported packet length {packet_len}. Skipping this packet");
                cursor.seek(SeekFrom::Current(packet_len as i64)).unwrap();
            }
        }
    }
}

// Refer: https://github.com/zerodha/pykiteconnect/blob/6b7b7621e575411921b506203b526bf275a702c7/kiteconnect/ticker.py#L740
fn send_ltp_quote_packet(cursor: &mut Cursor<Bytes>, tx: &Sender<Ticker>) {
    let instrument_token = cursor.read_u32::<BigEndian>().unwrap();
    let last_price = cursor.read_u32::<BigEndian>().unwrap();

    let divisor = get_divisor(instrument_token);
    let p = Ticker::LtpQuote(LtpQuote {
        instrument_token,
        last_price: last_price as f64 / divisor,
    });

    if let Err(err) = tx.send(p) {
        eprintln!("Trying to send LTP Packet to channel which is closed: {err}")
    }
}

// Refer: https://kite.trade/docs/connect/v3/websocket/#index-packet-structure
fn send_indices_quote_packet(cursor: &mut Cursor<Bytes>, packet_len: u16, tx: &Sender<Ticker>) {
    let instrument_token = cursor.read_u32::<BigEndian>().unwrap();
    let last_price = cursor.read_u32::<BigEndian>().unwrap();
    let high_of_day = cursor.read_u32::<BigEndian>().unwrap();
    let low_of_day = cursor.read_u32::<BigEndian>().unwrap();
    let open_of_day = cursor.read_u32::<BigEndian>().unwrap();
    let close_of_day = cursor.read_u32::<BigEndian>().unwrap();

    if packet_len == 32 {
        // TODO: Should we include exchange timestamp for incides quotes or not?
        // 4 (price_change) + 4 (exchange_timestamp) = 8 bytes to be skipped
        cursor.seek(SeekFrom::Current(8)).unwrap();
    } else {
        // Skip price change as it can be calculated later using ohlc and last_price
        cursor.seek(SeekFrom::Current(4)).unwrap();
    }

    let divisor = get_divisor(instrument_token);

    let p = Ticker::IndicesQuote(OhlcQuote {
        instrument_token,
        last_price: last_price as f64 / divisor,
        ohlc: Ohlc {
            open: open_of_day as f64 / divisor,
            high: high_of_day as f64 / divisor,
            low: low_of_day as f64 / divisor,
            close: close_of_day as f64 / divisor,
        },
    });

    if let Err(err) = tx.send(p) {
        eprintln!("Trying to send Quote Packet to channel which is closed: {err}")
    }
}

// Refer: https://github.com/zerodha/pykiteconnect/blob/6b7b7621e575411921b506203b526bf275a702c7/kiteconnect/ticker.py#L780
fn send_quote_n_full_packet(cursor: &mut Cursor<Bytes>, packet_len: u16, tx: &Sender<Ticker>) {
    let instrument_token = cursor.read_u32::<BigEndian>().unwrap();

    let divisor = get_divisor(instrument_token);

    let last_price = cursor.read_u32::<BigEndian>().unwrap() as f64 / divisor;
    let last_traded_quantity = cursor.read_u32::<BigEndian>().unwrap();
    let average_price = cursor.read_u32::<BigEndian>().unwrap() as f64 / divisor;
    let volume_traded = cursor.read_u32::<BigEndian>().unwrap();
    let total_buy_quantity = cursor.read_u32::<BigEndian>().unwrap();
    let total_sell_quantity = cursor.read_u32::<BigEndian>().unwrap();
    let open = cursor.read_u32::<BigEndian>().unwrap() as f64 / divisor;
    let high = cursor.read_u32::<BigEndian>().unwrap() as f64 / divisor;
    let low = cursor.read_u32::<BigEndian>().unwrap() as f64 / divisor;
    let close = cursor.read_u32::<BigEndian>().unwrap() as f64 / divisor;

    let quote = PartialQuote {
        instrument_token,
        last_price,
        last_traded_quantity,
        average_traded_price: average_price,
        volume_traded,
        total_buy_quantity,
        total_sell_quantity,
        ohlc: Ohlc {
            open,
            high,
            low,
            close,
        },
    };

    if packet_len == 184 {
        let last_trade_time = cursor.read_u32::<BigEndian>().unwrap();
        let oi = cursor.read_u32::<BigEndian>().unwrap();
        let oi_day_high = cursor.read_u32::<BigEndian>().unwrap();
        let oi_day_low = cursor.read_u32::<BigEndian>().unwrap();
        let exchange_timestamp = cursor.read_u32::<BigEndian>().unwrap();

        let mut depth = DepthBook::with_capacity(5);
        for i in 0..10 {
            if let (Ok(qty), Ok(price_raw), Ok(orders)) = (
                cursor.read_u32::<BigEndian>(),
                cursor.read_u32::<BigEndian>(),
                cursor.read_u16::<BigEndian>(),
            ) {
                // Skip the 2-byte padding after reading orders
                cursor.seek(SeekFrom::Current(2)).unwrap_or_default();

                let entry = Depth {
                    quantity: qty as i64,
                    price: price_raw as f64 / divisor,
                    orders: orders as i64,
                };
                if i < 5 {
                    depth.buy.push(entry);
                } else {
                    depth.sell.push(entry);
                }
            }
        }

        let full_quote = FullQuote {
            quote,
            oi,
            oi_day_high,
            oi_day_low,
            depth,
            exchange_timestamp,
            last_trade_time,
        };

        if let Err(err) = tx.send(Ticker::FullQuote(full_quote)) {
            eprintln!("Failed to send Full Quote Packet to channel which is already closed: {err}");
        }
    } else {
        if let Err(err) = tx.send(Ticker::PartialQuote(quote)) {
            eprintln!(
                "Failed to send Partial Quote Packet to channel which is already closed: {err}"
            );
        }
    }
}

#[inline]
const fn get_divisor(instrument_token: u32) -> f64 {
    const CDS_SEGMENT: u32 = 3;
    const BCD_SEGMENT: u32 = 6;

    let segment = instrument_token & 0xff;

    match segment {
        CDS_SEGMENT => 10_000_000.0,
        BCD_SEGMENT => 10_000.0,
        _ => 100.0,
    }
}

use crate::orders::Exchange;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::*;

/// API endpoint for retrieving all instruments.
pub const GET_INSTRUMENTS_ENDPOINT: &str = "https://api.kite.trade/instruments";
/// API endpoint for retrieving full market quotes.
pub const GET_FULL_MARKET_QUOTES: &str = "https://api.kite.trade/quote";
/// API endpoint for retrieving OHLC quotes.
pub const GET_OHLC_QUOTES: &str = "https://api.kite.trade/quote/ohlc";
/// API endpoint for retrieving LTP (Last Traded Price) quotes.
pub const GET_LTP_QUOTES: &str = "https://api.kite.trade/quote/ltp";

/// Represents a financial instrument (stock, futures, options, etc.) available for trading.
///
/// Instruments are identified by their trading symbol and exchange, and have various properties
/// like lot size, tick size, expiry (for derivatives), etc.
///
/// Refer to the [official documentation](https://kite.trade/docs/connect/v3/market-quotes/#instruments) for details.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Instrument {
    /// Numerical identifier issued by the exchange representing the instrument
    pub instrument_token: u32,
    /// Exchange-specific token identifier
    pub exchange_token: String,
    /// Trading symbol of the instrument
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    /// Full name of the instrument
    pub name: String,
    /// Last traded price of the instrument
    pub last_price: f64,
    /// Expiry date for derivatives (format: YYYY-MM-DD)
    pub expiry: String,
    /// Strike price for options (0 for non-options)
    pub strike: f64,
    /// Minimum price movement allowed for the instrument
    pub tick_size: f64,
    /// Lot size (number of units in one lot)
    pub lot_size: i64,
    /// Type of instrument (EQ, FUT, CE, PE)
    pub instrument_type: InstrumentType,
    /// Trading segment
    pub segment: String,
    /// Exchange name
    pub exchange: String,
}

/// Type of financial instrument.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum InstrumentType {
    /// Equity - Stocks and shares
    EQ,
    /// Futures - Futures contracts
    FUT,
    /// Call Option - Call options
    CE,
    /// Put Option - Put options
    PE,
}

/// Full market quote containing comprehensive market data for an instrument.
///
/// This includes price data, volume, open interest, market depth, and more.
/// Refer to the [official documentation](https://kite.trade/docs/connect/v3/market-quotes/#full-market-quote) for details.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Quote {
    /// The numerical identifier issued by the exchange representing the instrument.
    pub instrument_token: u32,
    /// The exchange timestamp of the quote packet
    pub timestamp: String,
    /// Last trade timestamp
    pub last_trade_time: Option<String>,
    /// Last traded market price
    pub last_price: f64,
    /// Volume traded today
    pub volume: i64,
    /// The volume weighted average price of a stock at a given time during the day. [Learn More](https://support.zerodha.com/category/trading-and-markets/general-kite/kite-mw/articles/what-does-the-average-price-on-kite-3-market-depth-mean)
    pub average_price: f64,
    /// Total quantity of buy orders pending at the exchange
    pub buy_quantity: i64,
    /// Total quantity of sell orders pending at the exchange
    pub sell_quantity: i64,
    /// Total number of outstanding contracts held by market participants exchange-wide (only F&O)
    #[serde(default)]
    pub open_interest: Option<f64>,
    /// Last traded quantity
    pub last_quantity: i64,
    /// OHLC (Open, High, Low, Close) price data
    pub ohlc: Ohlc,
    /// The absolute change from yesterday's close to last traded price
    pub net_change: f64,
    /// The current lower circuit limit
    pub lower_circuit_limit: f64,
    /// The current upper circuit limit
    pub upper_circuit_limit: f64,
    /// The Open Interest for a futures or options contract. [Learn More](https://zerodha.com/varsity/chapter/open-interest/)
    pub oi: f64,
    /// The highest Open Interest recorded during the day
    pub oi_day_high: f64,
    /// The lowest Open Interest recorded during the day
    pub oi_day_low: f64,
    /// Market depth (order book) showing buy and sell orders at different price levels
    pub depth: DepthBook,
}

/// OHLC (Open, High, Low, Close) quote with last traded price.
///
/// This is a lightweight quote structure containing only price data.
/// Refer to the [official documentation](https://kite.trade/docs/connect/v3/market-quotes/#ohlc-quote) for details.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct OhlcQuote {
    /// The numerical identifier issued by the exchange representing the instrument.
    pub instrument_token: u32,
    /// Last traded market price
    pub last_price: f64,
    /// OHLC (Open, High, Low, Close) price data
    pub ohlc: Ohlc,
}

/// Last Traded Price (LTP) quote - the most basic quote containing only the last traded price.
///
/// This is the lightest quote format, useful when you only need the current price.
/// Refer to the [official documentation](https://kite.trade/docs/connect/v3/market-quotes/#ltp-quote) for details.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct LtpQuote {
    /// The numerical identifier issued by the exchange representing the instrument.
    pub instrument_token: u32,
    /// Last traded market price
    pub last_price: f64,
}

/// OHLC (Open, High, Low, Close) price data for a trading period.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Ohlc {
    /// Price at market opening
    pub open: f64,
    /// Highest price today
    pub high: f64,
    /// Lowest price today
    pub low: f64,
    /// Closing price of the instrument from the last trading day
    pub close: f64,
}

/// Market depth book containing buy and sell orders at different price levels.
///
/// The depth book shows pending orders in the order book, typically showing the top 5 levels
/// on both buy and sell sides.
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DepthBook {
    /// Buy side depth (orders to buy)
    pub buy: Vec<Depth>,
    /// Sell side depth (orders to sell)
    pub sell: Vec<Depth>,
}

impl DepthBook {
    /// Creates a new empty `DepthBook`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `DepthBook` with the specified capacity for buy and sell depth vectors.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Initial capacity for both buy and sell depth vectors
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buy: Vec::with_capacity(capacity),
            sell: Vec::with_capacity(capacity),
        }
    }
}

/// Market depth entry representing orders at a specific price level.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Depth {
    /// Price at which the depth stands
    pub price: f64,
    /// Number of open orders at the price
    pub orders: i64,
    /// Net quantity from the pending orders
    pub quantity: i64,
}

impl KiteConnect<Authenticated> {
    /// Retrieves all instruments available for trading across all exchanges.
    ///
    /// This method downloads the complete instrument master file which can be large.
    /// The response is in CSV format and is parsed into a vector of `Instrument` structs.
    ///
    /// Refer to the [official documentation](https://kite.trade/docs/connect/v3/market-quotes/#instruments) for details.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Instrument>)` containing all available instruments
    /// * `Err(Error)` if the request failed
    ///
    /// # Performance
    ///
    /// This method has a 30-minute timeout due to the large file size.
    /// Consider caching the results locally for better performance.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use kite_connect::KiteConnect;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let kite: KiteConnect<kite_connect::Authenticated> = todo!();
    /// let instruments = kite.get_all_instruments().await?;
    /// println!("Total instruments: {}", instruments.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_all_instruments(&self) -> Result<Vec<Instrument>, Error> {
        let bytes = self
            .client
            .get(GET_INSTRUMENTS_ENDPOINT)
            // This is a large file, give it some extra time of 30 minutes
            .timeout(std::time::Duration::from_secs(1800))
            .send()
            .await?
            .bytes()
            .await?;

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(bytes.as_ref());

        let mut instruments = Vec::new();
        for result in rdr.deserialize() {
            let instrument: Instrument = result?;
            instruments.push(instrument);
        }

        Ok(instruments)
    }

    /// Retrieves all instruments for a specific exchange.
    ///
    /// This method downloads the instrument master file for the specified exchange.
    /// The response is in CSV format and is parsed into a vector of `Instrument` structs.
    ///
    /// Refer to the [official documentation](https://kite.trade/docs/connect/v3/market-quotes/#instruments) for details.
    ///
    /// # Arguments
    ///
    /// * `exchange` - The exchange for which to retrieve instruments
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Instrument>)` containing all instruments for the specified exchange
    /// * `Err(Error)` if the request failed
    ///
    /// # Performance
    ///
    /// This method has a 30-minute timeout due to the potentially large file size.
    /// Consider caching the results locally for better performance.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use kite_connect::{KiteConnect, orders::Exchange};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let kite: KiteConnect<kite_connect::Authenticated> = todo!();
    /// let nse_instruments = kite.get_exhchange_instruments(Exchange::NSE).await?;
    /// println!("NSE instruments: {}", nse_instruments.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_exhchange_instruments(
        &self,
        exchange: Exchange,
    ) -> Result<Vec<Instrument>, Error> {
        let bytes = self
            .client
            .get(format!("{GET_INSTRUMENTS_ENDPOINT}/{exchange}"))
            // This is a large file, give it some extra time of 30 minutes
            .timeout(std::time::Duration::from_secs(1800))
            .send()
            .await?
            .bytes()
            .await?;

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(bytes.as_ref());

        let mut instruments = Vec::new();
        for result in rdr.deserialize() {
            let instrument: Instrument = result?;
            instruments.push(instrument);
        }

        Ok(instruments)
    }

    /// Retrieves full market quotes for the specified instruments.
    ///
    /// This method returns comprehensive market data including price, volume, open interest,
    /// market depth, and more for each instrument.
    ///
    /// Refer to the [official documentation](https://kite.trade/docs/connect/v3/market-quotes/#full-market-quote) for details.
    ///
    /// # Arguments
    ///
    /// * `i` - A slice of instrument identifiers. Each identifier can be:
    ///   - An `instrument_token` (u32)
    ///   - A string in the format "EXCHANGE:TRADINGSYMBOL" (e.g., "NSE:INFY")
    ///
    /// # Returns
    ///
    /// * `Ok(HashMap<String, Quote>)` - A map where keys are instrument identifiers and values are full quotes
    /// * `Err(Error)` if the request failed
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use kite_connect::KiteConnect;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let kite: KiteConnect<kite_connect::Authenticated> = todo!();
    /// let instruments = ["NSE:INFY", "NSE:RELIANCE"];
    /// let quotes = kite.get_market_quotes(&instruments).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_market_quotes<I: Serialize + Copy>(
        &self,
        i: &[I],
    ) -> Result<HashMap<String, Quote>, Error> {
        self.get_quotes_impl(i, GET_FULL_MARKET_QUOTES).await
    }

    /// Retrieves OHLC quotes for the specified instruments.
    ///
    /// This method returns lightweight quotes containing only OHLC (Open, High, Low, Close) data
    /// and the last traded price.
    ///
    /// Refer to the [official documentation](https://kite.trade/docs/connect/v3/market-quotes/#ohlc-quote) for details.
    ///
    /// # Arguments
    ///
    /// * `i` - A slice of instrument identifiers. Each identifier can be:
    ///   - An `instrument_token` (u32)
    ///   - A string in the format "EXCHANGE:TRADINGSYMBOL" (e.g., "NSE:INFY")
    ///
    /// # Returns
    ///
    /// * `Ok(HashMap<String, OhlcQuote>)` - A map where keys are instrument identifiers and values are OHLC quotes
    /// * `Err(Error)` if the request failed
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use kite_connect::KiteConnect;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let kite: KiteConnect<kite_connect::Authenticated> = todo!();
    /// let instruments = ["NSE:INFY"];
    /// let quotes = kite.get_ohlc_quotes(&instruments).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_ohlc_quotes<I: Serialize + Copy>(
        &self,
        i: &[I],
    ) -> Result<HashMap<String, OhlcQuote>, Error> {
        self.get_quotes_impl(i, GET_OHLC_QUOTES).await
    }

    /// Retrieves Last Traded Price (LTP) quotes for the specified instruments.
    ///
    /// This method returns the most basic quotes containing only the last traded price.
    /// This is the lightest quote format and is useful when you only need current prices.
    ///
    /// Refer to the [official documentation](https://kite.trade/docs/connect/v3/market-quotes/#ltp-quote) for details.
    ///
    /// # Arguments
    ///
    /// * `i` - A slice of instrument identifiers. Each identifier can be:
    ///   - An `instrument_token` (u32)
    ///   - A string in the format "EXCHANGE:TRADINGSYMBOL" (e.g., "NSE:INFY")
    ///
    /// # Returns
    ///
    /// * `Ok(LtpQuote)` - The LTP quote (Note: The return type appears incorrect; should be `HashMap<String, LtpQuote>`)
    /// * `Err(Error)` if the request failed
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use kite_connect::KiteConnect;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let kite: KiteConnect<kite_connect::Authenticated> = todo!();
    /// let instruments = ["NSE:INFY"];
    /// let quotes = kite.get_ltp_quotes(&instruments).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_ltp_quotes<I: Serialize + Copy>(&self, i: &[I]) -> Result<LtpQuote, Error> {
        self.get_quotes_impl(i, GET_LTP_QUOTES).await
    }

    async fn get_quotes_impl<I, Q>(&self, i: &[I], endpoint: &'static str) -> Result<Q, Error>
    where
        I: Serialize + Copy,
        Q: for<'de> serde::de::Deserialize<'de>,
    {
        // TODO: Is this a good to be done in this function?
        let q: Vec<_> = i.iter().map(|&i| ("i", i)).collect();

        Ok(self
            .client
            .get(endpoint)
            .query(&q)
            .send()
            .await?
            .json::<Response<Q>>()
            .await?
            .into_result()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_quote() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "status": "success",
            "data": {
              "NSE:INFY": {
                "instrument_token": 408065,
                "timestamp": "2021-06-08 15:45:56",
                "last_trade_time": "2021-06-08 15:45:52",
                "last_price": 1412.95,
                "last_quantity": 5,
                "buy_quantity": 0,
                "sell_quantity": 5191,
                "volume": 7360198,
                "average_price": 1412.47,
                "oi": 0,
                "oi_day_high": 0,
                "oi_day_low": 0,
                "net_change": 0,
                "lower_circuit_limit": 1250.7,
                "upper_circuit_limit": 1528.6,
                "ohlc": {
                  "open": 1396,
                  "high": 1421.75,
                  "low": 1395.55,
                  "close": 1389.65
                },
                "depth": {
                  "buy": [
                    {
                      "price": 0,
                      "quantity": 0,
                      "orders": 0
                    },
                    {
                      "price": 0,
                      "quantity": 0,
                      "orders": 0
                    },
                    {
                      "price": 0,
                      "quantity": 0,
                      "orders": 0
                    },
                    {
                      "price": 0,
                      "quantity": 0,
                      "orders": 0
                    },
                    {
                      "price": 0,
                      "quantity": 0,
                      "orders": 0
                    }
                  ],
                  "sell": [
                    {
                      "price": 1412.95,
                      "quantity": 5191,
                      "orders": 13
                    },
                    {
                      "price": 0,
                      "quantity": 0,
                      "orders": 0
                    },
                    {
                      "price": 0,
                      "quantity": 0,
                      "orders": 0
                    },
                    {
                      "price": 0,
                      "quantity": 0,
                      "orders": 0
                    },
                    {
                      "price": 0,
                      "quantity": 0,
                      "orders": 0
                    }
                  ]
                }
              }
            }
          }
          "#;

        let value: Response<_> = serde_json::from_str(json)?;

        let mut sell_depth = vec![Depth {
            price: 1412.95,
            quantity: 5191,
            orders: 13,
        }];
        sell_depth.append(&mut vec![
            Depth {
                price: 0.0,
                quantity: 0,
                orders: 0,
            };
            4
        ]);

        let mut map = HashMap::new();
        map.insert(
            String::from("NSE:INFY"),
            Quote {
                instrument_token: 408065,
                timestamp: "2021-06-08 15:45:56".into(),
                last_trade_time: Some("2021-06-08 15:45:52".into()),
                last_price: 1412.95,
                last_quantity: 5,
                buy_quantity: 0,
                sell_quantity: 5191,
                volume: 7360198,
                average_price: 1412.47,
                oi: 0.0,
                oi_day_high: 0.0,
                oi_day_low: 0.0,
                net_change: 0.0,
                lower_circuit_limit: 1250.7,
                upper_circuit_limit: 1528.6,
                ohlc: Ohlc {
                    open: 1396.0,
                    high: 1421.75,
                    low: 1395.55,
                    close: 1389.65,
                },
                depth: DepthBook {
                    buy: vec![
                        Depth {
                            price: 0.0,
                            quantity: 0,
                            orders: 0
                        };
                        5
                    ],
                    sell: sell_depth,
                },
                open_interest: None,
            },
        );

        assert_eq!(value, Response::Success { data: map });

        Ok(())
    }

    #[test]
    fn test_ohlc_quote() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "status": "success",
            "data": {
                "NSE:INFY": {
                    "instrument_token": 408065,
                    "last_price": 1075,
                    "ohlc": {
                        "open": 1085.8,
                        "high": 1085.9,
                        "low": 1070.9,
                        "close": 1075.8
                    }
                }
            }
        }"#;

        let value: Response<_> = serde_json::from_str(json)?;

        let mut map = HashMap::new();
        map.insert(
            String::from("NSE:INFY"),
            OhlcQuote {
                instrument_token: 408065,
                last_price: 1075.0,
                ohlc: Ohlc {
                    open: 1085.8,
                    high: 1085.9,
                    low: 1070.9,
                    close: 1075.8,
                },
            },
        );

        assert_eq!(value, Response::Success { data: map });

        Ok(())
    }

    #[test]
    fn test_ltp_quote() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "status": "success",
            "data": {
                "NSE:INFY": {
                    "instrument_token": 408065,
                    "last_price": 1074.35
                }
            }
        }"#;

        let value: Response<_> = serde_json::from_str(json)?;

        let mut map = HashMap::new();
        map.insert(
            String::from("NSE:INFY"),
            LtpQuote {
                instrument_token: 408065,
                last_price: 1074.35,
            },
        );

        assert_eq!(value, Response::Success { data: map });

        Ok(())
    }
}

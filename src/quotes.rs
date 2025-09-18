use crate::orders::Exchange;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::*;

pub const GET_INSTRUMENTS_ENDPOINT: &str = "https://api.kite.trade/instruments";
pub const GET_FULL_MARKET_QUOTES: &str = "https://api.kite.trade/quote";
pub const GET_OHLC_QUOTES: &str = "https://api.kite.trade/quote/ohlc";
pub const GET_LTP_QUOTES: &str = "https://api.kite.trade/quote/ltp";

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Instrument {
    pub instrument_token: String,
    pub exchange_token: String,
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    pub name: String,
    pub last_price: f64,
    pub expiry: String,
    pub strike: f64,
    pub tick_size: f64,
    pub lot_size: i64,
    pub instrument_type: InstrumentType,
    pub segment: String,
    pub exchange: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum InstrumentType {
    EQ,
    FUT,
    CE,
    PE,
}

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
    pub depth: DepthBook,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct OhlcQuote {
    /// The numerical identifier issued by the exchange representing the instrument.
    pub instrument_token: u32,
    /// Last traded market price
    pub last_price: f64,
    pub ohlc: Ohlc,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct LtpQuote {
    /// The numerical identifier issued by the exchange representing the instrument.
    pub instrument_token: u32,
    /// Last traded market price
    pub last_price: f64,
}

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

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DepthBook {
    pub buy: Vec<Depth>,
    pub sell: Vec<Depth>,
}

impl DepthBook {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buy: Vec::with_capacity(capacity),
            sell: Vec::with_capacity(capacity),
        }
    }
}

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
    // TODO: Optimize this function performance
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

    pub async fn get_market_quotes<I: Serialize + Copy>(
        &self,
        i: &[I],
    ) -> Result<HashMap<String, Quote>, Error> {
        self.get_quotes_impl(i, GET_FULL_MARKET_QUOTES).await
    }

    pub async fn get_ohlc_quotes<I: Serialize + Copy>(
        &self,
        i: &[I],
    ) -> Result<HashMap<String, OhlcQuote>, Error> {
        self.get_quotes_impl(i, GET_OHLC_QUOTES).await
    }

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

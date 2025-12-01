use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::*;

pub const GET_HISTORICAL_CANDLE_ENDPOINT: &str = "https://api.kite.trade/instruments/historical/";

/// The format string used for candle timestamps.
pub const CANDLE_TIMESTAMP_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%z";

/// Time interval for historical candle data.
///
/// Different intervals are available for different types of analysis.
/// Refer to the [official documentation](https://kite.trade/docs/connect/v3/historical/) for details.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Interval {
    Minute,
    Day,
    #[serde(rename = "3minute")]
    ThreeMinute,
    #[serde(rename = "5minute")]
    FiveMinute,
    #[serde(rename = "10minute")]
    TenMinute,
    #[serde(rename = "15minute")]
    FifteenMinute,
    #[serde(rename = "30minute")]
    ThirtyMinute,
    #[serde(rename = "60minute")]
    SixtyMinute,
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::Minute => write!(f, "minute"),
            Interval::Day => write!(f, "day"),
            Interval::ThreeMinute => write!(f, "3minute"),
            Interval::FiveMinute => write!(f, "5minute"),
            Interval::TenMinute => write!(f, "10minute"),
            Interval::FifteenMinute => write!(f, "15minute"),
            Interval::ThirtyMinute => write!(f, "30minute"),
            Interval::SixtyMinute => write!(f, "60minute"),
        }
    }
}

/// Request structure for fetching historical candle data.
///
/// Historical data is available for various time intervals and date ranges.
/// Refer to the [official documentation](https://kite.trade/docs/connect/v3/historical/) for details.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct HistoricalCandleReq {
    /// `yyyy-mm-dd hh:mm:ss` formatted date indicating the start date of records
    pub from: String,
    /// `yyyy-mm-dd hh:mm:ss` formatted date indicating the end date of records
    pub to: String,
    /// pass `true` to get continuous data
    pub continuous: bool,
    /// pass `true` to get OI data
    pub oi: bool,
}

/// Represents a single candle (OHLCV data point) in historical data.
///
/// A candle contains the open, high, low, close prices and volume for a specific time period.
/// For F&O instruments, it may also include open interest data.
#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct Candle {
    pub timestamp: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
    pub oi: Option<i64>,
}

impl<'de> Deserialize<'de> for Candle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let arr: Vec<serde_json::Value> = Vec::deserialize(deserializer)?;
        if arr.len() < 6 || arr.len() > 7 {
            return Err(serde::de::Error::custom(
                "Expected array of length either 6 or 7 for candle",
            ));
        }

        Ok(Candle {
            timestamp: arr[0]
                .as_str()
                .ok_or_else(|| serde::de::Error::custom("Invalid timestamp"))?
                .to_string(),
            open: arr[1]
                .as_f64()
                .ok_or_else(|| serde::de::Error::custom("Invalid open"))?,
            high: arr[2]
                .as_f64()
                .ok_or_else(|| serde::de::Error::custom("Invalid high"))?,
            low: arr[3]
                .as_f64()
                .ok_or_else(|| serde::de::Error::custom("Invalid low"))?,
            close: arr[4]
                .as_f64()
                .ok_or_else(|| serde::de::Error::custom("Invalid close"))?,
            volume: arr[5]
                .as_i64()
                .ok_or_else(|| serde::de::Error::custom("Invalid volume"))?,
            oi: if arr.len() == 7 {
                arr[6].as_i64()
            } else {
                None
            },
        })
    }
}

impl KiteConnect<Authenticated> {
    /// Retrieves historical candle data for an instrument.
    ///
    /// This method fetches OHLCV (Open, High, Low, Close, Volume) data for a specified
    /// instrument, time interval, and date range. For F&O instruments, open interest data
    /// is also included if requested.
    ///
    /// Refer to the [official documentation](https://kite.trade/docs/connect/v3/historical/) for details.
    ///
    /// # Arguments
    ///
    /// * `instrument_token` - The instrument token for which to fetch historical data
    /// * `interval` - The time interval for candles (minute, 3minute, 5minute, etc.)
    /// * `req` - The request containing date range and other parameters
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Candle>)` containing historical candle data
    /// * `Err(Error)` if the request failed
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use kite_connect::{KiteConnect, historical::*};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let kite: KiteConnect<kite_connect::Authenticated> = todo!();
    /// let req = HistoricalCandleReq {
    ///     from: "2023-01-01 09:15:00".to_string(),
    ///     to: "2023-01-01 15:30:00".to_string(),
    ///     continuous: false,
    ///     oi: false,
    /// };
    ///
    /// let candles = kite.get_historical_data(408065, Interval::Minute, req).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_historical_data(
        &self,
        instrument_token: u32,
        interval: Interval,
        req: HistoricalCandleReq,
    ) -> Result<Vec<Candle>, Error> {
        #[derive(Deserialize)]
        struct Candles {
            candles: Vec<Candle>,
        }

        let q = [
            ("from", req.from.as_str()),
            ("to", req.to.as_str()),
            ("continuous", bool_to_int_str_impl(req.continuous)),
            ("oi", bool_to_int_str_impl(req.oi)),
        ];

        Ok(self
            .client
            .get(format!(
                "{GET_HISTORICAL_CANDLE_ENDPOINT}{instrument_token}/{interval}"
            ))
            .query(&q)
            .send()
            .await?
            .json::<Response<Candles>>()
            .await?
            .into_result()?
            .candles)
    }
}

const fn bool_to_int_str_impl(b: bool) -> &'static str {
    if b { "1" } else { "0" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Candles {
        candles: Vec<Candle>,
    }

    #[test]
    fn test_candles() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
          "status": "success",
          "data": {
            "candles": [
              [
                "2019-12-04T09:15:00+0530",
                12009.9,
                12019.35,
                12001.25,
                12001.5,
                163275,
                13667775
              ],
              [
                "2019-12-04T09:16:00+0530",
                12001,
                12003,
                11998.25,
                12001,
                105750,
                13667775
              ]
            ]
          }
        }"#;

        let value: Response<_> = serde_json::from_str(json)?;

        let expected = Candles {
            candles: vec![
                Candle {
                    timestamp: "2019-12-04T09:15:00+0530".into(),
                    open: 12009.9,
                    high: 12019.35,
                    low: 12001.25,
                    close: 12001.5,
                    volume: 163275,
                    oi: Some(13667775),
                },
                Candle {
                    timestamp: "2019-12-04T09:16:00+0530".into(),
                    open: 12001.0,
                    high: 12003.0,
                    low: 11998.25,
                    close: 12001.0,
                    volume: 105750,
                    oi: Some(13667775),
                },
            ],
        };

        assert_eq!(value, Response::Success { data: expected });

        Ok(())
    }
}

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::orders::{Exchange, Product, TransactionType};

use super::*;

pub const GET_HOLDINGS_ENDPOINT: &str = "https://api.kite.trade/portfolio/holdings";
pub const GET_HOLDINGS_AUCTION_ENDPOINT: &str =
    "https://api.kite.trade/portfolio/holdings/auctions";
pub const GET_PUT_POSITIONS_ENDPOINT: &str = "https://api.kite.trade/portfolio/positions";

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Holding {
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    pub exchange: Exchange,
    pub instrument_token: u32,
    pub isin: String,
    pub t1_quantity: i64,
    pub realised_quantity: i64,
    pub quantity: i64,
    pub used_quantity: i64,
    pub authorised_quantity: i64,
    pub opening_quantity: i64,
    pub authorised_date: String,
    pub price: f64,
    pub average_price: f64,
    pub last_price: f64,
    pub close_price: f64,
    pub pnl: f64,
    pub day_change: f64,
    pub day_change_percentage: f64,
    pub product: Product,
    pub collateral_quantity: i64,
    pub collateral_type: Option<String>,
    pub discrepancy: bool,
    // Undocumented fields in Kite Documentation
    pub authorisation: Value,
    pub mtf: Value,
    pub short_quantity: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HoldingAuction {
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    pub exchange: Exchange,
    pub instrument_token: u32,
    pub isin: String,
    pub product: Product,
    pub price: f64,
    pub quantity: i64,
    pub t1_quantity: i64,
    pub realised_quantity: i64,
    pub authorised_quantity: i64,
    pub authorised_date: String,
    pub opening_quantity: i64,
    pub collateral_quantity: i64,
    pub collateral_type: Option<String>,
    pub discrepancy: bool,
    pub average_price: f64,
    pub last_price: f64,
    pub close_price: f64,
    pub pnl: f64,
    pub day_change: f64,
    pub day_change_percentage: f64,
    pub auction_number: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Position {
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    pub exchange: Exchange,
    pub instrument_token: u32,
    pub product: Product,
    pub quantity: i64,
    pub overnight_quantity: i64,
    pub multiplier: i64,
    pub average_price: f64,
    pub close_price: f64,
    pub last_price: f64,
    pub value: f64,
    pub pnl: f64,
    pub m2m: f64,
    pub unrealised: f64,
    pub realised: f64,
    pub buy_quantity: i64,
    pub buy_price: f64,
    pub buy_value: f64,
    pub buy_m2m: f64,
    pub day_buy_quantity: i64,
    pub day_buy_price: f64,
    pub day_buy_value: f64,
    pub sell_quantity: i64,
    pub sell_price: f64,
    pub sell_value: f64,
    pub sell_m2m: f64,
    pub day_sell_quantity: i64,
    pub day_sell_price: f64,
    pub day_sell_value: f64,
}

// TODO: Find a better name
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Positions {
    pub net: Vec<Position>,
    pub day: Vec<Position>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ConvertPositionReq {
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    pub exchange: Exchange,
    pub transaction_type: TransactionType,
    pub position_type: PositionType,
    pub quantity: i64,
    pub old_product: Product,
    pub new_product: Product,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum PositionType {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "overnight")]
    OverNight,
}

impl KiteConnect<Authenticated> {
    pub async fn get_holdings(&self) -> Result<Vec<Holding>, Error> {
        Ok(self
            .client
            .get(GET_HOLDINGS_ENDPOINT)
            .send()
            .await?
            .json::<Response<_>>()
            .await?
            .into_result()?)
    }

    pub async fn get_holdings_auction(&self) -> Result<Vec<HoldingAuction>, Error> {
        Ok(self
            .client
            .get(GET_HOLDINGS_AUCTION_ENDPOINT)
            .send()
            .await?
            .json::<Response<_>>()
            .await?
            .into_result()?)
    }

    pub async fn get_positions(&self) -> Result<Positions, Error> {
        Ok(self
            .client
            .get(GET_PUT_POSITIONS_ENDPOINT)
            .send()
            .await?
            .json::<Response<_>>()
            .await?
            .into_result()?)
    }

    pub async fn convert_position(&self, req: &ConvertPositionReq) -> Result<bool, Error> {
        Ok(self
            .client
            .put(GET_PUT_POSITIONS_ENDPOINT)
            .form(req)
            .send()
            .await?
            .json::<Response<bool>>()
            .await?
            .into_result()?)
    }

    /// Unimplemented
    ///
    /// Refer <https://kite.trade/docs/connect/v3/portfolio/#holdings-authorisation>
    pub async fn authorise_holdings(&self) -> Result<(), Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holdings() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
          "status": "success",
          "data": [
            {
              "tradingsymbol": "AARON",
              "exchange": "NSE",
              "instrument_token": 263681,
              "isin": "INE721Z01010",
              "product": "CNC",
              "price": 0,
              "quantity": 1,
              "used_quantity": 0,
              "t1_quantity": 0,
              "realised_quantity": 1,
              "authorised_quantity": 0,
              "authorised_date": "2025-01-17 00:00:00",
              "authorisation": {},
              "opening_quantity": 1,
              "short_quantity": 0,
              "collateral_quantity": 0,
              "collateral_type": "",
              "discrepancy": false,
              "average_price": 161,
              "last_price": 352.95,
              "close_price": 352.35,
              "pnl": 191.95,
              "day_change": 0.5999999999999659,
              "day_change_percentage": 0.17028522775648244,
              "mtf": {
                "quantity": 1000,
                "used_quantity": 0,
                "average_price": 100,
                "value": 100000,
                "initial_margin": 0
              }
            },
            {
              "tradingsymbol": "SBIN",
              "exchange": "BSE",
              "instrument_token": 128028676,
              "isin": "INE062A01020",
              "product": "CNC",
              "price": 0,
              "quantity": 16,
              "used_quantity": 0,
              "t1_quantity": 0,
              "realised_quantity": 16,
              "authorised_quantity": 0,
              "authorised_date": "2025-01-17 00:00:00",
              "authorisation": {},
              "opening_quantity": 16,
              "short_quantity": 0,
              "collateral_quantity": 0,
              "collateral_type": "",
              "discrepancy": false,
              "average_price": 801.78125,
              "last_price": 762.45,
              "close_price": 766.4,
              "pnl": -629.2999999999993,
              "day_change": -3.949999999999932,
              "day_change_percentage": -0.5153966597077155,
              "mtf": {
                "quantity": 0,
                "used_quantity": 0,
                "average_price": 0,
                "value": 0,
                "initial_margin": 0
              }
            }
          ]
        }"#;

        let value: Response<_> = serde_json::from_str(json)?;

        let expected = Response::Success {
            data: vec![
                Holding {
                    trading_symbol: "AARON".into(),
                    exchange: Exchange::NSE,
                    instrument_token: 263681,
                    isin: "INE721Z01010".into(),
                    product: Product::CNC,
                    price: 0.0,
                    quantity: 1,
                    used_quantity: 0,
                    t1_quantity: 0,
                    realised_quantity: 1,
                    authorised_quantity: 0,
                    authorised_date: "2025-01-17 00:00:00".into(),
                    authorisation: serde_json::json!({}),
                    opening_quantity: 1,
                    short_quantity: 0,
                    collateral_quantity: 0,
                    collateral_type: Some("".into()),
                    discrepancy: false,
                    average_price: 161.0,
                    last_price: 352.95,
                    close_price: 352.35,
                    pnl: 191.95,
                    day_change: 0.5999999999999659,
                    day_change_percentage: 0.17028522775648244,
                    mtf: serde_json::json!({
                        "quantity": 1000,
                        "used_quantity": 0,
                        "average_price": 100,
                        "value": 100000,
                        "initial_margin": 0
                    }),
                },
                Holding {
                    trading_symbol: "SBIN".into(),
                    exchange: Exchange::BSE,
                    instrument_token: 128028676,
                    isin: "INE062A01020".into(),
                    product: Product::CNC,
                    price: 0.0,
                    quantity: 16,
                    used_quantity: 0,
                    t1_quantity: 0,
                    realised_quantity: 16,
                    authorised_quantity: 0,
                    authorised_date: "2025-01-17 00:00:00".into(),
                    authorisation: serde_json::json!({}),
                    opening_quantity: 16,
                    short_quantity: 0,
                    collateral_quantity: 0,
                    collateral_type: Some("".into()),
                    discrepancy: false,
                    average_price: 801.78125,
                    last_price: 762.45,
                    close_price: 766.4,
                    pnl: -629.2999999999993,
                    day_change: -3.949999999999932,
                    day_change_percentage: -0.5153966597077155,
                    mtf: serde_json::json!({
                        "quantity": 0,
                        "used_quantity": 0,
                        "average_price": 0,
                        "value": 0,
                        "initial_margin": 0
                    }),
                },
            ],
        };

        assert_eq!(value, expected);

        Ok(())
    }

    #[test]
    fn test_auction_holdings() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
          "status": "success",
          "data": [
            {
              "tradingsymbol": "ASHOKLEY",
              "exchange": "NSE",
              "instrument_token": 54282,
              "isin": "INE208A01029",
              "product": "CNC",
              "price": 0,
              "quantity": 1,
              "t1_quantity": 0,
              "realised_quantity": 1,
              "authorised_quantity": 0,
              "authorised_date": "2022-12-21 00:00:00",
              "opening_quantity": 1,
              "collateral_quantity": 0,
              "collateral_type": "",
              "discrepancy": false,
              "average_price": 131.95,
              "last_price": 142.5,
              "close_price": 145.1,
              "pnl": 10.550000000000011,
              "day_change": -2.5999999999999943,
              "day_change_percentage": -1.79186767746,
              "auction_number": "20"
            },
            {
              "tradingsymbol": "BHEL",
              "exchange": "NSE",
              "instrument_token": 112138,
              "isin": "INE257A01026",
              "product": "CNC",
              "price": 0,
              "quantity": 5,
              "t1_quantity": 0,
              "realised_quantity": 5,
              "authorised_quantity": 0,
              "authorised_date": "2022-12-21 00:00:00",
              "opening_quantity": 5,
              "collateral_quantity": 0,
              "collateral_type": "",
              "discrepancy": false,
              "average_price": 75.95,
              "last_price": 81.1,
              "close_price": 84,
              "pnl": 25.749999999999957,
              "day_change": -2.9000000000000057,
              "day_change_percentage": -3.4523809523809588,
              "auction_number": "34"
            }
          ]
        }"#;

        let value: Response<_> = serde_json::from_str(json)?;

        let expected = Response::Success {
            data: vec![
                HoldingAuction {
                    trading_symbol: "ASHOKLEY".into(),
                    exchange: Exchange::NSE,
                    instrument_token: 54282,
                    isin: "INE208A01029".into(),
                    product: Product::CNC,
                    price: 0.0,
                    quantity: 1,
                    t1_quantity: 0,
                    realised_quantity: 1,
                    authorised_quantity: 0,
                    authorised_date: "2022-12-21 00:00:00".into(),
                    opening_quantity: 1,
                    collateral_quantity: 0,
                    collateral_type: Some("".into()),
                    discrepancy: false,
                    average_price: 131.95,
                    last_price: 142.5,
                    close_price: 145.1,
                    pnl: 10.550000000000011,
                    day_change: -2.5999999999999943,
                    day_change_percentage: -1.79186767746,
                    auction_number: "20".into(),
                },
                HoldingAuction {
                    trading_symbol: "BHEL".into(),
                    exchange: Exchange::NSE,
                    instrument_token: 112138,
                    isin: "INE257A01026".into(),
                    product: Product::CNC,
                    price: 0.0,
                    quantity: 5,
                    t1_quantity: 0,
                    realised_quantity: 5,
                    authorised_quantity: 0,
                    authorised_date: "2022-12-21 00:00:00".into(),
                    opening_quantity: 5,
                    collateral_quantity: 0,
                    collateral_type: Some("".into()),
                    discrepancy: false,
                    average_price: 75.95,
                    last_price: 81.1,
                    close_price: 84.0,
                    pnl: 25.749999999999957,
                    day_change: -2.9000000000000057,
                    day_change_percentage: -3.4523809523809588,
                    auction_number: "34".into(),
                },
            ],
        };

        assert_eq!(value, expected);

        Ok(())
    }

    #[test]
    fn test_positions() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "status": "success",
            "data": {
                "net": [
                    {
                        "tradingsymbol": "LEADMINI17DECFUT",
                        "exchange": "MCX",
                        "instrument_token": 53496327,
                        "product": "NRML",
                        "quantity": 1,
                        "overnight_quantity": 0,
                        "multiplier": 1000,
                        "average_price": 161.05,
                        "close_price": 0,
                        "last_price": 161.05,
                        "value": -161050,
                        "pnl": 0,
                        "m2m": 0,
                        "unrealised": 0,
                        "realised": 0,
                        "buy_quantity": 1,
                        "buy_price": 161.05,
                        "buy_value": 161050,
                        "buy_m2m": 161050,
                        "sell_quantity": 0,
                        "sell_price": 0,
                        "sell_value": 0,
                        "sell_m2m": 0,
                        "day_buy_quantity": 1,
                        "day_buy_price": 161.05,
                        "day_buy_value": 161050,
                        "day_sell_quantity": 0,
                        "day_sell_price": 0,
                        "day_sell_value": 0
                    },
                    {
                        "tradingsymbol": "GOLDGUINEA17DECFUT",
                        "exchange": "MCX",
                        "instrument_token": 53505799,
                        "product": "NRML",
                        "quantity": 0,
                        "overnight_quantity": 3,
                        "multiplier": 1,
                        "average_price": 0,
                        "close_price": 23232,
                        "last_price": 23355,
                        "value": 801,
                        "pnl": 801,
                        "m2m": 276,
                        "unrealised": 801,
                        "realised": 0,
                        "buy_quantity": 4,
                        "buy_price": 23139.75,
                        "buy_value": 92559,
                        "buy_m2m": 93084,
                        "sell_quantity": 4,
                        "sell_price": 23340,
                        "sell_value": 93360,
                        "sell_m2m": 93360,
                        "day_buy_quantity": 1,
                        "day_buy_price": 23388,
                        "day_buy_value": 23388,
                        "day_sell_quantity": 4,
                        "day_sell_price": 23340,
                        "day_sell_value": 93360
                    },
                    {
                        "tradingsymbol": "SBIN",
                        "exchange": "NSE",
                        "instrument_token": 779521,
                        "product": "CO",
                        "quantity": 0,
                        "overnight_quantity": 0,
                        "multiplier": 1,
                        "average_price": 0,
                        "close_price": 0,
                        "last_price": 308.4,
                        "value": -2,
                        "pnl": -2,
                        "m2m": -2,
                        "unrealised": -2,
                        "realised": 0,
                        "buy_quantity": 1,
                        "buy_price": 311,
                        "buy_value": 311,
                        "buy_m2m": 311,
                        "sell_quantity": 1,
                        "sell_price": 309,
                        "sell_value": 309,
                        "sell_m2m": 309,
                        "day_buy_quantity": 1,
                        "day_buy_price": 311,
                        "day_buy_value": 311,
                        "day_sell_quantity": 1,
                        "day_sell_price": 309,
                        "day_sell_value": 309
                    }
                ],
                "day": [
                    {
                        "tradingsymbol": "GOLDGUINEA17DECFUT",
                        "exchange": "MCX",
                        "instrument_token": 53505799,
                        "product": "NRML",
                        "quantity": -3,
                        "overnight_quantity": 0,
                        "multiplier": 1,
                        "average_price": 23340,
                        "close_price": 23232,
                        "last_price": 23355,
                        "value": 69972,
                        "pnl": -93,
                        "m2m": -93,
                        "unrealised": -93,
                        "realised": 0,
                        "buy_quantity": 1,
                        "buy_price": 23388,
                        "buy_value": 23388,
                        "buy_m2m": 23388,
                        "sell_quantity": 4,
                        "sell_price": 23340,
                        "sell_value": 93360,
                        "sell_m2m": 93360,
                        "day_buy_quantity": 1,
                        "day_buy_price": 23388,
                        "day_buy_value": 23388,
                        "day_sell_quantity": 4,
                        "day_sell_price": 23340,
                        "day_sell_value": 93360
                    },
                    {
                        "tradingsymbol": "LEADMINI17DECFUT",
                        "exchange": "MCX",
                        "instrument_token": 53496327,
                        "product": "NRML",
                        "quantity": 1,
                        "overnight_quantity": 0,
                        "multiplier": 1000,
                        "average_price": 161.05,
                        "close_price": 0,
                        "last_price": 161.05,
                        "value": -161050,
                        "pnl": 0,
                        "m2m": 0,
                        "unrealised": 0,
                        "realised": 0,
                        "buy_quantity": 1,
                        "buy_price": 161.05,
                        "buy_value": 161050,
                        "buy_m2m": 161050,
                        "sell_quantity": 0,
                        "sell_price": 0,
                        "sell_value": 0,
                        "sell_m2m": 0,
                        "day_buy_quantity": 1,
                        "day_buy_price": 161.05,
                        "day_buy_value": 161050,
                        "day_sell_quantity": 0,
                        "day_sell_price": 0,
                        "day_sell_value": 0
                    },
                    {
                        "tradingsymbol": "SBIN",
                        "exchange": "NSE",
                        "instrument_token": 779521,
                        "product": "CO",
                        "quantity": 0,
                        "overnight_quantity": 0,
                        "multiplier": 1,
                        "average_price": 0,
                        "close_price": 0,
                        "last_price": 308.4,
                        "value": -2,
                        "pnl": -2,
                        "m2m": -2,
                        "unrealised": -2,
                        "realised": 0,
                        "buy_quantity": 1,
                        "buy_price": 311,
                        "buy_value": 311,
                        "buy_m2m": 311,
                        "sell_quantity": 1,
                        "sell_price": 309,
                        "sell_value": 309,
                        "sell_m2m": 309,
                        "day_buy_quantity": 1,
                        "day_buy_price": 311,
                        "day_buy_value": 311,
                        "day_sell_quantity": 1,
                        "day_sell_price": 309,
                        "day_sell_value": 309
                    }
                ]
            }
        }"#;

        let value: Response<_> = serde_json::from_str(json)?;

        let expected = Response::Success {
            data: Positions {
                net: vec![
                    Position {
                        trading_symbol: "LEADMINI17DECFUT".into(),
                        exchange: Exchange::MCX,
                        instrument_token: 53496327,
                        product: Product::NRML,
                        quantity: 1,
                        overnight_quantity: 0,
                        multiplier: 1000,
                        average_price: 161.05,
                        close_price: 0.0,
                        last_price: 161.05,
                        value: -161050.0,
                        pnl: 0.0,
                        m2m: 0.0,
                        unrealised: 0.0,
                        realised: 0.0,
                        buy_quantity: 1,
                        buy_price: 161.05,
                        buy_value: 161050.0,
                        buy_m2m: 161050.0,
                        sell_quantity: 0,
                        sell_price: 0.0,
                        sell_value: 0.0,
                        sell_m2m: 0.0,
                        day_buy_quantity: 1,
                        day_buy_price: 161.05,
                        day_buy_value: 161050.0,
                        day_sell_quantity: 0,
                        day_sell_price: 0.0,
                        day_sell_value: 0.0,
                    },
                    Position {
                        trading_symbol: "GOLDGUINEA17DECFUT".into(),
                        exchange: Exchange::MCX,
                        instrument_token: 53505799,
                        product: Product::NRML,
                        quantity: 0,
                        overnight_quantity: 3,
                        multiplier: 1,
                        average_price: 0.0,
                        close_price: 23232.0,
                        last_price: 23355.0,
                        value: 801.0,
                        pnl: 801.0,
                        m2m: 276.0,
                        unrealised: 801.0,
                        realised: 0.0,
                        buy_quantity: 4,
                        buy_price: 23139.75,
                        buy_value: 92559.0,
                        buy_m2m: 93084.0,
                        sell_quantity: 4,
                        sell_price: 23340.0,
                        sell_value: 93360.0,
                        sell_m2m: 93360.0,
                        day_buy_quantity: 1,
                        day_buy_price: 23388.0,
                        day_buy_value: 23388.0,
                        day_sell_quantity: 4,
                        day_sell_price: 23340.0,
                        day_sell_value: 93360.0,
                    },
                    Position {
                        trading_symbol: "SBIN".into(),
                        exchange: Exchange::NSE,
                        instrument_token: 779521,
                        product: Product::CO,
                        quantity: 0,
                        overnight_quantity: 0,
                        multiplier: 1,
                        average_price: 0.0,
                        close_price: 0.0,
                        last_price: 308.4,
                        value: -2.0,
                        pnl: -2.0,
                        m2m: -2.0,
                        unrealised: -2.0,
                        realised: 0.0,
                        buy_quantity: 1,
                        buy_price: 311.0,
                        buy_value: 311.0,
                        buy_m2m: 311.0,
                        sell_quantity: 1,
                        sell_price: 309.0,
                        sell_value: 309.0,
                        sell_m2m: 309.0,
                        day_buy_quantity: 1,
                        day_buy_price: 311.0,
                        day_buy_value: 311.0,
                        day_sell_quantity: 1,
                        day_sell_price: 309.0,
                        day_sell_value: 309.0,
                    },
                ],
                day: vec![
                    Position {
                        trading_symbol: "GOLDGUINEA17DECFUT".into(),
                        exchange: Exchange::MCX,
                        instrument_token: 53505799,
                        product: Product::NRML,
                        quantity: -3,
                        overnight_quantity: 0,
                        multiplier: 1,
                        average_price: 23340.0,
                        close_price: 23232.0,
                        last_price: 23355.0,
                        value: 69972.0,
                        pnl: -93.0,
                        m2m: -93.0,
                        unrealised: -93.0,
                        realised: 0.0,
                        buy_quantity: 1,
                        buy_price: 23388.0,
                        buy_value: 23388.0,
                        buy_m2m: 23388.0,
                        sell_quantity: 4,
                        sell_price: 23340.0,
                        sell_value: 93360.0,
                        sell_m2m: 93360.0,
                        day_buy_quantity: 1,
                        day_buy_price: 23388.0,
                        day_buy_value: 23388.0,
                        day_sell_quantity: 4,
                        day_sell_price: 23340.0,
                        day_sell_value: 93360.0,
                    },
                    Position {
                        trading_symbol: "LEADMINI17DECFUT".into(),
                        exchange: Exchange::MCX,
                        instrument_token: 53496327,
                        product: Product::NRML,
                        quantity: 1,
                        overnight_quantity: 0,
                        multiplier: 1000,
                        average_price: 161.05,
                        close_price: 0.0,
                        last_price: 161.05,
                        value: -161050.0,
                        pnl: 0.0,
                        m2m: 0.0,
                        unrealised: 0.0,
                        realised: 0.0,
                        buy_quantity: 1,
                        buy_price: 161.05,
                        buy_value: 161050.0,
                        buy_m2m: 161050.0,
                        sell_quantity: 0,
                        sell_price: 0.0,
                        sell_value: 0.0,
                        sell_m2m: 0.0,
                        day_buy_quantity: 1,
                        day_buy_price: 161.05,
                        day_buy_value: 161050.0,
                        day_sell_quantity: 0,
                        day_sell_price: 0.0,
                        day_sell_value: 0.0,
                    },
                    Position {
                        trading_symbol: "SBIN".into(),
                        exchange: Exchange::NSE,
                        instrument_token: 779521,
                        product: Product::CO,
                        quantity: 0,
                        overnight_quantity: 0,
                        multiplier: 1,
                        average_price: 0.0,
                        close_price: 0.0,
                        last_price: 308.4,
                        value: -2.0,
                        pnl: -2.0,
                        m2m: -2.0,
                        unrealised: -2.0,
                        realised: 0.0,
                        buy_quantity: 1,
                        buy_price: 311.0,
                        buy_value: 311.0,
                        buy_m2m: 311.0,
                        sell_quantity: 1,
                        sell_price: 309.0,
                        sell_value: 309.0,
                        sell_m2m: 309.0,
                        day_buy_quantity: 1,
                        day_buy_price: 311.0,
                        day_buy_value: 311.0,
                        day_sell_quantity: 1,
                        day_sell_price: 309.0,
                        day_sell_value: 309.0,
                    },
                ],
            },
        };

        assert_eq!(value, expected);

        Ok(())
    }
}

use super::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub const PLACE_REGULAR_ORDER_ENDPOINT: &str = "https://api.kite.trade/orders/regular";
pub const PLACE_AMO_ORDER_ENDPOINT: &str = "https://api.kite.trade/orders/amo";
pub const PLACE_CO_ORDER_ENDPOINT: &str = "https://api.kite.trade/orders/co";
pub const PLACE_ICEBERG_ORDER_ENDPOINT: &str = "https://api.kite.trade/orders/iceberg";
pub const PLACE_AUCTION_ORDER_ENDPOINT: &str = "https://api.kite.trade/orders/auction";

pub const MODIFY_REGULAR_ORDER_ENDPOINT: &str = "https://api.kite.trade/orders/regular/";
pub const MODIFY_COVER_ORDER_ENDPOINT: &str = "https://api.kite.trade/orders/regular/co/";

pub const CANCEL_REGULAR_ORDER_ENDPOINT: &str = "https://api.kite.trade/orders/regular/";
pub const CANCEL_AMO_ORDER_ENDPOINT: &str = "https://api.kite.trade/orders/amo/";
pub const CANCEL_CO_ORDER_ENDPOINT: &str = "https://api.kite.trade/orders/co/";
pub const CANCEL_ICEBERG_ORDER_ENDPOINT: &str = "https://api.kite.trade/orders/iceberg/";
pub const CANCEL_AUCTION_ORDER_ENDPOINT: &str = "https://api.kite.trade/orders/auction/";

pub const GET_ORDERS_ENDPOINT: &str = "https://api.kite.trade/orders";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Variety {
    /// Regular order
    Regular,
    /// After Market Order
    AMO,
    /// Cover Order. Read More <https://support.zerodha.com/category/trading-and-markets/charts-and-orders/order/articles/what-are-cover-orders-and-how-to-use-them>
    CO,
    /// Iceberg Order. Read More <https://support.zerodha.com/category/trading-and-markets/charts-and-orders/order/articles/iceberg-orders>
    IceBerg,
    /// Auction Order. Read More <https://support.zerodha.com/category/trading-and-markets/general-kite/auctions/articles/participation-in-the-auction>
    Auction,
}

/// Represents an exchange
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Exchange {
    /// BSE Futures & Options
    BFO,
    /// Multi Commodity Exchange
    MCX,
    /// National Stock Exchange
    NSE,
    /// Currency Derivatives Segment
    CDS,
    /// Bombay Stock Exchange
    BSE,
    /// Bombay Currency Derivatives
    BCD,
    /// Mutual Funds
    MF,
    /// NSE Futures & Options
    NFO,
}

impl Display for Exchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Exchange::BFO => "BFO",
            Exchange::MCX => "MCX",
            Exchange::NSE => "NSE",
            Exchange::CDS => "CDS",
            Exchange::BSE => "BSE",
            Exchange::BCD => "BCD",
            Exchange::MF => "MF",
            Exchange::NFO => "NFO",
        };
        write!(f, "{}", s)
    }
}

/// Margin product
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Product {
    /// Cash and Carry
    CNC,
    /// Normal
    NRML,
    /// Margin Intraday Square-off
    MIS,
    /// Bracket Order
    BO,
    /// Cover Order
    CO,
}

/// Order types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    /// Market order
    #[serde(rename = "MARKET")]
    Market,
    /// Limit order
    #[serde(rename = "LIMIT")]
    Limit,
    /// Stop Loss order
    SL,
    /// Stop Loss Market order
    #[allow(non_camel_case_types)]
    #[serde(rename = "SL-M")]
    SL_M,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Validity {
    Day,
    Ioc,
    TTL,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionType {
    Buy,
    Sell,
}

/// Read More: <https://zerodha.com/varsity/chapter/understanding-the-various-order-types/>
// TODO: Some properties depend on variety, while some on OrderType. Have these type store that extra
// metadata so it is easier to create correct request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaceOrderRequest {
    #[serde(skip_serializing)]
    pub variety: Variety,
    /// Tradingsymbol of the instrument
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    /// Name of the exchange (NSE, BSE, NFO, CDS, BCD, MCX)
    pub exchange: Exchange,
    /// BUY or SELL
    pub transaction_type: TransactionType,
    /// Order type (MARKET, LIMIT etc.)
    pub order_type: OrderType,
    /// Quantity to transact
    pub quantity: u32,
    /// Margin product to use for the order (margins are blocked based on this)
    pub product: Product,
    /// The price to execute the order at (for LIMIT orders)
    pub price: Option<f64>,
    /// The price at which an order should be triggered (SL, SL-M)
    pub trigger_price: Option<f64>,
    /// Quantity to disclose publicly (for equity trades)
    pub disclosed_quantity: Option<u32>,
    /// Order validity (DAY, IOC and TTL)
    pub validity: Validity,
    /// Order life span in minutes for TTL validity orders
    pub validity_ttl: Option<u32>,
    /// Total number of legs for iceberg order type (number of legs per Iceberg should be between 2 and 10)
    pub iceberg_legs: Option<u32>,
    /// Split quantity for each iceberg leg order (quantity/iceberg_legs)
    pub iceberg_quantity: Option<u32>,
    /// A unique identifier for a particular auction
    pub auction_number: Option<String>,
    /// An optional tag to apply to an order to identify it (alphanumeric, max 20 chars)
    pub tag: Option<String>,
}

// TODO: Add utility functions to create order

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModifyRegularOrderRequest {
    pub order_type: Option<OrderType>,
    pub quantity: Option<u32>,
    pub price: Option<f64>,
    pub trigger_price: Option<f64>,
    pub disclosed_quantity: Option<u32>,
    pub validity: Option<Validity>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModifyCoverOrderRequest {
    /// Unique order ID
    pub order_id: Option<String>,
    /// The price to execute the order at
    pub price: Option<f64>,
    /// For LIMIT Cover orders
    pub trigger_price: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderStatus {
    Open,
    Cancelled,
    Rejected,
    Complete,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    /// Unique order ID
    pub order_id: String,
    /// Order ID of the parent order (only applicable in case of multi-legged orders like CO)
    pub parent_order_id: Option<String>,
    /// Exchange generated order ID. Orders that don't reach the exchange have null IDs
    pub exchange_order_id: Option<String>,
    /// Indicate that the order has been modified since placement by the user
    pub modified: bool,
    /// ID of the user that placed the order. This may different from the user's ID for orders
    /// placed outside of Kite, for instance, by dealers at the brokerage using dealer terminals
    pub placed_by: String,
    /// Order variety (regular, amo, co etc.)
    pub variety: Variety,
    /// Current status of the order. Most common values or COMPLETE, REJECTED, CANCELLED, and OPEN.
    /// There may be other values as well.
    pub status: OrderStatus,
    /// Exchange tradingsymbol of the instrument
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    /// Exchange
    pub exchange: Exchange,
    /// The numerical identifier issued by the exchange representing the instrument. Used for
    /// subscribing to live market data over WebSocket
    #[serde(deserialize_with = "crate::utils::deserialize_number_or_string")]
    pub instrument_token: String,
    /// BUY or SELL
    pub transaction_type: TransactionType,
    /// Order type (MARKET, LIMIT etc.)
    pub order_type: OrderType,
    /// Margin product to use for the order (margins are blocked based on this)
    pub product: Product,
    /// Order validity
    pub validity: Validity,
    /// Price at which the order was placed (LIMIT orders)
    pub price: Option<f64>,
    /// Quantity ordered
    pub quantity: u32,
    /// Trigger price (for SL, SL-M, CO orders)
    pub trigger_price: Option<f64>,
    /// Average price at which the order was executed (only for COMPLETE orders)
    pub average_price: Option<f64>,
    /// Pending quantity to be filled
    pub pending_quantity: u32,
    /// Quantity that's been filled
    pub filled_quantity: u32,
    /// Quantity to be disclosed (may be different from actual quantity) to the public exchange
    /// orderbook. Only for equities
    pub disclosed_quantity: Option<u32>,
    /// Timestamp at which the order was registered by the API
    pub order_timestamp: String,
    /// Timestamp at which the order was registered by the exchange. Orders that don't reach
    /// the exchange have null timestamps
    pub exchange_timestamp: Option<String>,
    /// Timestamp at which an order's state changed at the exchange
    pub exchange_update_timestamp: Option<String>,
    /// Textual description of the order's status. Failed orders come with human readable explanation
    pub status_message: Option<String>,
    /// Raw textual description of the failed order's status, as received from the OMS
    pub status_message_raw: Option<String>,
    /// Quantity that's cancelled
    pub cancelled_quantity: u32,
    /// A unique identifier for a particular auction
    pub auction_number: Option<String>,
    /// An optional tag to apply to an order to identify it (alphanumeric, max 20 chars)
    pub tag: Option<String>,
    /// Unusable request id to avoid order duplication
    pub guid: String,
    /// Map of arbitrary fields that the system may attach to an order.
    #[serde(flatten)]
    pub meta: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct Data {
    order_id: String,
}

impl KiteConnect<Authenticated> {
    pub async fn place_order(&self, req: &PlaceOrderRequest) -> Result<(), Error> {
        let endpoint = place_order_endpoint_url_impl(&req.variety);

        match self
            .client
            .post(endpoint)
            .form(req)
            .timeout(std::time::Duration::from_millis(50))
            .send()
            .await
        {
            Ok(r) => r.json::<Response<Data>>().await?.into_result()?,
            Err(err) => {
                if err.is_timeout() {
                    return Ok(());
                } else {
                    return Err(err.into());
                }
            }
        };

        Ok(())
    }

    pub async fn place_order_poll(&self, req: &PlaceOrderRequest) -> Result<String, Error> {
        let endpoint = place_order_endpoint_url_impl(&req.variety);

        Ok(self
            .client
            .post(endpoint)
            .form(req)
            .send()
            .await?
            .json::<Response<Data>>()
            .await?
            .into_result()?
            .order_id)
    }

    pub async fn modify_regular_oder(
        &self,
        order_id: &str,
        req: &ModifyRegularOrderRequest,
    ) -> Result<(), Error> {
        let _ = self
            .client
            .put(format!("{MODIFY_REGULAR_ORDER_ENDPOINT}{order_id}"))
            .form(req)
            .send()
            .await?
            .json::<Response<Data>>()
            .await?
            .into_result()?;

        Ok(())
    }

    pub async fn modify_cover_order(
        &self,
        order_id: &str,
        req: &ModifyCoverOrderRequest,
    ) -> Result<(), Error> {
        let _ = self
            .client
            .put(format!("{MODIFY_COVER_ORDER_ENDPOINT}{order_id}"))
            .form(req)
            .send()
            .await?
            .json::<Response<Data>>()
            .await?
            .into_result()?;

        Ok(())
    }

    pub async fn cancel_order(&self, order_id: &str, variety: &Variety) -> Result<(), Error> {
        let endpoint = cancel_order_endpoint_url_impl(variety);

        let _ = self
            .client
            .delete(format!("{endpoint}{order_id}"))
            .send()
            .await?
            .json::<Response<Data>>()
            .await?
            .into_result()?;
        Ok(())
    }

    pub async fn get_orders(&self) -> Result<Order, Error> {
        Ok(self
            .client
            .get(GET_ORDERS_ENDPOINT)
            .send()
            .await?
            .json::<Response<_>>()
            .await?
            .into_result()?)
    }
}

const fn place_order_endpoint_url_impl(variety: &Variety) -> &'static str {
    match variety {
        Variety::Regular => PLACE_REGULAR_ORDER_ENDPOINT,
        Variety::AMO => PLACE_AMO_ORDER_ENDPOINT,
        Variety::CO => PLACE_CO_ORDER_ENDPOINT,
        Variety::IceBerg => PLACE_ICEBERG_ORDER_ENDPOINT,
        Variety::Auction => PLACE_AUCTION_ORDER_ENDPOINT,
    }
}

const fn cancel_order_endpoint_url_impl(variety: &Variety) -> &'static str {
    match variety {
        Variety::Regular => CANCEL_REGULAR_ORDER_ENDPOINT,
        Variety::AMO => CANCEL_AMO_ORDER_ENDPOINT,
        Variety::CO => CANCEL_CO_ORDER_ENDPOINT,
        Variety::IceBerg => CANCEL_ICEBERG_ORDER_ENDPOINT,
        Variety::Auction => CANCEL_AUCTION_ORDER_ENDPOINT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_req() -> Result<(), Box<dyn std::error::Error>> {
        let order_req = PlaceOrderRequest {
            variety: Variety::Regular,
            trading_symbol: "COROMANDEL".to_string(),
            exchange: Exchange::NSE,
            transaction_type: TransactionType::Buy,
            order_type: OrderType::Market,
            quantity: 1,
            product: Product::CNC,
            price: None,
            trigger_price: None,
            disclosed_quantity: None,
            validity: Validity::TTL,
            validity_ttl: Some(2),
            iceberg_legs: None,
            iceberg_quantity: None,
            auction_number: None,
            tag: Some("Nobelium".to_string()),
        };

        let value = serde_urlencoded::to_string(order_req)?;
        assert_eq!(value, "tradingsymbol=COROMANDEL&exchange=NSE&transaction_type=BUY&order_type=MARKET&quantity=1&product=CNC&validity=TTL&validity_ttl=2&tag=Nobelium".to_string());

        Ok(())
    }

    #[test]
    fn test_orders() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
          "status": "success",
          "data": [
            {
              "placed_by": "XXXXXX",
              "order_id": "100000000000000",
              "exchange_order_id": "200000000000000",
              "parent_order_id": null,
              "status": "CANCELLED",
              "status_message": null,
              "status_message_raw": null,
              "order_timestamp": "2021-05-31 09:18:57",
              "exchange_update_timestamp": "2021-05-31 09:18:58",
              "exchange_timestamp": "2021-05-31 09:15:38",
              "variety": "regular",
              "modified": false,
              "exchange": "CDS",
              "tradingsymbol": "USDINR21JUNFUT",
              "instrument_token": 412675,
              "order_type": "LIMIT",
              "transaction_type": "BUY",
              "validity": "DAY",
              "product": "NRML",
              "quantity": 1,
              "disclosed_quantity": 0,
              "price": 72,
              "trigger_price": 0,
              "average_price": 0,
              "filled_quantity": 0,
              "pending_quantity": 1,
              "cancelled_quantity": 1,
              "market_protection": 0,
              "meta": {},
              "tag": null,
              "guid": "XXXXX"
            },
            {
              "placed_by": "XXXXXX",
              "order_id": "300000000000000",
              "exchange_order_id": "400000000000000",
              "parent_order_id": null,
              "status": "COMPLETE",
              "status_message": null,
              "status_message_raw": null,
              "order_timestamp": "2021-05-31 15:20:28",
              "exchange_update_timestamp": "2021-05-31 15:20:28",
              "exchange_timestamp": "2021-05-31 15:20:28",
              "variety": "regular",
              "modified": false,
              "exchange": "NSE",
              "tradingsymbol": "IOC",
              "instrument_token": 415745,
              "order_type": "LIMIT",
              "transaction_type": "BUY",
              "validity": "DAY",
              "product": "CNC",
              "quantity": 1,
              "disclosed_quantity": 0,
              "price": 109.4,
              "trigger_price": 0,
              "average_price": 109.4,
              "filled_quantity": 1,
              "pending_quantity": 0,
              "cancelled_quantity": 0,
              "market_protection": 0,
              "meta": {},
              "tag": null,
              "guid": "XXXXXX"
            }
          ]
        }"#;

        let value: Response<_> = serde_json::from_str(json)?;

        let expected = Response::Success {
            data: vec![
                Order {
                    placed_by: "XXXXXX".into(),
                    order_id: "100000000000000".into(),
                    exchange_order_id: Some("200000000000000".into()),
                    parent_order_id: None,
                    status: OrderStatus::Cancelled,
                    status_message: None,
                    status_message_raw: None,
                    order_timestamp: "2021-05-31 09:18:57".into(),
                    exchange_update_timestamp: Some("2021-05-31 09:18:58".into()),
                    exchange_timestamp: Some("2021-05-31 09:15:38".into()),
                    variety: Variety::Regular,
                    modified: false,
                    exchange: Exchange::CDS,
                    trading_symbol: "USDINR21JUNFUT".into(),
                    instrument_token: "412675".into(),
                    order_type: OrderType::Limit,
                    transaction_type: TransactionType::Buy,
                    validity: Validity::Day,
                    product: Product::NRML,
                    quantity: 1,
                    disclosed_quantity: Some(0),
                    price: Some(72.0),
                    trigger_price: Some(0.0),
                    average_price: Some(0.0),
                    filled_quantity: 0,
                    pending_quantity: 1,
                    cancelled_quantity: 1,
                    tag: None,
                    guid: "XXXXX".into(),
                    auction_number: None,
                    meta: Some(serde_json::json!({
                        "market_protection": 0,
                        "meta": {}
                    })),
                },
                Order {
                    placed_by: "XXXXXX".into(),
                    order_id: "300000000000000".into(),
                    exchange_order_id: Some("400000000000000".into()),
                    parent_order_id: None,
                    status: OrderStatus::Complete,
                    status_message: None,
                    status_message_raw: None,
                    order_timestamp: "2021-05-31 15:20:28".into(),
                    exchange_update_timestamp: Some("2021-05-31 15:20:28".into()),
                    exchange_timestamp: Some("2021-05-31 15:20:28".into()),
                    variety: Variety::Regular,
                    modified: false,
                    exchange: Exchange::NSE,
                    trading_symbol: "IOC".into(),
                    instrument_token: "415745".into(),
                    order_type: OrderType::Limit,
                    transaction_type: TransactionType::Buy,
                    validity: Validity::Day,
                    product: Product::CNC,
                    quantity: 1,
                    disclosed_quantity: Some(0),
                    price: Some(109.4),
                    average_price: Some(109.4),
                    trigger_price: Some(0.0),
                    filled_quantity: 1,
                    pending_quantity: 0,
                    cancelled_quantity: 0,
                    tag: None,
                    guid: "XXXXXX".into(),
                    auction_number: None,
                    meta: Some(serde_json::json!({
                        "market_protection": 0,
                        // TODO: Make the values of meta, go inside the top level meta object
                        "meta": {}
                    })),
                },
            ],
        };

        assert_eq!(value, expected);

        Ok(())
    }
}

use super::*;
use serde::{Deserialize, Serialize};

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
}

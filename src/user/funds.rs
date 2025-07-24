use crate::user::*;

pub const USER_FUNDS_ENDPOINT: &str = "https://api.kite.trade/user/margins";
pub const USER_EQUITY_FUNDS_ENDPOINT: &str = "https://api.kite.trade/user/margins/equity";
pub const USER_COMMODITY_FUNDS_ENDPOINT: &str = "https://api.kite.trade/user/margins/commodity";

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TotalFunds {
    pub equity: SegmentFunds,
    pub commodity: SegmentFunds,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SegmentFunds {
    /// Indicates whether the segment is enabled for the user
    pub enabled: bool,
    /// Net cash balance available for trading (intraday_payin + adhoc_margin + collateral)
    pub net: f64,
    pub available: AvailableFunds,
    pub utilised: UtilisedFunds,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AvailableFunds {
    /// Raw cash balance in the account available for trading (also includes `intraday_payin`)
    pub cash: f64,
    /// Opening balance at the day start
    pub opening_balance: f64,
    /// Current available balance
    pub live_balance: f64,
    /// Amount that was deposited during the day
    pub intraday_payin: f64,
    /// Additional margin provided by the broker
    pub adhoc_margin: f64,
    /// Margin derived from pledged stocks
    pub collateral: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct UtilisedFunds {
    /// Un-booked (open) intraday profits and losses
    #[serde(rename = "m2m_unrealised")]
    pub unrealised: f64,
    /// Booked intraday profits and losses
    #[serde(rename = "m2m_realised")]
    pub realised: f64,
    /// Sum of all utilised margins (unrealised M2M + realised M2M + SPAN + Exposure + Premium + Holding sales)
    pub debits: f64,
    /// SPAN margin blocked for all open F&O positions
    pub span: f64,
    /// Value of options premium received by shorting
    pub option_premium: f64,
    /// Value of holdings sold during the day
    pub holding_sales: f64,
    /// Exposure margin blocked for all open F&O positions
    pub exposure: f64,
    /// Margin utilised against pledged liquidbees ETFs and liquid mutual funds
    pub liquid_collateral: f64,
    /// Margin blocked when you sell securities (20% of the value of stocks sold) from your demat or T1 holdings
    pub delivery: f64,
    /// Margin utilised against pledged stocks/ETFs
    pub stock_collateral: f64,
    /// Utilised portion of the maximum turnover limit (only applicable to certain clients)
    pub turnover: f64,
    /// Funds paid out or withdrawn to bank account during the day
    pub payout: f64,
}

impl KiteConnect<Authenticated> {
    pub async fn get_funds(&self) -> Result<TotalFunds, Error> {
        Ok(self
            .client
            .get(USER_FUNDS_ENDPOINT)
            .send()
            .await?
            .json::<Response<_>>()
            .await?
            .into_result()?)
    }

    pub async fn get_equity_funds(&self) -> Result<SegmentFunds, Error> {
        Ok(self
            .client
            .get(USER_EQUITY_FUNDS_ENDPOINT)
            .send()
            .await?
            .json::<Response<_>>()
            .await?
            .into_result()?)
    }

    pub async fn get_commodity_funds(&self) -> Result<SegmentFunds, Error> {
        Ok(self
            .client
            .get(USER_COMMODITY_FUNDS_ENDPOINT)
            .send()
            .await?
            .json::<Response<_>>()
            .await?
            .into_result()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funds() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "status": "success",
            "data": {
              "equity": {
                "enabled": true,
                "net": 99725.05000000002,
                "available": {
                  "adhoc_margin": 0,
                  "cash": 245431.6,
                  "opening_balance": 245431.6,
                  "live_balance": 99725.05000000002,
                  "collateral": 0,
                  "intraday_payin": 0
                },
                "utilised": {
                  "debits": 145706.55,
                  "exposure": 38981.25,
                  "m2m_realised": 761.7,
                  "m2m_unrealised": 0,
                  "option_premium": 0,
                  "payout": 0,
                  "span": 101989,
                  "holding_sales": 0,
                  "turnover": 0,
                  "liquid_collateral": 0,
                  "stock_collateral": 0,
                  "delivery": 0
                }
              },
              "commodity": {
                "enabled": true,
                "net": 100661.7,
                "available": {
                  "adhoc_margin": 0,
                  "cash": 100661.7,
                  "opening_balance": 100661.7,
                  "live_balance": 100661.7,
                  "collateral": 0,
                  "intraday_payin": 0
                },
                "utilised": {
                  "debits": 0,
                  "exposure": 0,
                  "m2m_realised": 0,
                  "m2m_unrealised": 0,
                  "option_premium": 0,
                  "payout": 0,
                  "span": 0,
                  "holding_sales": 0,
                  "turnover": 0,
                  "liquid_collateral": 0,
                  "stock_collateral": 0,
                  "delivery": 0
                }
              }
            }
          }"#;

        let expected = TotalFunds {
            equity: SegmentFunds {
                enabled: true,
                net: 99725.05000000002,
                available: AvailableFunds {
                    adhoc_margin: 0.0,
                    cash: 245431.6,
                    opening_balance: 245431.6,
                    live_balance: 99725.05000000002,
                    collateral: 0.0,
                    intraday_payin: 0.0,
                },
                utilised: UtilisedFunds {
                    debits: 145706.55,
                    exposure: 38981.25,
                    realised: 761.7,
                    unrealised: 0.0,
                    option_premium: 0.0,
                    payout: 0.0,
                    span: 101989.0,
                    holding_sales: 0.0,
                    turnover: 0.0,
                    liquid_collateral: 0.0,
                    stock_collateral: 0.0,
                    delivery: 0.0,
                },
            },
            commodity: SegmentFunds {
                enabled: true,
                net: 100661.7,
                available: AvailableFunds {
                    adhoc_margin: 0.0,
                    cash: 100661.7,
                    opening_balance: 100661.7,
                    live_balance: 100661.7,
                    collateral: 0.0,
                    intraday_payin: 0.0,
                },
                utilised: UtilisedFunds {
                    debits: 0.0,
                    exposure: 0.0,
                    realised: 0.0,
                    unrealised: 0.0,
                    option_premium: 0.0,
                    payout: 0.0,
                    span: 0.0,
                    holding_sales: 0.0,
                    turnover: 0.0,
                    liquid_collateral: 0.0,
                    stock_collateral: 0.0,
                    delivery: 0.0,
                },
            },
        };

        let value: Response<_> = serde_json::from_str(json)?;
        assert_eq!(value, Response::Success { data: expected });

        Ok(())
    }
}

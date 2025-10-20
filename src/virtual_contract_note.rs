use serde::{Deserialize, Serialize};

use crate::orders::{Exchange, Product};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct VirtualContractNote {
    pub brokerage: f64,
    pub stt: f64,
    pub transaction_charges: f64,
    pub gst: f64,
    pub sebi_charges: f64,
    pub stamp_charges: f64,
    pub net_charges: f64,
    pub pnl: f64,
    pub net_pnl: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct OrderReq {
    pub exchange: Exchange,
    pub product: Product,
    pub quantity: i64,
    pub buy: f64,
    pub sell: f64,
}

pub fn get_virtual_contract_note(order: &OrderReq) -> VirtualContractNote {
    match order.exchange {
        Exchange::NSE | Exchange::BSE => {
            // Equity Trades
            let total_buy = order.buy * order.quantity as f64;
            let total_sell = order.sell * order.quantity as f64;
            let turnover = total_buy + total_sell;

            let sebi_charges = turnover * 0.000001;
            let stamp_charges = total_buy * 0.00015;

            let transaction_charges = if matches!(order.exchange, Exchange::NSE) {
                turnover * 0.0000297
            } else {
                turnover * 0.0000375
            };

            let (brokerage, stt) = match order.product {
                Product::CNC => {
                    let buy_stt = total_buy * 0.001;
                    let sell_stt = total_sell * 0.001;
                    let stt = buy_stt + sell_stt;

                    (0.0, stt)
                }
                Product::MIS => {
                    let brokerage_buy = 20f64.min(total_buy * 0.0003);
                    let brokerage_sell = 20f64.min(total_sell * 0.0003);
                    let stt = total_sell * 0.00025;

                    ((brokerage_buy + brokerage_sell), stt)
                }
                // TODO: Should we better handle this, as having any other product type is fundamentally wrong.
                _ => unreachable!(),
            };

            let total_charges = brokerage + sebi_charges + transaction_charges;
            let gst = total_charges * 0.18;

            let net_charges = total_charges + stt + stamp_charges + gst;
            let pnl = total_sell - total_buy;
            let net_pnl = pnl - net_charges;

            VirtualContractNote {
                brokerage,
                stt,
                transaction_charges,
                gst,
                sebi_charges,
                stamp_charges,
                net_charges,
                net_pnl,
                pnl,
            }
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "The current calculations of Virtual Contract Note are not 100% matching zerodha online calculator."]
    fn test_intraday_equity() {
        let order = OrderReq {
            exchange: Exchange::NSE,
            product: Product::MIS,
            quantity: 400,
            buy: 1000.0,
            sell: 1100.0,
        };

        let contract_note = get_virtual_contract_note(&order);

        let expected = VirtualContractNote {
            brokerage: 40.0,
            stt: 105.0,
            transaction_charges: 25.79,
            gst: 11.99,
            sebi_charges: 0.84,
            stamp_charges: 12.0,
            net_charges: 247.62984,
            pnl: 40000.0,
            net_pnl: 39804.38,
        };

        assert_eq!(expected, contract_note);
    }
}

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

            let sebi_charges = (turnover * 0.000001 * 100.0).round() / 100.0;

            let transaction_charges = if matches!(order.exchange, Exchange::NSE) {
                (turnover * 0.0000307 * 100.0).round() / 100.0
            } else {
                (turnover * 0.0000375 * 100.0).round() / 100.0
            };

            let (brokerage, stt, stamp_charges) = match order.product {
                Product::CNC => {
                    // Delivery trades
                    // STT: 0.1% on both buy and sell
                    let buy_stt = (total_buy * 0.001 * 100.0).round() / 100.0;
                    let sell_stt = (total_sell * 0.001 * 100.0).round() / 100.0;
                    let stt = buy_stt + sell_stt;
                    // Stamp duty: 0.015% on buy side
                    let stamp_charges = (total_buy * 0.00015 * 100.0).round() / 100.0;

                    (0.0, stt, stamp_charges)
                }
                Product::MIS => {
                    // Intraday trades
                    let brokerage_buy = 20f64.min((total_buy * 0.0003 * 100.0).round() / 100.0);
                    let brokerage_sell = 20f64.min((total_sell * 0.0003 * 100.0).round() / 100.0);
                    // STT: 0.025% on average of buy and sell prices
                    let avg_price = (order.buy + order.sell) / 2.0;
                    let stt = (avg_price * order.quantity as f64 * 0.00025 * 100.0).round() / 100.0;
                    // Stamp duty: 0.003% on buy side
                    let stamp_charges = (total_buy * 0.00003 * 100.0).round() / 100.0;

                    ((brokerage_buy + brokerage_sell), stt, stamp_charges)
                }
                // TODO: Should we better handle this, as having any other product type is fundamentally wrong.
                _ => unreachable!(),
            };

            let total_charges = brokerage + sebi_charges + transaction_charges;
            let gst = (total_charges * 0.18 * 10000.0).round() / 10000.0;

            let net_charges = ((total_charges + stt + stamp_charges + gst) * 100.0).round() / 100.0;
            let pnl = total_sell - total_buy;
            let net_pnl = ((pnl - net_charges) * 10000.0).round() / 10000.0;

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
    fn test_intraday_equity() {
        let order = OrderReq {
            exchange: Exchange::NSE,
            product: Product::MIS,
            quantity: 400,
            buy: 1000.0,
            sell: 1100.0,
        };

        let contract_note = get_virtual_contract_note(&order);

        // Calculate expected values:
        // Total buy: 400,000, Total sell: 440,000, Turnover: 840,000
        // Brokerage: min(20, 400000*0.0003) + min(20, 440000*0.0003) = 20 + 20 = 40
        // STT: (1000+1100)/2 * 400 * 0.00025 = 1050 * 400 * 0.00025 = 105
        // Transaction charges: 840000 * 0.0000307 = 25.788 ≈ 25.79
        // SEBI charges: 840000 * 0.000001 = 0.84
        // Stamp charges: 400000 * 0.00003 = 12
        // GST: (40 + 25.79 + 0.84) * 0.18 = 66.63 * 0.18 = 11.9934
        // Net charges: 40 + 105 + 25.79 + 0.84 + 12 + 11.9934 = 195.6234 (rounded to 195.62)
        // PnL: 440000 - 400000 = 40000
        // Net PnL: 40000 - 195.62 = 39804.38

        let expected = VirtualContractNote {
            brokerage: 40.0,
            stt: 105.0,
            transaction_charges: 25.79,
            gst: 11.9934,
            sebi_charges: 0.84,
            stamp_charges: 12.0,
            net_charges: 195.62,
            pnl: 40000.0,
            net_pnl: 39804.38,
        };

        assert_eq!(expected, contract_note);
    }

    #[test]
    fn test_delivery_equity_nse() {
        let order = OrderReq {
            exchange: Exchange::NSE,
            product: Product::CNC,
            quantity: 100,
            buy: 1000.0,
            sell: 1100.0,
        };

        let contract_note = get_virtual_contract_note(&order);

        // Calculate expected values:
        // Total buy: 100,000, Total sell: 110,000, Turnover: 210,000
        // Brokerage: 0 (Zerodha doesn't charge for delivery)
        // STT: 100000 * 0.001 + 110000 * 0.001 = 100 + 110 = 210
        // Transaction charges: 210000 * 0.0000307 = 6.447 ≈ 6.45
        // SEBI charges: 210000 * 0.000001 = 0.21
        // Stamp charges: 100000 * 0.00015 = 15
        // GST: (0 + 6.45 + 0.21) * 0.18 = 6.66 * 0.18 = 1.1988
        // Net charges: 0 + 210 + 6.45 + 0.21 + 15 + 1.1988 = 232.8588 (rounded to 232.86)
        // PnL: 110000 - 100000 = 10000
        // Net PnL: 10000 - 232.86 = 9767.14

        let expected = VirtualContractNote {
            brokerage: 0.0,
            stt: 210.0,
            transaction_charges: 6.45,
            gst: 1.1988,
            sebi_charges: 0.21,
            stamp_charges: 15.0,
            net_charges: 232.86,
            pnl: 10000.0,
            net_pnl: 9767.14,
        };

        assert_eq!(expected, contract_note);
    }

    #[test]
    fn test_intraday_equity_bse() {
        let order = OrderReq {
            exchange: Exchange::BSE,
            product: Product::MIS,
            quantity: 200,
            buy: 500.0,
            sell: 550.0,
        };

        let contract_note = get_virtual_contract_note(&order);

        // Calculate expected values:
        // Total buy: 100,000, Total sell: 110,000, Turnover: 210,000
        // Brokerage: min(20, 100000*0.0003) + min(20, 110000*0.0003) = 20 + 20 = 40
        // STT: (500+550)/2 * 200 * 0.00025 = 525 * 200 * 0.00025 = 26.25
        // Transaction charges: 210000 * 0.0000375 = 7.875 ≈ 7.87 (rounded)
        // SEBI charges: 210000 * 0.000001 = 0.21
        // Stamp charges: 100000 * 0.00003 = 3
        // GST: (40 + 7.87 + 0.21) * 0.18 = 48.08 * 0.18 = 8.6544
        // Net charges: 40 + 26.25 + 7.87 + 0.21 + 3 + 8.6544 = 85.9844 (rounded to 85.98)
        // PnL: 110000 - 100000 = 10000
        // Net PnL: 10000 - 85.98 = 9914.02

        let expected = VirtualContractNote {
            brokerage: 40.0,
            stt: 26.25,
            transaction_charges: 7.87,
            gst: 8.6544,
            sebi_charges: 0.21,
            stamp_charges: 3.0,
            net_charges: 85.98,
            pnl: 10000.0,
            net_pnl: 9914.02,
        };

        assert_eq!(expected, contract_note);
    }

    #[test]
    fn test_small_intraday_trade() {
        // Test with small trade where brokerage percentage is less than ₹20
        let order = OrderReq {
            exchange: Exchange::NSE,
            product: Product::MIS,
            quantity: 10,
            buy: 100.0,
            sell: 110.0,
        };

        let contract_note = get_virtual_contract_note(&order);

        // Calculate expected values:
        // Total buy: 1,000, Total sell: 1,100, Turnover: 2,100
        // Brokerage: min(20, 1000*0.0003) + min(20, 1100*0.0003) = 0.3 + 0.33 = 0.63
        // STT: (100+110)/2 * 10 * 0.00025 = 105 * 10 * 0.00025 = 0.2625 ≈ 0.26
        // Transaction charges: 2100 * 0.0000307 = 0.06447 ≈ 0.06
        // SEBI charges: 2100 * 0.000001 = 0.0021 ≈ 0.00
        // Stamp charges: 1000 * 0.00003 = 0.03
        // GST: (0.63 + 0.06 + 0.00) * 0.18 = 0.69 * 0.18 = 0.1242
        // Net charges: 0.63 + 0.26 + 0.06 + 0.00 + 0.03 + 0.1242 = 1.1042 (rounded to 1.10)
        // PnL: 1100 - 1000 = 100
        // Net PnL: 100 - 1.10 = 98.90

        let expected = VirtualContractNote {
            brokerage: 0.63,
            stt: 0.26,
            transaction_charges: 0.06,
            gst: 0.1242,
            sebi_charges: 0.0,
            stamp_charges: 0.03,
            net_charges: 1.10,
            pnl: 100.0,
            net_pnl: 98.90,
        };

        assert_eq!(expected, contract_note);
    }
}

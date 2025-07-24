use crate::user::*;

pub const USER_PROFILE_ENDPOINT: &str = "https://api.kite.trade/user/profile";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UserProfile {
    /// The unique, permanent user id registered with the broker and the exchanges
    pub user_id: String,
    /// User's real name
    pub user_name: String,
    /// Shortened version of the user's real name
    pub user_shortname: String,
    /// User's email
    pub email: String,
    /// User's registered role at the broker. This will be `individual` for all retail users
    pub user_type: String,
    /// The broker ID
    pub broker: String,
    /// Exchanges enabled for trading on the user's account
    pub exchanges: Vec<Exchange>,
    /// Margin product types enabled for the user
    pub products: Vec<Product>,
    /// Order types enabled for the user
    pub order_types: Vec<OrderType>,
    /// empty, consent or physical
    pub meta: UserMetaData,
    /// Full URL to the user's avatar (PNG image) if there's one
    #[serde(deserialize_with = "crate::utils::deserialize_nullable_string")]
    pub avatar_url: String,
}

impl KiteConnect<Authenticated> {
    pub async fn get_user_profile(&self) -> Result<Response<UserProfile>, Error> {
        Ok(self
            .client
            .get(USER_PROFILE_ENDPOINT)
            .send()
            .await?
            .json()
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_profile() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
          "status": "success",
          "data": {
            "user_id": "AB1234",
            "user_type": "individual",
            "email": "xxxyyy@gmail.com",
            "user_name": "AxAx Bxx",
            "user_shortname": "AxAx",
            "broker": "ZERODHA",
            "exchanges": [
              "BFO",
              "MCX",
              "NSE",
              "CDS",
              "BSE",
              "BCD",
              "MF",
              "NFO"
            ],
            "products": [
              "CNC",
              "NRML",
              "MIS",
              "BO",
              "CO"
            ],
            "order_types": [
              "MARKET",
              "LIMIT",
              "SL",
              "SL-M"
            ],
            "avatar_url": null,
            "meta": {
              "demat_consent": "physical"
            }
          }
        }"#;

        let expected = UserProfile {
            user_id: "AB1234".into(),
            user_type: "individual".into(),
            email: "xxxyyy@gmail.com".into(),
            user_name: "AxAx Bxx".into(),
            user_shortname: "AxAx".into(),
            broker: "ZERODHA".into(),
            exchanges: vec![
                Exchange::BFO,
                Exchange::MCX,
                Exchange::NSE,
                Exchange::CDS,
                Exchange::BSE,
                Exchange::BCD,
                Exchange::MF,
                Exchange::NFO,
            ],
            products: vec![
                Product::CNC,
                Product::NRML,
                Product::MIS,
                Product::BO,
                Product::CO,
            ],
            order_types: vec![
                OrderType::Market,
                OrderType::Limit,
                OrderType::SL,
                OrderType::SL_M,
            ],
            avatar_url: String::new(),
            meta: UserMetaData {
                demat_consent: DematConsent::Physical,
            },
        };

        let value: Response<_> = serde_json::from_str(json)?;
        assert_eq!(value, Response::Success { data: expected });

        Ok(())
    }
}

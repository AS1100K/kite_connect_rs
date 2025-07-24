use super::*;

pub const SESSION_TOKEN_ENDPOINT: &str = "https://api.kite.trade/session/token";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SessionToken {
    /// The unique, permanent user id registered with the broker and the exchanges
    pub user_id: String,
    /// User's real name
    pub user_name: String,
    /// Shortened version of the user's real name
    pub user_shortname: String,
    /// User's email
    pub email: String,
    /// User's registered role at the broker. This will be `individual` for all retail users
    // TODO: Use enum's
    pub user_type: String,
    /// The broker ID
    pub broker: String,
    /// Exchanges enabled for trading on the user's account
    pub exchanges: Vec<Exchange>,
    /// Margin product types enabled for the user
    pub products: Vec<Product>,
    /// Order types enabled for the user
    pub order_types: Vec<OrderType>,
    /// The API key for which the authentication was performed
    pub api_key: String,
    /// The authentication token that's used with every subsequent request Unless this is invalidated using the API,
    /// or invalidated by a master-logout from the Kite Web trading terminal, it'll expire at 6 AM on the next
    /// day (regulatory requirement)
    pub access_token: String,
    /// A token for public session validation where requests may be exposed to the public
    pub public_token: String,
    /// A token for getting long standing read permissions. This is only available to certain approved platforms
    pub refresh_token: String,
    /// User's last login time
    pub login_time: String,
    /// empty, consent or physical
    pub meta: Meta,
    /// Full URL to the user's avatar (PNG image) if there's one
    #[serde(deserialize_with = "crate::utils::deserialize_nullable_string")]
    pub avatar_url: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Meta {
    pub demat_consent: DematConsent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DematConsent {
    #[default]
    Empty,
    Consent,
    Physical,
}

impl KiteConnect<AuthPending> {
    pub async fn generate_session_token(
        &self,
        request_token: &str,
    ) -> Result<Response<SessionToken>, Error> {
        #[derive(Serialize)]
        struct SessionTokenRequest<'a> {
            api_key: &'a str,
            request_token: &'a str,
            checksum: &'a str,
        }

        let checksum = sha2::Sha256::digest(format!(
            "{}{}{}",
            self.api_key(),
            request_token,
            self.auth_info.api_secret()
        ));
        let checksum_hex = format!("{checksum:x}");

        let req = SessionTokenRequest {
            api_key: self.auth_info.api_key(),
            request_token,
            checksum: &checksum_hex,
        };

        Ok(self
            .client
            .post(SESSION_TOKEN_ENDPOINT)
            .form(&req)
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
    fn test_session_token() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "status": "success",
            "data": {
                "user_type": "individual",
                "email": "XXXXXX",
                "user_name": "Kite Connect",
                "user_shortname": "Connect",
                "broker": "ZERODHA",
                "exchanges": [
                    "NSE",
                    "NFO",
                    "BFO",
                    "CDS",
                    "BSE",
                    "MCX",
                    "BCD",
                    "MF"
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
                "avatar_url": "abc",
                "user_id": "XX0000",
                "api_key": "XXXXXX",
                "access_token": "XXXXXX",
                "public_token": "XXXXXXXX",
                "enctoken": "XXXXXX",
                "refresh_token": "",
                "silo": "",
                "login_time": "2021-01-01 16:15:14",
                "meta": {
                    "demat_consent": "physical"
                }
            }
        }"#;

        let expected = SessionToken {
            user_id: "XX0000".into(),
            user_name: "Kite Connect".into(),
            user_shortname: "Connect".into(),
            email: "XXXXXX".into(),
            user_type: "individual".into(),
            broker: "ZERODHA".into(),
            exchanges: vec![
                Exchange::NSE,
                Exchange::NFO,
                Exchange::BFO,
                Exchange::CDS,
                Exchange::BSE,
                Exchange::MCX,
                Exchange::BCD,
                Exchange::MF,
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
            api_key: "XXXXXX".into(),
            access_token: "XXXXXX".into(),
            public_token: "XXXXXXXX".into(),
            refresh_token: "".into(),
            login_time: "2021-01-01 16:15:14".into(),
            meta: Meta {
                demat_consent: DematConsent::Physical,
            },
            avatar_url: "abc".into(),
        };

        let value: Response<SessionToken> = serde_json::from_str(json)?;
        assert_eq!(value, Response::Success { data: expected });

        Ok(())
    }
}

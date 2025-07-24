use reqwest::Url;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::{Authenticated, KiteConnect, error::Error, user::LOGIN_ENDPOINT};

/// A helper struct for handling one-time, interactive authentication flows with Kite Connect.
///
/// # Note
/// This implementation is not optimized for speed or concurrent/multiple authentications.
/// It is intended for single-use, manual authentication flows (e.g., CLI tools or setup scripts).
pub struct AutoAuth {
    /// The port to listen on for the authentication callback.
    port: u16,
    /// The API key for Kite Connect.
    api_key: String,
    /// The API secret for Kite Connect.
    api_secret: String,
}

impl AutoAuth {
    /// Creates a new [`AutoAuth`] instance with the given API key and secret.
    ///
    /// The default port is set to 8000.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key for Kite Connect.
    /// * `api_secret` - The API secret for Kite Connect.
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            port: 8000,
            api_key,
            api_secret,
        }
    }

    /// Sets the port to listen on for the authentication callback.
    ///
    /// # Arguments
    ///
    /// * `port` - The port number to use.
    pub fn set_port(&mut self, port: u16) {
        self.port = port
    }

    /// Performs the authentication flow by listening for a single HTTP GET request containing the request token.
    ///
    /// This method starts a local TCP listener and waits for a single authentication callback.
    /// It is **not** optimized for speed or concurrent/multiple authentications, and is intended for single-use flows.
    ///
    /// # Returns
    ///
    /// * `Ok(KiteConnect<Authenticated>)` if authentication is successful.
    /// * `Err(Error)` if an error occurs during the process.
    ///
    /// # Note
    ///
    /// - Only the first valid authentication request will be processed.
    /// - The implementation is simple and suitable for CLI tools or setup scripts, not for production servers.
    pub async fn authenticate(self) -> Result<KiteConnect<Authenticated>, Error> {
        println!("Go Ahead and authenticate yourself at:");
        println!("{LOGIN_ENDPOINT}{}", self.api_key);

        let listener = TcpListener::bind(format!("localhost:{}", self.port)).await?;
        let mut buffer = [0u8; 150]; // Required 90, Have length 150 just to be safe
        let request_token;

        loop {
            let (mut stream, _) = listener.accept().await?;
            let n = stream.read(&mut buffer).await?;

            if n < 3 {
                continue;
            }

            let request = String::from_utf8_lossy(&buffer[..n]);
            let mut chunk = request.split_whitespace();

            match chunk.next() {
                Some(protocol) => {
                    if protocol != "GET" {
                        let _ = stream
                            .write(format!("Unsupported Protocol. Only GET Method is allowed, You are using {protocol}").as_bytes())
                            .await;
                    }
                }
                None => continue,
            }

            if let Some(path) = chunk.next() {
                let url = format!("http://localhost{path}");
                if let Ok(parsed_url) = Url::parse(&url) {
                    let Some(token) = parsed_url.query_pairs().find_map(|(k, v)| {
                        if k == "request_token" {
                            return Some(v);
                        }

                        None
                    }) else {
                        continue;
                    };

                    let _ = stream
                        .write("Authenticated Successfully. Got the Request Token".as_bytes())
                        .await;

                    request_token = token.to_string();
                    break;
                }
            }
        }

        let kc = KiteConnect::new(self.api_key, self.api_secret);
        let kc = kc.authenticate_with_request_token(&request_token).await?;

        Ok(kc)
    }
}

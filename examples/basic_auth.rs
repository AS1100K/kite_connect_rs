use std::env;

use kite_connect::AutoAuth;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let api_key = env::var("API_KEY").unwrap_or_default();
    let api_secret = env::var("API_SECRET").unwrap_or_default();

    let auto_auth = AutoAuth::new(api_key, api_secret);
    let kc = auto_auth.authenticate().await.unwrap();

    let access_token = kc.access_token();

    println!("Access Token: {access_token}");
    println!("🤫 Keep it safe.")
}

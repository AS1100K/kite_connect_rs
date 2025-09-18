use kite_connect::{AutoAuth, KiteConnect, ws::Req};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("API_KEY").expect("Please pass the API_KEY as env variable.");
    let api_secret = env::var("API_SECRET").expect("Please pass the API_SECRET as env variable.");
    let access_token = env::var("ACCESS_TOKEN");

    let kc = if let Ok(access_token) = access_token {
        let kc = KiteConnect::new(api_key, api_secret);
        kc.authenticate_with_access_token(access_token).unwrap()
    } else {
        let auto_auth = AutoAuth::new(api_key, api_secret);
        let kc = auto_auth.authenticate().await.unwrap();

        let access_token = kc.access_token();
        println!("Access Token: {access_token}");
        println!("🤫 Keep it safe.");

        kc
    };

    let (mut kt, rx) = kc
        .web_socket()
        .await
        .expect("Failed to create WebSocket Connection");

    // Subscribe to COROMANDEL, TATAMOTORS, INFY, NIFTY 50
    kt.send(Req::Subscribe(&[189185, 884737, 408065, 256265]))
        .await
        .unwrap();

    kt.send_raw(tokio_tungstenite::tungstenite::Message::Text(
        serde_json::json!({
            "a": "mode",
            "v": ["ltp", [189185, 884737], "quote", [408065]]
        })
        .to_string()
        .into(),
    ))
    .await
    .unwrap();

    while let Ok(packet) = rx.recv() {
        println!("{:?}", packet);
    }
}

use std::env;

use kite_connect::{AutoAuth, KiteConnect};

#[tokio::main(flavor = "current_thread")]
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

    let user_profile = kc.get_user_profile().await.unwrap();
    println!("User Profile: {user_profile:#?}");
}

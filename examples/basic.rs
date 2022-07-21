use clap::Parser;
use std::env;
use teslatte::auth::{AccessToken, Authentication};
use teslatte::vehicles::SetChargeLimit;
use teslatte::Api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let access_token = match env::var("TESLA_ACCESS_TOKEN") {
        Ok(t) => AccessToken(t),
        Err(_) => {
            let auth = Authentication::new().unwrap();
            let (access_token, refresh_token) = auth.interactive_get_access_token().await.unwrap();
            println!("Access token: {}", access_token.0);
            println!("Refresh token: {}", refresh_token.0);
            access_token
        }
    };

    let api = Api::new(&access_token);

    let vehicles = api.vehicles().await.unwrap();
    dbg!(&vehicles);

    if vehicles.len() > 0 {
        let vehicle_data = api.vehicle_data(&vehicles[0].id).await.unwrap();
        dbg!(vehicle_data);

        let charge_state = api.charge_state(&vehicles[0].id).await.unwrap();
        dbg!(&charge_state);
    } else {
        println!("No vehicles found!");
    }
}

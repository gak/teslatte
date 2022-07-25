use clap::Parser;
use std::env;
use teslatte::auth::AccessToken;
use teslatte::vehicles::SetChargeLimit;
use teslatte::Api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let api = match env::var("TESLA_ACCESS_TOKEN") {
        Ok(t) => Api::new(AccessToken(t), None),
        Err(_) => Api::from_interactive_url().await.unwrap(),
    };

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

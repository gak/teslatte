use std::env;
use teslatte::auth::AccessToken;
use teslatte::Api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let api = match env::var("TESLA_ACCESS_TOKEN") {
        Ok(t) => Api::new(AccessToken(t), None),
        Err(_) => {
            let api = Api::from_interactive_url().await.unwrap();
            println!("TOKEN: {:?}", api.access_token);
            api
        }
    };

    let vehicles = api.vehicles().await.unwrap();
    dbg!(&vehicles);

    if !vehicles.is_empty() {
        let vehicle_data = api.vehicle_data(&vehicles[0].id).await.unwrap();
        dbg!(vehicle_data);
    } else {
        println!("No vehicles found!");
    }
}

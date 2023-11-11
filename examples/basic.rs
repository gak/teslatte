use std::env;
use teslatte::auth::AccessToken;
use teslatte::products::Product;
use teslatte::vehicles::GetVehicleData;
use teslatte::{OwnerApi, VehicleApi};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let api = match env::var("TESLA_ACCESS_TOKEN") {
        Ok(t) => OwnerApi::new(AccessToken(t), None),
        Err(_) => {
            let api = OwnerApi::from_interactive_url().await.unwrap();
            println!("TOKEN: {:?}", api.access_token);
            api
        }
    };

    let vehicles = api.vehicles().await.unwrap();
    dbg!(&*vehicles);

    if !vehicles.is_empty() {
        let get_vehicle_data = GetVehicleData::new(vehicles[0].id.clone());
        let vehicle_data = api.vehicle_data(&get_vehicle_data).await.unwrap();
        dbg!(&vehicle_data);
    } else {
        println!("No vehicles found!");
    }

    let products = api.products().await.unwrap();
    dbg!(&*products);

    if !products.is_empty() {
        for product in &*products {
            match product {
                Product::Vehicle(v) => {
                    dbg!(v);
                }

                Product::Solar(e) => {
                    let site_info = api.energy_sites_site_info(&e.energy_site_id).await.unwrap();
                    dbg!(&site_info);

                    let live_info = api
                        .energy_sites_live_status(&e.energy_site_id)
                        .await
                        .unwrap();
                    dbg!(&live_info);
                }

                Product::Powerwall(p) => {
                    let live_info = api
                        .energy_sites_live_status(&p.energy_site_id)
                        .await
                        .unwrap();
                    dbg!(&live_info);
                }
            }
        }
    } else {
        println!("No products found!");
    }
}

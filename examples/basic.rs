use teslatte::auth::Authentication;
use teslatte::Api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let api = Authentication::new().unwrap();
    let (access_token, refresh_token) = api.interactive_get_access_token().await.unwrap();
    println!("Access token: {}", access_token.0);
    println!("Refresh token: {}", refresh_token.0);

    let api = Api::new(&access_token);

    let vehicles = api.vehicles().await.unwrap();
    dbg!(&vehicles);

    let charge_state = api.charge_state(&vehicles[0].id).await.unwrap();
    dbg!(&charge_state);
}

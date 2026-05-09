mod data_models;
mod human;
mod machine;
mod settings;
use human::client;

// Main
#[tokio::main]
async fn main() -> () {
    let api_key = human::client::get_api_key_env();

    client::run(api_key).await;
}

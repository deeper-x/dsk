mod data_models;
mod human;
mod machine;
mod settings;

use human::client;

// Main
#[tokio::main]
async fn main() {
    client::print_motd();

    client::run().await;
}

mod human;
mod machine;
mod mytypes;

use human::client;

// Main
#[tokio::main]
async fn main() {
    client::print_motd();
    client::run().await;
}

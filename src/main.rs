mod data_models;
mod human;
mod machine;
mod settings;

use human::client;

// Main
#[tokio::main]
async fn main() {
    let api_key = human::client::get_api_key_env();

    match api_key {
        Ok(k) => client::run(k).await,
        Err(e) => {
            eprint!(
                "Error {}: {} environment variable not set.\n",
                e,
                settings::api::DEEPSEEK_API_KEY
            );
            std::process::exit(1);
        }
    }
}

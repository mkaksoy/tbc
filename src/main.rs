mod utils;
mod errors;
mod core;
mod enums;

use std::env;

#[tokio::main]
async fn main() {
    let console = core::console::Console::new();

    let mode = env::args().collect::<Vec<String>>()[1].clone();

    if mode == "h" {
        core::network::host::create(8080, String::from("cert")).await.unwrap();
    } else if mode == "c" {
        core::network::client::join(String::from("0.0.0.0"), 8080, String::from("cert")).await.unwrap();
    }
}

use std::error::Error;
use serde::de::Error;
use serde::ser::Error;
use reqwest::Client;
use boltz_api::{create_swap, Swap};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Replace with a valid Bitcoin or Lightning address
    let address = "your_address_here";
    let currency = "BTC";

    let swap = create_swap(&client, address, currency).await?;

    println!("Swap created: {:#?}", swap);

    Ok(())
}

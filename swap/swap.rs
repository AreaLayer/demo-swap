use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct SwapRequest {
    address: String,
    currency: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SwapResponse {
    id: String,
    timeout_block_height: u64,
    on_chain_address: String,
    lockup_address: String,
    redeem_script: String,
}

async fn create_swap(
    client: &Client,
    address: &str,
    currency: &str,
) -> Result<SwapResponse, Box<dyn Error>> {
    let swap_request = SwapRequest {
        address: address.to_string(),
        currency: currency.to_string(),
    };

    let res = client
        .post("https://api.boltz.exchange/api/swap")
        .json(&swap_request)
        .send()
        .await?;

    let swap_response = res.json::<SwapResponse>().await?;
    Ok(swap_response)
}
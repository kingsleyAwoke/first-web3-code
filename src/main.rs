use ethers::prelude::*;
use std::convert::TryFrom;
use std::error::Error;
use dotenv::dotenv; // Import dotenv
use std::env; // Import the env module

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok(); // Load the .env file

    // Retrieve the Infura API key from the environment variable
    let infura_api_key = env::var("INFURA_API_KEY")?;
    let provider_url = format!("https://mainnet.infura.io/v3/{}", infura_api_key);
    
    let provider = Provider::<Http>::try_from(provider_url)?;
    let block_number = provider.get_block_number().await?;
    
    println!("Latest block number: {:?}", block_number);
    Ok(())
}
use std::time::Duration;

use ethers::{
    prelude::{ Address, LocalWallet, Middleware, Provider, Signer, TransactionRequest, U256 },
    utils::Ganache,
};
use eyre::{ContextCompat, Result};
use hex::ToHex;


use std::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
//Create and print ganache endpoint
    
    let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artefact";
    // Spawns a local blockchain in background, using the mnemonic passed to generate a wallet
    let ganache = Ganache::new().mnemonic(mnemonic).spawn();
    println!("HTTP endpoint: {}", ganache.endpoint());

//Get wallet and print address
    let wallet: LocalWallet = ganache.keys()[0].clone().into();
    let first_address = wallet.address();
    println!(
        "Wallet first address: {}",
        first_address.encode_hex::<String>()
    );
//Get wallet balance and print it
    let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));
    
    let first_balance = provider.get_balance(first_address, None).await?;
    println!("Wallet first address balance: {}", first_balance);

//Get balance of another wallet in eth
    let other_address_hex = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646";
    let other_address = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646".parse::<Address>()?;
    let other_balance = provider.get_balance(other_address, None).await?;
    
    println!(
        "Balance of address {} is: {}",
        other_address_hex, other_balance
    );

//Make a transaction from a wallet to another
//    let tx = TransactionRequest::pay(other_address, U256::)

    return Result::Ok(());
}
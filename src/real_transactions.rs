use std::time::Duration;

use ethers::{
    prelude::{ Address, LocalWallet, Middleware, Provider, Signer, TransactionRequest, U256 },
    utils::Ganache,
};
use eyre::{ContextCompat, Result};
use hex::ToHex;



#[tokio::main]
async fn main() -> Result<()> {
//Create and print ganache endpoint
    
    let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artefact";
    // Spawns a local blockchain in background, using the mnemonic passed to generate a wallet
    let ganache = Ganache::new().mnemonic(mnemonic).spawn();
    println!("HTTP endpoint: {}", ganache.endpoint());

//Get the local wallet and print address
    let wallet: LocalWallet = ganache.keys()[0].clone().into();
    let first_address = wallet.address();
    println!(
        "Wallet first address: {}",
        first_address.encode_hex::<String>()
    );
//Get wallet balance and print it
//The provider connects to the ganache endpoint
    let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));
    
    let first_balance = provider.get_balance(first_address, None).await?;
    println!("Wallet first address balance: {}", first_balance);

//Get balance of a random wallet in eth, it doesnt exist but it gives its balance anyway, this address is new
    let other_address_hex = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646";
    let other_address = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646".parse::<Address>()?;
    let other_balance = provider.get_balance(other_address, None).await?;
    
    println!(
        "Balance of address {} is: {}",
        other_address_hex, other_balance
    );

//Make a transaction from a wallet to another
    let tx = TransactionRequest::pay(other_address, U256::from(1000u64)).from(first_address);

    let receipt = provider
        .send_transaction(tx, None)
        .await?
        .log_msg("Pending transfer")
        .confirmations(1)
        .await?
        .context("Missing receipt")?;

    println!(
        "TX mined in block {}",
        receipt.block_number.context("cannot get block number")?
    );
    println!(
        "Balance of {} is {}",
        other_address_hex,
        provider.get_balance(other_address, None).await?
    );

    println!(
        "Remain balance of local wallet is {}",
        provider.get_balance(first_address, None).await?
    );

    return Ok(())
}
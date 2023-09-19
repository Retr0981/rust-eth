use std::time::Duration;


user ethers::{
    prelude::{Address, LocalWallet, Middleware, Provider,Signer, TransactionRequest, U256},
    utils::Ganache,
    };
use eyre::{ContextCompat, Result};
use hex::ToHex;


#[tokio::main]
async fn main() -> Result<()> {
    let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artefact";
    let ganache = Ganache::new().mnemonic(mnemonic).spawn();
    println!("HTTP Endpoint: {}", ganache.endpoint());


    let wallet: LocalWallet = ganache.keys()[0].clone.into();
    

}
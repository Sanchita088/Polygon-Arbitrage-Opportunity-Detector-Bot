use ethers::contract::abigen;
use ethers::providers::{Provider, Http};
use ethers::types::{Address, U256};
use std::sync::Arc;
use anyhow::Result;

// Step 4 me abigen! macro se contract bindings banayenge
abigen!(
    UniswapV2Router,
    r#"[
        function getAmountsOut(uint amountIn, address[] memory path) external view returns (uint[] memory amounts)
    ]"#
);

#[tokio::main]
async fn main() -> Result<()> {
    // Step 5: Polygon RPC provider banayein
    let provider = Provider::<Http>::try_from("https://polygon-rpc.com")?;
    let provider = Arc::new(provider);

    // Step 6: Uniswap V2 Router contract address (QuickSwap ka example)
    let router_address: Address = "0x1b02da8cb0d097eb8d57a175b88c7d8b47997506".parse()?;

    // Step 7: Contract instance banayein
    let router = UniswapV2Router::new(router_address, provider.clone());

    // Step 8: Input parameters define karein
    // 1 WETH (18 decimals)
    let amount_in = U256::exp10(18);

    // Token addresses (Polygon network)
    let weth: Address = "0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619".parse()?;
    let usdc: Address = "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174".parse()?;

    // Path: WETH -> USDC
    let path = vec![weth, usdc];

    // Step 9: getAmountsOut function call karein
    let amounts = router.get_amounts_out(amount_in, path).call().await?;

    // Step 10: Result print karein
    println!("Amounts out: {:?}", amounts);

    Ok(())
}
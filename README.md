# Polygon Arbitrage Opportunity Detector Bot

## Overview

This Rust-based bot detects potential arbitrage opportunities for token pairs (e.g., WETH/USDC) on the Polygon network by comparing prices across two decentralized exchanges (DEXes) such as QuickSwap and SushiSwap.

It periodically fetches token prices from the DEXes, identifies profitable price differences exceeding a configured threshold, and logs simulated arbitrage profits after accounting for simplified gas costs.

---

## Features

- Connects to Polygon RPC endpoint
- Fetches token pair prices from two DEXes using Uniswap V2 Router ABI
- Detects arbitrage opportunities based on configurable minimum profit threshold
- Calculates simulated profit for a fixed trade size considering gas costs
- Logs detected opportunities to a SQLite database
- Configurable via `config.toml`

---

## Getting Started

### Prerequisites

- Rust (1.65 or later recommended)
- Cargo package manager
- Internet connection to access Polygon RPC endpoint

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/polygon-arbitrage-bot.git
   cd polygon-arbitrage-bot

2. Configure the bot by editing config.toml:
   ```bash
   polygon_rpc_url = "https://polygon-rpc.com"  

    [uniswap]
    router_address = "0x1b02da8cb0d097eb8d57a175b88c7d8b47997506" # QuickSwap Router
    
    [sushiswap]
    router_address = "0x1b02da8cb0d097eb8d57a175b88c7d8b47997506" # Example address
    
    [tokens]
    weth = "0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619"
    usdc = "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174"
    
    min_profit_usdc = 1.0
    trade_size_weth = 1.0
3. Build and run the bot:
   ```bah
   cargo run --release
### How It Works:
1. The bot connects to the Polygon network via the configured RPC URL.
2. It queries the getAmountsOut function on the Uniswap V2 Router contracts of two DEXes to get the output amount of USDC for a fixed input amount of WETH.
3. It compares the prices from both DEXes to find if buying on one and selling on the other yields a profit above the configured threshold.
4. It calculates a simulated profit by subtracting a fixed gas cost.
5. If an opportunity is found, it logs the details (timestamp, buy DEX, sell DEX, prices, profit) into a SQLite database (arbitrage_opportunities.db).
6. The bot repeats this process periodically (every 30 seconds by default).

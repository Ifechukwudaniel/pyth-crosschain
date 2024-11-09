# Pyth Stylus SDK

This package provides utilities for consuming prices from the [Pyth Network](https://pyth.network/) Oracle in Solidity. It also includes the [Pyth Interface ABI](./abis/IPyth.json), which can be used in your libraries to interact with the Pyth contract.

It is **strongly recommended** to follow the [consumer best practices](https://docs.pyth.network/documentation/pythnet-price-feeds/best-practices) when consuming data from Pyth.

## Features

- Integrates with the [`pyth-solidity-contracts`] library for external calls to Pyth smart contracts.
- Provides first-class `no_std` support.
- Includes Solidity constructors powered by [`koba`].
- Supports both [unit] and [integration] test affordances for thorough testing.

## Installation

To add the Stylus Contracts from crates.io, add the following line to your `Cargo.toml` (pinning to a specific version is recommended):

```toml
[dependencies]
pyth-stylus = "0.0.1"
```

For the latest changes from the `main` branch, you can also specify a git dependency:

```toml
[dependencies]
pyth-stylus = { git = "URL" }
```

## Example Usage

To consume prices, use the functions interface. Be sure to read the function documentation to ensure safe use of price data.

For example, to read the latest price, call [`getPriceNoOlderThan`](https://github.com/Ifechukwudaniel/pyth-crosschain/blob/stylus-sdk/target_chains/ethereum/sdk/stylus/contracts/src/pyth/functions.rs) with the Price ID of the price feed you are interested in:

```rust
use pyth_stylus::pyth::functions::get_price_no_older_than;

sol_storage! {
    #[entrypoint]
    struct FunctionCallsExample {
        address pyth_address;
        bytes32 price_id;
        StoragePrice price;
        StoragePriceFeed price_feed;
        StoragePrice ema_price;
        StoragePriceFeed ema_price_feed;
    }
}


impl FunctionCallsExample {
    pub fn get_price_no_older_than(&mut self) -> Result<(), Vec<u8>> {
       let _ = get_price_no_older_than(self, self.pyth_address.get(), self.price_id.get(), U256::from(1000))?;
       Ok(())
    }
}
```

Alternatively, you can interact directly with the Pyth contract, which implements the IPyth functions, instead of using call functions:

```rust
#![cfg_attr(not(test), no_std, no_main)]
extern crate alloc;

use stylus_sdk::prelude::{entrypoint, public, sol_storage};
use pyth_stylus::pyth::pyth_contract::PythContract;

sol_storage! {
    #[entrypoint]
    struct ProxyCallsExample {
        #[borrow]
        PythContract pyth
    }
}

#[public]
#[inherit(PythContract)]
impl ProxyCallsExample {
}
```

## Mocking Pyth

[MockPyth](./mock.rs) is a mock contract that can be deployed locally to simulate Pyth contract behavior. To set and update price feeds, call `updatePriceFeeds` and provide an array of encoded price feeds as the argument. Encoded price feeds can be created using the `create_price_feed_update_data` function in the mock contract, which is also available in the functions module.

### Releases

We use [Semantic Versioning](https://semver.org/) for our releases. To release a new version of this package and publish it to npm, follow these steps:

1. Run `npm version <new version number> --no-git-tag-version` to update the package version, then push your changes to GitHub.
2. Once the change is merged into `main`, create a release with the tag `v<new version number>`, such as `v1.5.2`. A GitHub action will automatically publish the new version of the package to npm.
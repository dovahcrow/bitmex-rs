# bitmex-rs [![Crates.io](https://img.shields.io/crates/v/bitmex.svg)](https://crates.io/crates/bitmex) [![Build Status](https://travis-ci.org/dovahcrow/bitmex-rs.png?branch=master)](https://travis-ci.org/dovahcrow/bitmex-rs) [![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)

[BitMEX](https://www.bitmex.com/app/apiOverview) (non-official) client for rust, with async/await support!

[Documentation](https://docs.rs/crate/bitmex)

## Caveat

Please run the tests before you use since BitMEX often introduces breaking changes without
changing their Swagger API definition.

Also, the Swagger definition somehow is not aligned with BitMEX's realworld API. Since bitmex-rs
auto-generates the code from swagger.json, you may experience some API breakage. Please do not 
hesitate to file an issue!

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
bitmex-rs = "0.2.0-alpha.1"
```

Examples located in the examples and tests folder.

## Implementation status

Currently all the API features are implemented, including websocket! 

# bitmex-rs [![Crates.io](https://img.shields.io/crates/v/bitmex.svg)](https://crates.io/crates/bitmex) [![Build Status](https://travis-ci.org/dovahcrow/bitmex-rs.png?branch=master)](https://travis-ci.org/dovahcrow/bitmex-rs) [![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)

[BitMEX](https://www.bitmex.com/app/apiOverview) (non-official) client for rust

[Documentation](https://docs.rs/crate/bitmex)

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
bitmex-rs = 0.1.2
```

Examples located in the examples and tests folder.

## Implementation status

If you find some features are not implemented and you just need them urgently, file an issue and I'll put that in priority. A PR is also welcome :)

- Announcement
  - [x] GET /announcement
  - [x] GET /announcement/urgent
- APIKey
  - [x] GET `/apiKey`
  - [x] POST `/apiKey`
  - [x] DELETE `/apiKey`
  - [x] POST `/apiKey/enable`
  - [x] POST `/apiKey/disable`
- Chat
  - [x] GET `/chat`
  - [x] POST `/chat`
  - [x] GET `/chat/channels`
  - [x] GET `/chat/connected`
- Execution
  - [x] GET `/execution`
  - [x] GET `/execution/tradeHistory`
- Funding
  - [x] GET `/funding`
- GlobalNotification
  - [x] GET `/globalNotification`
- Instrument
  - [x] GET `/instrument`
  - [x] GET `/instrument/active`
  - [x] GET `/instrument/activeAndIndices`
  - [x] GET `/instrument/activeIntervals`
  - [x] GET `/instrument/compositeIndex`
  - [x] GET `/instrument/indices`
- Insurance
  - [x] GET `/insurance`
- Leaderboard
  - [x] GET `/leaderboard`
  - [x] GET `/leaderboard/name`
- Liquidation
  - [x] GET `/liquidation`
- Order
  - [x] GET `/order`
  - [x] PUT `/order`
  - [x] POST `/order`
  - [x] DELETE `/order`
  - [x] DELETE `/order/all`
  - [ ] PUT `/order/bulk`
  - [ ] POST `/order/bulk`
  - [x] POST `/order/cancelAllAfter`
  - [x] POST `/order/closePosition`
- OrderBook
  - [x] GET `/orderBook/L2`
- Position
  - [x] GET `/position`
  - [x] POST `/position/isolate`
  - [x] POST `/position/leverage`
  - [x] POST `/position/riskLimit`
  - [x] POST `/position/transferMargin`
- Quote
  - [x] GET `/quote`
  - [x] GET `/quote/bucketed`
- Schema
  - [ ] GET `/schema`
  - [ ] GET `/schema/websocketHelp`
- Settlement
  - [ ] GET `/settlement`
- Stat
  - [ ] GET `/stats`
  - [ ] GET `/stats/history`
  - [ ] GET `/stats/historyUSD`
- Trade
  - [ ] GET `/trade`
  - [ ] GET `/trade/bucketed`
- User
  - [ ] GET `/user`
  - [ ] PUT `/user`
  - [ ] GET `/user/affiliateStatus`
  - [ ] POST `/user/cancelWithdrawal`
  - [ ] GET `/user/checkReferralCode`
  - [ ] GET `/user/commission`
  - [ ] POST `/user/communicationToken`
  - [ ] POST `/user/confirmEmail`
  - [ ] POST `/user/confirmEnableTFA`
  - [ ] POST `/user/confirmWithdrawal`
  - [ ] GET `/user/depositAddress`
  - [ ] POST `/user/disableTFA`
  - [ ] GET `/user/executionHistory`
  - [ ] POST `/user/logout`
  - [ ] POST `/user/logoutAll`
  - [ ] GET `/user/margin`
  - [ ] GET `/user/minWithdrawalFee`
  - [ ] POST `/user/preferences`
  - [ ] POST `/user/requestEnableTFA`
  - [ ] POST `/user/requestWithdrawal`
  - [ ] GET `/user/wallet`
  - [ ] GET `/user/walletHistory`
  - [ ] GET `/user/walletSummary`
- UserEvent
  - [ ] GET `/userEvent`
- [x] Websocket
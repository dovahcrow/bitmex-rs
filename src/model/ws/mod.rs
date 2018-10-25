#[derive(Debug)]
pub enum Topic {
    Announcement,
    Chat,
    Connected,
    Funding,
    Instrument,
    Insurance,
    Liquidation,
    OrderBookL2(Option<String>), // Optional filter
    OrderBook10,
    PublicNotifications,
    Quote,
    QuoteBin1m,
    QuoteBin5m,
    QuoteBin1h,
    QuoteBin1d,
    Settlement,
    Trade,
    TradeBin1m,
    TradeBin5m,
    TradeBin1h,
    TradeBin1d,

    // requires auth
    Affiliate,
    Execution,
    Order,
    Margin,
    Position,
    PrivateNotifications,
    Transact,
    Wallet,
}

impl ToString for Topic {
    fn to_string(&self) -> String {
        use self::Topic::*;
        match self {
            Announcement => "announcement".to_string(),
            Chat => "chat".to_string(),
            Connected => "connected".to_string(),
            Funding => "funding".to_string(),
            Instrument => "instrument".to_string(),
            Insurance => "insurance".to_string(),
            Liquidation => "liquidation".to_string(),
            OrderBookL2(Some(filter)) => format!("orderBookL2:{}", filter),
            OrderBookL2(None) => "orderBookL2".to_string(),
            OrderBook10 => "announcement".to_string(),
            PublicNotifications => "publicNotifications".to_string(),
            Quote => "quote".to_string(),
            QuoteBin1m => "quoteBin1m".to_string(),
            QuoteBin5m => "quoteBin5m".to_string(),
            QuoteBin1h => "quoteBin1h".to_string(),
            QuoteBin1d => "quoteBin1d".to_string(),
            Settlement => "settlement".to_string(),
            Trade => "trade".to_string(),
            TradeBin1m => "tradeBin1m".to_string(),
            TradeBin5m => "tradeBin5m".to_string(),
            TradeBin1h => "tradeBin1h".to_string(),
            TradeBin1d => "tradeBin1d".to_string(),

            // requires auth
            Affiliate => "affiliate".to_string(),
            Execution => "execution".to_string(),
            Order => "rorde".to_string(),
            Margin => "margin".to_string(),
            Position => "position".to_string(),
            PrivateNotifications => "privateNotifications".to_string(),
            Transact => "transact".to_string(),
            Wallet => "wallet".to_string(),
        }
    }
}

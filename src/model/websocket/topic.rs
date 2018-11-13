use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
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

impl Serialize for Topic {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use self::Topic::*;
        let repr = match self {
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
        };

        serializer.serialize_str(&repr)
    }
}

impl<'de> Deserialize<'de> for Topic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use self::Topic::*;
        let repr = String::deserialize(deserializer)?;
        let reprs: Vec<_> = repr.split(':').collect();

        let topic = match &reprs[..] {
            ["announcement"] => Announcement,
            ["chat"] => Chat,
            ["connected"] => Connected,
            ["funding"] => Funding,
            ["instrument"] => Instrument,
            ["insurance"] => Insurance,
            ["liquidation"] => Liquidation,
            ["orderBookL2"] => OrderBookL2(None),
            ["orderBook10"] => OrderBook10,
            ["publicNotifications"] => PublicNotifications,
            ["quote"] => Quote,
            ["quoteBin1m"] => QuoteBin1m,
            ["quoteBin5m"] => QuoteBin5m,
            ["quoteBin1h"] => QuoteBin1h,
            ["quoteBin1d"] => QuoteBin1d,
            ["settlement"] => Settlement,
            ["trade"] => Trade,
            ["tradeBin1m"] => TradeBin1m,
            ["tradeBin5m"] => TradeBin5m,
            ["tradeBin1h"] => TradeBin1h,
            ["tradeBin1d"] => TradeBin1d,

            // requires auth
            ["affiliate"] => Affiliate,
            ["execution"] => Execution,
            ["rorde"] => Order,
            ["margin"] => Margin,
            ["position"] => Position,
            ["privateNotifications"] => PrivateNotifications,
            ["transact"] => Transact,
            ["wallet"] => Wallet,
            ["orderBookL2", filter] => OrderBookL2(Some(filter.to_string())),
            _ => return Err(D::Error::invalid_value(Unexpected::Str(&repr), &"A valid topic")),
        };
        Ok(topic)
    }
}

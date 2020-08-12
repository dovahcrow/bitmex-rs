use crate::BitMEXError;
use fehler::throw;
use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Topic {
    Announcement,
    Chat,
    Connected,
    Funding,
    Instrument,
    Insurance,
    Liquidation,
    OrderBookL2_25(Option<String>), // Optional filter
    OrderBookL2(Option<String>),    // Optional filter
    OrderBook10(Option<String>),
    PublicNotifications,
    Quote(Option<String>),
    QuoteBin1m(Option<String>),
    QuoteBin5m(Option<String>),
    QuoteBin1h(Option<String>),
    QuoteBin1d(Option<String>),
    Settlement,
    Trade(Option<String>),
    TradeBin1m(Option<String>),
    TradeBin5m(Option<String>),
    TradeBin1h(Option<String>),
    TradeBin1d(Option<String>),

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

impl std::fmt::Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::Topic::*;
        let repr = match self {
            Announcement => "announcement".to_string(),
            Chat => "chat".to_string(),
            Connected => "connected".to_string(),
            Funding => "funding".to_string(),
            Instrument => "instrument".to_string(),
            Insurance => "insurance".to_string(),
            Liquidation => "liquidation".to_string(),

            OrderBook10(None) => "orderBook10".to_string(),
            OrderBook10(Some(filter)) => format!("orderBook10:{}", filter),
            OrderBookL2(None) => "orderBookL2".to_string(),
            OrderBookL2(Some(filter)) => format!("orderBookL2:{}", filter),
            OrderBookL2_25(None) => "orderBookL2_25".to_string(),
            OrderBookL2_25(Some(filter)) => format!("orderBookL2_25:{}", filter),

            PublicNotifications => "publicNotifications".to_string(),

            Quote(None) => "quote".to_string(),
            Quote(Some(filter)) => format!("quote:{}", filter),
            QuoteBin1m(None) => "quoteBin1m".to_string(),
            QuoteBin1m(Some(filter)) => format!("quoteBin1m:{}", filter),
            QuoteBin5m(None) => "quoteBin5m".to_string(),
            QuoteBin5m(Some(filter)) => format!("quoteBin5m:{}", filter),
            QuoteBin1h(None) => "quoteBin1h".to_string(),
            QuoteBin1h(Some(filter)) => format!("quoteBin1h:{}", filter),
            QuoteBin1d(None) => "quoteBin1d".to_string(),
            QuoteBin1d(Some(filter)) => format!("quoteBin1d:{}", filter),

            Settlement => "settlement".to_string(),

            Trade(None) => "trade".to_string(),
            Trade(Some(filter)) => format!("trade:{}", filter),
            TradeBin1m(None) => "tradeBin1m".to_string(),
            TradeBin1m(Some(filter)) => format!("tradeBin1m:{}", filter),
            TradeBin5m(None) => "tradeBin5m".to_string(),
            TradeBin5m(Some(filter)) => format!("tradeBin5m:{}", filter),
            TradeBin1h(None) => "tradeBin1h".to_string(),
            TradeBin1h(Some(filter)) => format!("tradeBin1h:{}", filter),
            TradeBin1d(None) => "tradeBin1d".to_string(),
            TradeBin1d(Some(filter)) => format!("tradeBin1d:{}", filter),

            // requires auth
            Affiliate => "affiliate".to_string(),
            Execution => "execution".to_string(),
            Order => "order".to_string(),
            Margin => "margin".to_string(),
            Position => "position".to_string(),
            PrivateNotifications => "privateNotifications".to_string(),
            Transact => "transact".to_string(),
            Wallet => "wallet".to_string(),
        };

        writeln!(f, "{}", repr)
    }
}

impl std::str::FromStr for Topic {
    type Err = BitMEXError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Topic::*;
        let reprs: Vec<_> = s.split(':').collect();

        let topic = match reprs.as_slice() {
            ["announcement"] => Announcement,
            ["chat"] => Chat,
            ["connected"] => Connected,
            ["funding"] => Funding,
            ["instrument"] => Instrument,
            ["insurance"] => Insurance,
            ["liquidation"] => Liquidation,

            ["orderBook10"] => OrderBook10(None),
            ["orderBook10", filter] => OrderBook10(Some((*filter).to_string())),
            ["orderBookL2"] => OrderBookL2(None),
            ["orderBookL2", filter] => OrderBookL2(Some((*filter).to_string())),
            ["orderBookL2_25"] => OrderBookL2_25(None),
            ["orderBookL2_25", filter] => OrderBookL2_25(Some((*filter).to_string())),

            ["publicNotifications"] => PublicNotifications,

            ["quote"] => Quote(None),
            ["quote", filter] => Quote(Some((*filter).to_string())),
            ["quoteBin1m"] => QuoteBin1m(None),
            ["quoteBin1m", filter] => QuoteBin1m(Some((*filter).to_string())),
            ["quoteBin5m"] => QuoteBin5m(None),
            ["quoteBin5m", filter] => QuoteBin5m(Some((*filter).to_string())),
            ["quoteBin1h"] => QuoteBin1h(None),
            ["quoteBin1h", filter] => QuoteBin1h(Some((*filter).to_string())),
            ["quoteBin1d"] => QuoteBin1d(None),
            ["quoteBin1d", filter] => QuoteBin1d(Some((*filter).to_string())),

            ["settlement"] => Settlement,

            ["trade"] => Trade(None),
            ["trade", filter] => Trade(Some((*filter).to_string())),
            ["tradeBin1m"] => TradeBin1m(None),
            ["tradeBin1m", filter] => TradeBin1m(Some((*filter).to_string())),
            ["tradeBin5m"] => TradeBin5m(None),
            ["tradeBin5m", filter] => TradeBin5m(Some((*filter).to_string())),
            ["tradeBin1h"] => TradeBin1h(None),
            ["tradeBin1h", filter] => TradeBin1h(Some((*filter).to_string())),
            ["tradeBin1d"] => TradeBin1d(None),
            ["tradeBin1d", filter] => TradeBin1d(Some((*filter).to_string())),

            // requires auth
            ["affiliate"] => Affiliate,
            ["execution"] => Execution,
            ["order"] => Order,
            ["margin"] => Margin,
            ["position"] => Position,
            ["privateNotifications"] => PrivateNotifications,
            ["transact"] => Transact,
            ["wallet"] => Wallet,
            _ => throw!(BitMEXError::ParseTopicError(s.into())),
        };

        Ok(topic)
    }
}
impl Serialize for Topic {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Topic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let repr = String::deserialize(deserializer)?;
        let topic = repr
            .parse()
            .map_err(|_| D::Error::invalid_value(Unexpected::Str(&repr), &"A valid topic"))?;
        Ok(topic)
    }
}

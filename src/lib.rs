pub mod sniffer;
pub mod accountant;
pub mod trader;
pub mod merchant;
use agnostic::coin::Coin;

#[derive(Default, Clone, Debug)]
pub struct CoinConverter {}

impl agnostic::coin::CoinConverter for CoinConverter {
    type Coin = btc_sdk::coin::Coin;

    fn to_string(&self, coin: agnostic::coin::Coin) -> String {
        match coin {
            Coin::USDT => btc_sdk::coin::Coin::USDT.to_string(),
            Coin::TON => btc_sdk::coin::Coin::TON.to_string(),
            Coin::BTC => btc_sdk::coin::Coin::BTC.to_string(),
            Coin::Unknown(c) => btc_sdk::coin::Coin::Unknown(c).to_string(),
        }
    }

    fn to_coin(&self, coin: agnostic::coin::Coin) -> Self::Coin {
        match coin {
            Coin::USDT => btc_sdk::coin::Coin::USDT,
            Coin::TON => btc_sdk::coin::Coin::TON,
            Coin::BTC => btc_sdk::coin::Coin::BTC,
            Coin::Unknown(c) => btc_sdk::coin::Coin::Unknown(c),
        }
    }

    fn from_coin(&self, coin: Self::Coin) -> agnostic::coin::Coin {
        match coin {
            btc_sdk::coin::Coin::USDT => Coin::USDT,
            btc_sdk::coin::Coin::TON => Coin::TON,
            btc_sdk::coin::Coin::BTC => Coin::BTC,
            btc_sdk::coin::Coin::Unknown(c) => Coin::Unknown(c),
        }
    }
}

pub struct SymbolResult(Result<btc_sdk::coin::Symbol, String>);

impl From<&agnostic::coin::CoinPair> for SymbolResult {
    fn from(pair: &agnostic::coin::CoinPair) -> Self {
        use agnostic::coin::CoinConverter;
        let converter = crate::CoinConverter::default();
        let sell_coin = converter.to_coin(pair.sell.clone());
        let buy_coin = converter.to_coin(pair.buy.clone());
        match (sell_coin, buy_coin) {
            (btc_sdk::coin::Coin::TON, btc_sdk::coin::Coin::USDT) => SymbolResult(Ok(
                btc_sdk::coin::Symbol::new(
                    btc_sdk::coin::Coin::TON,
                    btc_sdk::coin::Coin::USDT)
            )),
            (btc_sdk::coin::Coin::USDT, btc_sdk::coin::Coin::TON) => SymbolResult(Ok(
                btc_sdk::coin::Symbol::new(
                    btc_sdk::coin::Coin::TON,
                    btc_sdk::coin::Coin::USDT)
            )),
            (left, right) => SymbolResult(Err(format!(
                "INVALID CURRENCY. CANNOT PROCESS. LEFT {} | RIGHT {}",
                left.to_string(),
                right.to_string()))),
        }
    }
}

pub struct SideResult(Result<btc_sdk::base::Side, String>);

impl From<&agnostic::coin::CoinPair> for SideResult {
    fn from(pair: &agnostic::coin::CoinPair) -> SideResult {
        use agnostic::coin::CoinConverter;
        let converter = crate::CoinConverter::default();
        match pair.sell.clone() {
            Coin::TON => SideResult(Ok(btc_sdk::base::Side::Sell)),
            Coin::USDT => SideResult(Ok(btc_sdk::base::Side::Buy)),
            any => SideResult(Err(format!(
                "INVALID CURRENCY. FAILED TO FIND SIDE: {}",
                converter.to_string(any)))),
        }
    }
}

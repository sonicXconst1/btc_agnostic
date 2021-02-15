pub mod accountant;
pub mod merchant;
pub mod sniffer;
pub mod trader;
use agnostic::trading_pair;
use agnostic::trading_pair::Coin;
use agnostic::trading_pair::Coins;
use agnostic::trading_pair::TradingPair;
use btc_sdk::coin::{Coin as BtcCoin, Symbol};

#[derive(Default, Clone, Debug)]
pub struct TradingPairConverter {}

impl trading_pair::TradingPairConverter for TradingPairConverter {
    type Coin = btc_sdk::coin::Coin;
    type Pair = btc_sdk::coin::Symbol;

    fn to_string(&self, trading_pair: TradingPair) -> String {
        self.to_pair(trading_pair).to_string()
    }

    fn to_pair(&self, trading_pair: TradingPair) -> Self::Pair {
        match trading_pair.coins {
            Coins::TonUsdt => Symbol::new(BtcCoin::TON, BtcCoin::USDT),
        }
    }

    fn from_agnostic_coin(&self, coin: trading_pair::Coin) -> Self::Coin {
        match coin {
            Coin::TON => BtcCoin::TON,
            Coin::USDT => BtcCoin::USDT,
        }
    }

    fn to_agnostic_coin(&self, coin: Self::Coin) -> Option<trading_pair::Coin> {
        match coin {
            BtcCoin::TON => Some(Coin::TON),
            BtcCoin::USDT => Some(Coin::USDT),
            any => {
                log::debug!("Invalid coin: {:#?}", any);
                None
            },
        }
    }
}

pub fn from_agnostic_side(target: trading_pair::Target, side: trading_pair::Side) -> btc_sdk::base::Side {
    match (target, side) {
        (trading_pair::Target::Market, trading_pair::Side::Buy) => btc_sdk::base::Side::Buy,
        (trading_pair::Target::Market, trading_pair::Side::Sell) => btc_sdk::base::Side::Sell,
        (trading_pair::Target::Limit, trading_pair::Side::Buy) => btc_sdk::base::Side::Sell,
        (trading_pair::Target::Limit, trading_pair::Side::Sell) => btc_sdk::base::Side::Buy,
    }
}

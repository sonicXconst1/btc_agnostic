use agnostic::trading_pair::TradingPair;
use agnostic::trading_pair::TradingPairConverter;
use agnostic::trading_pair::Coin;
use agnostic::trading_pair::Side;

pub struct Accountant<TConnector>
{
    private_client: std::sync::Arc<btc_sdk::client::BTCClient<TConnector>>,
    price_epsilon: f64,
}

impl<TConnector> Accountant<TConnector>
where
    TConnector: hyper::client::connect::Connect + Sync + Send + Clone + 'static
{
    pub fn new(
        private_client: std::sync::Arc<btc_sdk::client::BTCClient<TConnector>>,
    ) -> Accountant<TConnector> {
        Accountant {
            private_client,
            price_epsilon: 0.0001,
        }
    }
}

impl<TConnector> agnostic::market::Accountant for Accountant<TConnector> 
where
    TConnector: hyper::client::connect::Connect + Sync + Send + Clone + 'static
{
    fn ask(
        &self,
        coin: Coin,
    ) -> agnostic::market::Future<Result<agnostic::currency::Currency, String>> {
        let private_client = self.private_client.clone();
        let converter = crate::TradingPairConverter::default();
        let future = async move {
            let balance = match private_client.get_trading_balance().await {
                Some(balance) => balance,
                None => return Err("FAILED TO GET TRADING BALANCE".to_owned()),
            };
            let coin_as_string = converter.from_agnostic_coin(coin.clone()).to_string();
            match balance.into_iter().find_map(|currency| {
                if currency.currency != coin_as_string {
                    return None;
                }
                let currency = btc_sdk::models::typed::Currency::from(currency);
                Some(agnostic::currency::Currency {
                    coin: coin.clone(),
                    amount: currency.available,
                    held: currency.reserved,
                })
            }) {
                Some(currency) => Ok(currency),
                None => Err("Failed to find currency in balance".to_owned()),
            }
        };
        Box::pin(future)
    }

    fn ask_both(
        &self,
        left: Coin,
        right: Coin,
    ) -> agnostic::market::Future<Result<(agnostic::currency::Currency, agnostic::currency::Currency), String>> {
        let private_client = self.private_client.clone();
        let converter = crate::TradingPairConverter::default();
        let future = async move {
            let balance = match private_client.get_trading_balance().await {
                Some(balance) => balance,
                None => return Err("FAILED TO GET TRADING BALANCE".to_owned()),
            };
            let left_coin_as_string = converter.from_agnostic_coin(left.clone()).to_string();
            let left_coin = balance.iter().find_map(|currency| {
                if currency.currency != left_coin_as_string {
                    return None;
                }
                let currency = btc_sdk::models::typed::Currency::from(currency.clone());
                Some(agnostic::currency::Currency {
                    coin: left.clone(),
                    amount: currency.available,
                    held: currency.reserved,
                })
            });
            let right_coin_as_string = converter.from_agnostic_coin(right.clone()).to_string();
            let right_coin = balance.iter().find_map(|currency| {
                if currency.currency != right_coin_as_string {
                    return None;
                }
                let currency = btc_sdk::models::typed::Currency::from(currency.clone());
                Some(agnostic::currency::Currency {
                    coin: right.clone(),
                    amount: currency.available,
                    held: currency.reserved,
                })
            });
            match (left_coin, right_coin) {
                (Some(left), Some(right)) => Ok((left, right)),
                _ => Err("Failed to find coins in wallet".to_owned()),
            }
        };
        Box::pin(future)
    }

    fn calculate_volume(&self, _trading_pair: TradingPair, price: f64, amount: f64) -> f64 {
        price * amount
    }

    fn nearest_price(&self, trading_pair: TradingPair, price: f64) -> f64 {
        match trading_pair.side {
            Side::Buy => price + self.price_epsilon,
            Side::Sell => price - self.price_epsilon,
        }
    }
}

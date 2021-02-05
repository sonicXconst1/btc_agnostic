use agnostic::coin::CoinConverter;

pub struct Accountant<TConnector>
{
    private_client: std::sync::Arc<btc_sdk::client::BTCClient<TConnector>>,
}

impl<TConnector> Accountant<TConnector>
where
    TConnector: hyper::client::connect::Connect + Sync + Send + Clone + 'static
{
    pub fn new(
        private_client: std::sync::Arc<btc_sdk::client::BTCClient<TConnector>>,
    ) -> Accountant<TConnector> {
        Accountant {
            private_client
        }
    }
}

impl<TConnector> agnostic::market::Accountant for Accountant<TConnector> 
where
    TConnector: hyper::client::connect::Connect + Sync + Send + Clone + 'static
{
    fn ask(&self, coin: agnostic::coin::Coin) -> agnostic::market::Future<Result<agnostic::currency::Currency, String>> {
        let private_client = self.private_client.clone();
        let converter = crate::CoinConverter::default();
        let future = async move {
            let balance = match private_client.get_trading_balance().await {
                Some(balance) => balance,
                None => return Err("FAILED TO GET TRADING BALANCE".to_owned()),
            };
            let coin_as_string = converter.to_string(coin.clone());
            balance.into_iter().find_map(|currency| {
                if currency.currency != coin_as_string {
                    return None;
                }
                let currency = btc_sdk::models::typed::Currency::from(currency);
                Some(agnostic::currency::Currency {
                    coin: coin.clone(),
                    amount: currency.available,
                    held: currency.reserved,
                })
            });
            unimplemented!()
        };
        Box::pin(future)
    }

    fn ask_both(
        &self,
        coins: agnostic::coin::CoinPair,
    ) -> agnostic::market::Future<Result<(agnostic::currency::Currency, agnostic::currency::Currency), String>> {
        let private_client = self.private_client.clone();
        let converter = crate::CoinConverter::default();
        let future = async move {
            let balance = match private_client.get_trading_balance().await {
                Some(balance) => balance,
                None => return Err("FAILED TO GET TRADING BALANCE".to_owned()),
            };
            let sell_coin_as_string = converter.to_string(coins.sell.clone());
            let left_coin = balance.iter().find_map(|currency| {
                if currency.currency != sell_coin_as_string {
                    return None;
                }
                let currency = btc_sdk::models::typed::Currency::from(currency.clone());
                Some(agnostic::currency::Currency {
                    coin: coins.sell.clone(),
                    amount: currency.available,
                    held: currency.reserved,
                })
            });
            let buy_coin_as_string = converter.to_string(coins.buy.clone());
            let right_coin = balance.iter().find_map(|currency| {
                if currency.currency != buy_coin_as_string {
                    return None;
                }
                let currency = btc_sdk::models::typed::Currency::from(currency.clone());
                Some(agnostic::currency::Currency {
                    coin: coins.buy.clone(),
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

    fn calculate_volume(&self, _coins: agnostic::coin::CoinPair, price: f64, amount: f64) -> f64 {
        price * amount
    }

    fn nearest_price(&self, coins: agnostic::coin::CoinPair, price: f64) -> f64 {
        match crate::SideResult::from(&coins) {
            crate::SideResult(Ok(side)) => {
                match side {
                    btc_sdk::base::Side::Sell => {
                        price - 0.0001
                    },
                    btc_sdk::base::Side::Buy => {
                        price + 0.0001
                    },
                }
            },
            crate::SideResult(Err(error)) => {
                log::error!("{}", error);
                price
            }
        }
    }
}

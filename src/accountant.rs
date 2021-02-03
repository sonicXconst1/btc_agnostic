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
        todo!()
    }

    fn ask_both(
        &self,
        coins: agnostic::coin::CoinPair,
    ) -> agnostic::market::Future<Result<(agnostic::currency::Currency, agnostic::currency::Currency), String>> {
        todo!()
    }

    fn calculate_volume(&self, coins: agnostic::coin::CoinPair, price: f64, amount: f64) -> f64 {
        todo!()
    }
}

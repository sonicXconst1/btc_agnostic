use crate::SymbolResult;
use crate::SideResult;

pub struct Sniffer<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    client: std::sync::Arc<btc_sdk::public_client::PublicClient<TConnector>>,
}

impl<TConnector> Sniffer<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    pub fn new(
        client: std::sync::Arc<btc_sdk::public_client::PublicClient<TConnector>>,
    ) -> Sniffer<TConnector> {
        Sniffer { client }
    }
}

impl<TConnector> agnostic::market::Sniffer for Sniffer<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    fn all_the_best_orders(
        &self,
        coins: agnostic::coin::CoinPair,
        count: u32,
    ) -> agnostic::market::Future<Result<Vec<agnostic::order::Order>, String>> {
        let client = self.client.clone();
        let future = async move {
            let symbol = match SymbolResult::from(&coins) {
                SymbolResult(Ok(symbol)) => symbol,
                SymbolResult(Err(error)) => {
                    log::error!("{}", error);
                    return Err(error);
                }
            };
            let side = match SideResult::from(&coins) {
                SideResult(Ok(side)) => side,
                SideResult(Err(error)) => {
                    log::error!("{}", error);
                    return Err(error);
                }
            };
            let orderbook = match client
                .get_orderbook(Some(count as u64), Some(vec![symbol.clone()]))
                .await
            {
                Some(orderbook) => orderbook,
                None => {
                    return Err("Failed to get orderbook".to_owned());
                }
            };
            let page = match btc_sdk::models::typed::OrderBookPage::new(
                symbol.clone(),
                side,
                &orderbook,
            ) {
                Some(page) => page,
                None => {
                    return Err("Failed to get page from orderbook".to_owned());
                }
            };
            Ok(page
                .prices
                .into_iter()
                .map(|price| {
                    let rate = match side {
                        btc_sdk::base::Side::Sell => price.rate,
                        btc_sdk::base::Side::Buy => 1.0 / price.rate,
                    };
                    agnostic::order::Order {
                        coins: coins.clone(),
                        price: rate,
                        amount: price.amount,
                    }
                })
                .collect())
        };
        Box::pin(future)
    }

    fn the_best_order(
        &self,
        coins: agnostic::coin::CoinPair,
    ) -> agnostic::market::Future<Result<agnostic::order::Order, String>> {
        let future = self.all_the_best_orders(coins, 1u32);
        let future = async move {
            match future.await {
                Ok(orders) => match orders.get(0) {
                    Some(order) => Ok(order.clone()),
                    None => Err("Failed to get the first order from orders.".to_owned()),
                },
                Err(error) => Err(error),
            }
        };
        Box::pin(future)
    }
}

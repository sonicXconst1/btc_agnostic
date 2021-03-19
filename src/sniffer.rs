use crate::from_agnostic_side;
use agnostic::trading_pair::TradingPair;
use agnostic::trading_pair::TradingPairConverter;
use std::str::FromStr;

pub struct Sniffer<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    client: std::sync::Arc<btc_sdk::public_client::PublicClient<TConnector>>,
    private_client: std::sync::Arc<btc_sdk::client::BTCClient<TConnector>>,
}

impl<TConnector> Sniffer<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    pub fn new(
        client: std::sync::Arc<btc_sdk::public_client::PublicClient<TConnector>>,
        private_client: std::sync::Arc<btc_sdk::client::BTCClient<TConnector>>,
    ) -> Sniffer<TConnector> {
        Sniffer {
            client,
            private_client,
        }
    }
}

impl<TConnector> agnostic::market::Sniffer for Sniffer<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    fn all_the_best_orders(
        &self,
        trading_pair: TradingPair,
        count: u32,
    ) -> agnostic::market::Future<Result<Vec<agnostic::order::Order>, String>> {
        let client = self.client.clone();
        let future = async move {
            let converter = crate::TradingPairConverter::default();
            let symbol = converter.to_pair(trading_pair.clone());
            let side = from_agnostic_side(trading_pair.target.clone(), trading_pair.side.clone());
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
                .map(|price| agnostic::order::Order {
                    trading_pair: trading_pair.clone(),
                    price: price.rate,
                    amount: price.amount,
                })
                .collect())
        };
        Box::pin(future)
    }

    fn get_my_orders(
        &self,
        trading_pair: TradingPair,
    ) -> agnostic::market::Future<Result<Vec<agnostic::order::OrderWithId>, String>> {
        let client = self.private_client.clone();
        let future = async move {
            let converter = crate::TradingPairConverter::default();
            let symbol = converter.to_pair(trading_pair.clone());
            let side = from_agnostic_side(trading_pair.target.clone(), trading_pair.side.clone());
            match client.get_active_orders(Some(symbol)).await {
                Some(orders) => Ok(orders
                    .into_iter()
                    .filter_map(|order| {
                        let sell_string = "sell".to_owned();
                        let order_side = if sell_string == order.side {
                            btc_sdk::base::Side::Sell
                        } else {
                            btc_sdk::base::Side::Buy
                        };
                        if order_side != side {
                            return None;
                        }
                        Some(agnostic::order::OrderWithId {
                            id: order.client_order_id,
                            trading_pair: trading_pair.clone(),
                            price: f64::from_str(
                                &order
                                    .price
                                    .expect("Orderbook cannot return order without price"),
                            )
                            .unwrap(),
                            amount: f64::from_str(&order.quantity).unwrap(),
                        })
                    })
                    .collect()),
                None => Err("Failed to get active orders".to_owned()),
            }
        };
        Box::pin(future)
    }
}

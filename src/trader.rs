use crate::from_agnostic_side;
use agnostic::trading_pair::{TradingPairConverter, Target};
use agnostic::order::{Order, OrderWithId};
use agnostic::market;
use agnostic::trade::{Trade, TradeResult};

pub struct Trader<TConnector> {
    private_client: std::sync::Arc<btc_sdk::client::BTCClient<TConnector>>,
}

impl<TConnector> Trader<TConnector>
where
    TConnector: hyper::client::connect::Connect + Sync + Send + Clone + 'static,
{
    pub fn new(
        private_client: std::sync::Arc<btc_sdk::client::BTCClient<TConnector>>,
    ) -> Trader<TConnector> {
        Trader { private_client }
    }
}

impl<TConnector> agnostic::market::Trader for Trader<TConnector>
where
    TConnector: hyper::client::connect::Connect + Sync + Send + Clone + 'static,
{
    fn create_order(&self, order: Order) -> market::Future<Result<Trade, String>> {
        let client = self.private_client.clone();
        let future = async move {
            let converter = crate::TradingPairConverter::default();
            let trading_pair = order.trading_pair.clone();
            let symbol = converter.to_pair(order.trading_pair.clone());
            let side = from_agnostic_side(
                order.trading_pair.target.clone(),
                order.trading_pair.side.clone());
            let price = order.price;
            let amount = order.amount;
            match order.trading_pair.target {
                Target::Market => {
                    let converter = crate::TradingPairConverter::default();
                    let order = btc_sdk::models::typed::CreateMarketOrder::new(
                        converter.to_pair(order.trading_pair.clone()),
                        match order.trading_pair.side {
                            agnostic::trading_pair::Side::Sell => btc_sdk::base::Side::Sell,
                            agnostic::trading_pair::Side::Buy => btc_sdk::base::Side::Buy,
                        },
                        order.amount);
                    use std::str::FromStr;
                    let order = client.create_market_order(order).await?; 
                    Ok(Trade::Market(TradeResult{
                        id: order.id.to_string(),
                        trading_pair,
                        price: match order.price{
                            Some(trade_price) => match f64::from_str(&trade_price) {
                                Ok(price) => price,
                                Err(_) => price,
                            },
                            None => price,
                        },
                        amount,
                    }))
                },
                Target::Limit => {
                    let order = btc_sdk::models::typed::CreateLimitOrder::new(
                        symbol,
                        side,
                        amount,
                        price);
                    let order = client.create_limit_order(order).await?; 
                    Ok(Trade::Limit(OrderWithId {
                        id: order.id.to_string(),
                        trading_pair,
                        price,
                        amount,
                    }))
                },
            }
        };
        Box::pin(future)
    }

    fn delete_order(&self, id: &str) -> agnostic::market::Future<Result<(), String>> {
        let client = self.private_client.clone();
        let id = id.to_owned();
        let future = async move {
            let _order = client.cancel_order_by_id(&id).await?;
            Ok(())
        };
        Box::pin(future)
    }
}

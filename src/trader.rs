use crate::from_agnostic_side;
use agnostic::trading_pair::TradingPairConverter;

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
    fn create_order(
        &self,
        order: agnostic::order::Order,
    ) -> agnostic::market::Future<Result<(), String>> {
        let client = self.private_client.clone();
        let future = async move {
            let converter = crate::TradingPairConverter::default();
            let symbol = converter.to_pair(order.trading_pair.clone());
            let side = from_agnostic_side(
                order.trading_pair.target.clone(),
                order.trading_pair.side.clone());
            let order = btc_sdk::models::typed::CreateLimitOrder::new(
                symbol,
                side,
                order.amount,
                order.price);
            match client.create_limit_order(order).await {
                Some(order) => {
                    log::debug!("Limit order created: {:#?}", order);
                    Ok(())
                },
                None => Err("Failed to create limit order!".to_owned()),
            }
        };
        Box::pin(future)
    }

    fn delete_and_create(
        &self,
        id: &str,
        new_order: agnostic::order::Order,
    ) -> agnostic::market::Future<Result<String, String>> {
        let client = self.private_client.clone();
        let id = id.to_owned();
        let future = async move {
            match client.cancel_order_by_id(&id).await {
                Some(order) => {
                    log::debug!("Order canceled: {:#?}", order);
                    let converter = crate::TradingPairConverter::default();
                    let symbol = converter.to_pair(new_order.trading_pair.clone());
                    let side = from_agnostic_side(
                        new_order.trading_pair.target.clone(),
                        new_order.trading_pair.side.clone());
                    let order = btc_sdk::models::typed::CreateLimitOrder::new(
                        symbol,
                        side,
                        new_order.amount,
                        new_order.price);
                    match client.create_limit_order(order).await {
                        Some(order) => {
                            let result = format!("Limit order created: {:#?}", order);
                            Ok(result)
                        },
                        None => Err("Failed to create limit order!".to_owned()),
                    }
                },
                None => Err("Failed to cancel order by id".to_owned()),
            }
        };
        Box::pin(future)
    }

    fn delete_order(&self, id: &str) -> agnostic::market::Future<Result<(), String>> {
        let client = self.private_client.clone();
        let id = id.to_owned();
        let future = async move {
            match client.cancel_order_by_id(&id).await {
                Some(order) => {
                    log::debug!("Order canceled: {:#?}", order);
                    Ok(())
                },
                None => Err("Failed to cancel order by id".to_owned()),
            }
        };
        Box::pin(future)
    }

    fn create_trade_from_order(
        &self,
        order: agnostic::order::Order,
    ) -> agnostic::market::Future<Result<(), String>> {
        let client = self.private_client.clone();
        let future = async move {
            let converter = crate::TradingPairConverter::default();
            let order = btc_sdk::models::typed::CreateMarketOrder::new(
                converter.to_pair(order.trading_pair.clone()),
                crate::from_agnostic_side(
                    order.trading_pair.target.clone(),
                    order.trading_pair.side.clone()),
                order.amount);
            match client.create_market_order(order).await {
                Some(order) => {
                    log::debug!("Order canceled: {:#?}", order);
                    Ok(())
                },
                None => Err("Failed to cancel order by id".to_owned()),
            }
        };
        Box::pin(future)
    }
}

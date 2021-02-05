use crate::SymbolResult;
use crate::SideResult;

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
            let symbol = match SymbolResult::from(&order.coins) {
                SymbolResult(Ok(symbol)) => symbol,
                SymbolResult(Err(error)) => return Err(error),
            };
            let side = match SideResult::from(&order.coins) {
                SideResult(Ok(side)) => side,
                SideResult(Err(error)) => return Err(error),
            };
            let price = match side {
                btc_sdk::base::Side::Sell => order.price,
                btc_sdk::base::Side::Buy => 1f64 / order.price,
            };
            let order = btc_sdk::models::typed::CreateLimitOrder::new(
                symbol,
                side,
                order.amount,
                price);
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
                    let symbol = match SymbolResult::from(&new_order.coins) {
                        SymbolResult(Ok(symbol)) => symbol,
                        SymbolResult(Err(error)) => return Err(error),
                    };
                    let side = match SideResult::from(&new_order.coins) {
                        SideResult(Ok(side)) => side,
                        SideResult(Err(error)) => return Err(error),
                    };
                    let price = match side {
                        btc_sdk::base::Side::Sell => new_order.price,
                        btc_sdk::base::Side::Buy => 1f64 / new_order.price,
                    };
                    let order = btc_sdk::models::typed::CreateLimitOrder::new(
                        symbol,
                        side,
                        new_order.amount,
                        price);
                    match client.create_limit_order(order).await {
                        Some(order) => {
                            log::debug!("Limit Order created: {:#?}", order);
                            Ok(format!("{}", order.id))
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

    fn create_trade_by_id(&self, order_id: &str) -> agnostic::market::Future<Result<(), String>> {
        todo!()
    }

    fn create_trade_from_order(
        &self,
        order: agnostic::order::Order,
    ) -> agnostic::market::Future<Result<(), String>> {
        todo!()
    }
}

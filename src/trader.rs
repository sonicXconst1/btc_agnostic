pub struct Trader<TConnector>
{
    private_client: std::sync::Arc<btc_sdk::client::BTCClient<TConnector>>,
}

impl<TConnector> Trader<TConnector>
where
    TConnector: hyper::client::connect::Connect + Sync + Send + Clone + 'static
{
    pub fn new(
        private_client: std::sync::Arc<btc_sdk::client::BTCClient<TConnector>>,
    ) -> Trader<TConnector> {
        Trader {
            private_client
        }
    }
}

impl<TConnector> agnostic::market::Trader for Trader<TConnector> 
where
    TConnector: hyper::client::connect::Connect + Sync + Send + Clone + 'static
{
    fn create_order(&self, order: agnostic::order::Order) -> agnostic::market::Future<Result<(), String>> {
        todo!()
    }

    fn update_order(&self, id: &str, new_order: agnostic::order::Order) -> agnostic::market::Future<Result<(), String>> {
        todo!()
    }

    fn delete_order(&self, id: &str) -> agnostic::market::Future<Result<(), String>> {
        todo!()
    }

    fn create_trade_by_id(&self, order_id: &str) -> agnostic::market::Future<Result<(), String>> {
        todo!()
    }

    fn create_trade_from_order(&self, order: agnostic::order::Order) -> agnostic::market::Future<Result<(), String>> {
        todo!()
    }
}

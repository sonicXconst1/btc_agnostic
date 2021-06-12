use std::sync::Arc;
pub struct Merchant<TConnector> 
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static
{
    id: &'static str,
    accountant: Arc<crate::accountant::Accountant<TConnector>>,
    sniffer: Arc<crate::sniffer::Sniffer<TConnector>>,
    trader: Arc<crate::trader::Trader<TConnector>>,
}

impl<TConnector> Merchant<TConnector> 
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static
{
    pub fn new(
        id: &'static str,
        client: std::sync::Arc<hyper::Client<TConnector>>,
        public_key: String,
        private_key: String,
        base_url: url::Url,
    ) -> Merchant<TConnector> {
        let auth_context = std::sync::Arc::new(btc_sdk::context::AuthContext::new(
            public_key,
            private_key,
            base_url));
        let private_client = std::sync::Arc::new(btc_sdk::client::BTCClient::new(
            client.clone(),
            auth_context.clone()));
        let public_client = std::sync::Arc::new(btc_sdk::public_client::PublicClient::new(
            client.clone(),
            auth_context.base_url.clone()));
        let accountant = Arc::new(crate::accountant::Accountant::new(private_client.clone()));
        let sniffer = Arc::new(crate::sniffer::Sniffer::new(
                public_client.clone(),
                private_client.clone()));
        let trader = Arc::new(crate::trader::Trader::new(private_client.clone()));
        Merchant {
            id,
            accountant,
            sniffer,
            trader,
        }
    }
}

impl<TConnector> agnostic::merchant::Merchant for Merchant<TConnector> 
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static
{
    fn id(&self) -> &'static str {
        self.id
    }

    fn accountant(&self) -> std::sync::Arc<dyn agnostic::market::Accountant> {
        self.accountant.clone()
    }

    fn trader(&self) -> std::sync::Arc<dyn agnostic::market::Trader> {
        self.trader.clone()
    }

    fn sniffer(&self) -> std::sync::Arc<dyn agnostic::market::Sniffer> {
        self.sniffer.clone()
    }
}

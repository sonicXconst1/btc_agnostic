pub struct Merchant<TConnector> {
    private_client: std::sync::Arc<btc_sdk::client::BTCClient<TConnector>>,
    public_client: std::sync::Arc<btc_sdk::public_client::PublicClient<TConnector>>,
}

impl<TConnector> Merchant<TConnector> 
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static
{
    pub fn new(
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
        Merchant {
            private_client,
            public_client,
        }
    }
}

impl<TConnector> agnostic::merchant::Merchant for Merchant<TConnector> 
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static
{
    type Accountant = crate::accountant::Accountant<TConnector>;
    type Trader = crate::trader::Trader<TConnector>;
    type Sniffer = crate::sniffer::Sniffer<TConnector>;

    fn accountant(&self) -> Self::Accountant {
        crate::accountant::Accountant::new(self.private_client.clone())
    }

    fn trader(&self) -> Self::Trader {
        crate::trader::Trader::new(self.private_client.clone())
    }

    fn sniffer(&self) -> Self::Sniffer {
        crate::sniffer::Sniffer::new(
            self.public_client.clone(),
            self.private_client.clone())
    }
}

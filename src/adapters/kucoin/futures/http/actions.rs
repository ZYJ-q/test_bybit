use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;

use super::client::KucoinHttpClient;

pub struct KucoinFuturesApi {
    client: KucoinHttpClient,
}

impl KucoinFuturesApi {
    pub fn new(
        base_url: &str,
        api_key: &str,
        api_secret: &str,
        recv_window: &str,
    ) -> Self {
        let client = KucoinHttpClient::new(base_url, api_key, api_secret, recv_window, );
        Self { client: client }
    }

    pub async fn get_account_overview(&self, account_type: Option<&str>) -> Option<Value> {
        // let my_currency = String::from(currency.unwrap_or("USDT"));

        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert(
            String::from("accountType"),
            serde_json::to_value(account_type).unwrap(),
        );

        let response = self
            .client
            .send(Method::GET, "/v5/account/wallet-balance", true, &params)
            .await;

        let res_data = self.client.check_response_data(response);

        println!("账户信息{:?}", res_data);

        match res_data {
            Some(data) => {
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }
}

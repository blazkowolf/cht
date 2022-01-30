use crate::config::Config;
use crate::error::ChtError;
use hyper::Uri;
use std::error;

use hyper::{Body, Client, Response};

pub struct ChtClient {
    scheme: String,
    base_uri: String,
}

impl Default for ChtClient {
    fn default() -> Self {
        ChtClient {
            scheme: "http".to_string(),
            base_uri: "cht.sh".to_string(),
        }
    }
}

impl ChtClient {
    pub async fn cheat(
        &self,
        config: Config,
    ) -> Result<Response<Body>, Box<dyn error::Error + Send + Sync>> {
        let uri = self.get_req_uri(config)?;
        let client = Client::new();

        let res = client.get(uri).await?;

        println!("Response: {}", res.status());
        println!("Headers: {:#?}\n", res.headers());

        Ok(res)
    }

    fn get_req_uri(&self, config: Config) -> Result<Uri, ChtError> {
        let qry_str = config
            .get_query_str()
            .unwrap_or_else(|| ":list".to_string());
        let req_str = format!(
            "{}://{}/{}/{}",
            self.scheme, self.base_uri, config.lang, qry_str
        );
        let uri: Uri = req_str.parse()?;

        Ok(uri)
    }
}

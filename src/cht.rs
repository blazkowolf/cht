use crate::config::Config;
use crate::error::ChtError;
use hyper::{header::USER_AGENT, Uri};
use std::error;

use hyper::{Body, Client, Request, Response};

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
    #[allow(dead_code)]
    pub fn new(scheme: &str, base_uri: &str) -> ChtClient {
        ChtClient {
            scheme: scheme.to_owned(),
            base_uri: base_uri.to_owned(),
        }
    }

    pub async fn cheat(
        &self,
        config: Config,
    ) -> Result<Response<Body>, Box<dyn error::Error + Send + Sync>> {
        let uri = self.get_req_uri(config)?;
        let req = Request::get(uri)
            .header(USER_AGENT, "curl")
            .body(Body::empty())?;

        let client = Client::new();
        let res = client.request(req).await?;

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

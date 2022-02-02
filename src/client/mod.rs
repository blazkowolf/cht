pub mod config;

use crate::{error::ChtError, ChtArgs};
use hyper::{header::USER_AGENT, Uri, StatusCode};


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
        path_config: &ChtArgs,
    ) -> Result<Response<Body>, ChtError> {
        let uri = self.get_req_uri(path_config)?;
        let req = Request::get(uri)
            // User-Agent header set to `curl` is required
            // for cht.sh to return plain text reponse
            .header(USER_AGENT, "curl")
            .body(Body::empty())?;

        let client = Client::new();
        let res = client.request(req).await?;

        match res.status() {
            StatusCode::NOT_FOUND => Err(ChtError::UnknownCheatSheet),
            _ => Ok(res)
        }
    }

    fn get_req_uri(&self, path_config: &ChtArgs) -> Result<Uri, ChtError> {
        let req_str = format!(
            "{}://{}/{}",
            self.scheme, self.base_uri, path_config
        );
        let uri: Uri = req_str.parse()?;

        Ok(uri)
    }
}

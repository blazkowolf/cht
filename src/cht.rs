use std::error::Error;

use crate::config::Config;
use crate::error::ChtError;
use hyper::Uri;

pub struct Cht;

impl Cht {
    pub async fn run(config: Config) -> Result<(), Box<dyn Error + Send + Sync>> {
        let uri = Cht::get_req_uri(config)?;
        println!("{:?}", uri);

        Ok(())
    }

    fn get_req_uri(config: Config) -> Result<Uri, ChtError> {
        let req_str = match config.get_query_str() {
            Some(qry_str) => format!("https://cht.sh/{}/{}", config.lang, qry_str),
            None => format!("https://cht.sh/{}", config.lang),
        };
        let uri: Uri = req_str.parse()?;

        Ok(uri)
    }
}

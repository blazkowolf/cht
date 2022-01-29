use crate::error::ChtError;

pub struct Config {
    pub lang: String,
    query_parts: Vec<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, ChtError> {
        if args.len() < 2 {
            // return Err("Not enough arguments! Must provide a programming language name and optionally a query string");
            return Err(ChtError::TooFewArguments);
        }
        let _binary = args[0].to_owned();
        let lang = args[1].to_owned();
        let query_parts = args[2..].to_owned();

        Ok(Config { lang, query_parts })
    }

    pub fn get_query_str(&self) -> Option<String> {
        let qry = self.query_parts.join("+");
        if qry == "" {
            return None;
        }

        Some(qry)
    }
}

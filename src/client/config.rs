use hyper::Uri;

#[derive(Debug)]
pub struct ChtshClientConfig {
    pub base_uri: Uri,
}

impl ChtshClientConfig {
    pub fn new(base_uri: &str) -> Self {
        Self {
            base_uri: base_uri
                .parse()
                .expect("base_uri must be a valid URL segment"),
        }
    }
}

impl Default for ChtshClientConfig {
    fn default() -> Self {
        Self {
            base_uri: Uri::from_static("https://cht.sh"),
        }
    }
}

// impl std::fmt::Display for ChtshClientConfig {
//     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> std::fmt::Result {
//         formatter.write_fmt(format_args!("{}/{}", self.language, self.query_parts.join("+")))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_works() {
        let config = ChtshClientConfig::default();
        assert_eq!(config.base_uri, Uri::from_static("https://cht.sh"))
    }

    #[test]
    fn new_works() {
        let config = ChtshClientConfig::new("http://someotherurl.com");
        assert_eq!(config.base_uri, Uri::from_static("http://someotherurl.com"))
    }

    #[test]
    #[should_panic(expected = "base_uri must be a valid URL segment")]
    fn new_invalid_url() {
        let _ = ChtshClientConfig::new("yoyo:/bigban-stunna");
    }
}

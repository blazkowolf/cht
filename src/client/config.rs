use hyper::Uri;

#[derive(Debug)]
pub struct ChtshClientConfig {
    pub base_url: Uri,
}

impl ChtshClientConfig {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.parse().expect("base_url must be a valid URL segment"),
        }
    }
}

impl Default for ChtshClientConfig {
    fn default() -> Self {
        Self {
            base_url: Uri::from_static("http://cht.sh"),
        }
    }
}

// impl std::fmt::Display for ChtshClientConfig {
//     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> std::fmt::Result {
//         formatter.write_fmt(format_args!("{}/{}", self.language, self.query_parts.join("+")))
//     }
// }

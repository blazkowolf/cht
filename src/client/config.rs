use clap::Parser;
use std::fmt;

#[derive(Parser, Debug, PartialEq)]
#[clap(name = "cht.sh Rust CLI", author, version, about, long_about = None)]
pub struct ChtArgs {
    #[clap(required = true)]
    language: String,

    #[clap(default_value = ":list")]
    query_parts: Vec<String>,
}

impl fmt::Display for ChtArgs {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_fmt(format_args!("{}/{}", self.language, self.query_parts.join("+")))
    }
}

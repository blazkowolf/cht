use crate::error::ChtError;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub lang: String,
    query_parts: Vec<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, ChtError> {
        if args.len() < 2 {
            return Err(ChtError::TooFewArguments);
        }
        let _binary = args[0].to_owned();
        let lang = args[1].to_owned();
        let query_parts = args[2..].to_owned();

        Ok(Config { lang, query_parts })
    }

    pub fn get_query_str(&self) -> Option<String> {
        let qry = self.query_parts.join("+");
        if qry.is_empty() {
            return None;
        }

        Some(qry)
    }
}

#[cfg(test)]
mod tests {
    use super::{ChtError, Config};

    #[test]
    fn when_too_few_args_return_err() {
        // Arrange
        let args: Vec<String> = Vec::new();
        let expected = ChtError::TooFewArguments;
        // Act
        let _actual = Config::new(&args).unwrap_err();
        // Assert
        assert!(matches!(expected, _actual));
    }

    #[test]
    fn when_correct_args_return_cfg() {
        // Arrange
        let args = vec![
            String::from("binary_name"),
            String::from("one"),
            String::from("two"),
            String::from("three"),
        ];
        let expected = Config {
            lang: String::from("one"),
            query_parts: vec![String::from("two"), String::from("three")],
        };
        // Act
        let actual = Config::new(&args).unwrap();
        // Assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn when_no_qry_parts_return_none() {
        // Arrange
        let args = vec![String::from("binary_name"), String::from("rust")];
        let expected: Option<String> = Option::None;
        // Act
        let cfg = Config::new(&args).unwrap();
        let actual = cfg.get_query_str();
        // Assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn when_qry_parts_return_qry_str() {
        // Arrange
        // Act
        // Assert
    }
}

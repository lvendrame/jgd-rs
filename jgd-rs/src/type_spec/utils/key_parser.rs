use std::sync::LazyLock;

use regex::Regex;

use crate::Arguments;


static RE_KEY: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([^(]+)(\(.+\))?").unwrap());

pub struct KeyParser {
    pub key: String,
    pub arguments: Arguments,
}

impl From<&str> for KeyParser {
    fn from(value: &str) -> Self {
        let captures = RE_KEY.captures(value).unwrap();
        let key = captures.get(1).unwrap().as_str();
        let arguments = captures.get(2).map(|m| m.as_str());
        let arguments = Arguments::from(arguments.unwrap_or(""));

        Self { key: key.to_string(), arguments }
    }
}

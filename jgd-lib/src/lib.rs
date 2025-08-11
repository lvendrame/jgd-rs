use std::fs;

use serde_json::Value;

use crate::type_spec::Jgd;

mod type_spec;
mod fake;
mod locales_keys;

pub fn from_str(value: &str) -> Value {
    let jgd: Jgd = serde_json::from_str(value).unwrap();

    jgd.generate()
}

pub fn from_file(path: &str) -> Value {
    let jgd_string = fs::read_to_string(path);

    from_str(&jgd_string.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //let result = add(2, 2);
        //assert_eq!(result, 4);
    }
}

use crate::error::AppError;
use json::JsonValue;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {}

impl Config {
    pub fn from_file(path: &str, get_values: Vec<&str>) -> Result<Vec<String>, AppError> {
        let parsed: JsonValue = Self::json_parser(path)?;

        let mut values: Vec<String> = Vec::new();
        for value in get_values {
            values.push(parsed[value].as_str()
                .ok_or_else(|| AppError::MissingField(value.to_string()))?
                .to_string());
        }

        Ok(values)
    }

    fn json_parser(path: &str) -> Result<JsonValue, AppError> {
        let mut setting_file = File::open(path)?;
        let mut contents = String::new();
        setting_file.read_to_string(&mut contents)?;

        Ok(json::parse(&contents)?)
    }
}

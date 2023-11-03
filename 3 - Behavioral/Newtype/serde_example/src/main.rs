
use derive_more::{Display, Into};
use serde::{Serialize, Deserialize};

#[derive(Display, Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct IdNumber(String);

impl From<IdNumber> for String {
  fn from(s: IdNumber) -> String {
    s.0
  }
}

use std::str::FromStr;
impl FromStr for IdNumber {
    type Err = IdNumberParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("len: {}", s.len());

        if s.len() != 15 {
            Err(IdNumberParseError::InvalidFormat)
        } else {
            Ok(IdNumber(s.to_string()))
        }
    }
}

use std::convert::TryFrom;
impl TryFrom<String> for IdNumber {
    type Error = IdNumberParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        // This is boilerplate code, but don't rely on Derive More's
        // TryFrom for this. We need it to go through our
        // implementation of FromStr so that it follows our validation
        // rules.
        println!("value: {:?}", value);
        value.parse()
    }
}

#[derive(Display, Debug, PartialEq)]
pub enum IdNumberParseError {
    InvalidFormat
}

impl std::error::Error for IdNumberParseError {}

fn main() {

    // Processa JSON.
    let json = "\"123456789001443\"".to_string();
    let result: serde_json::Value = serde_json::from_str(&json).unwrap();
    println!("result: {}", result);

    // Realiza conversão para o tipo personalizado.
    let result :IdNumber = serde_json::from_str(&json).unwrap();
    println!("result = {:?}", result);
}



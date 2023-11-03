
use derive_more::Display;

#[derive(Display, Debug, PartialEq)]
pub struct IdNumber(String);

use std::str::FromStr;
impl FromStr for IdNumber {
    type Err = IdNumberParseError;

    // Realiza a validação permitindo apenas numeros com N caracteres.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 15 {
            Err(IdNumberParseError::InvalidFormat)
        } else {
            Ok(IdNumber(s.to_string()))
        }
    }
}

#[derive(Display, Debug, PartialEq)]
pub enum IdNumberParseError {
    InvalidFormat
}

impl std::error::Error for IdNumberParseError {}

fn main() {
    let id = IdNumber::from_str("12345");
    println!("{:?}", id);

    let id = IdNumber::from_str("1234567890123121");
    println!("{:?}", id);

    let id = IdNumber::from_str("123456789012312");
    println!("{:?}", id);

    println!("My ID Number is {}", id.unwrap());
}



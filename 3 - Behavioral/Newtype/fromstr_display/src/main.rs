
use std::str::FromStr;
use std::fmt;

pub struct PhoneNumber(String);

// Exemplo de como implementar a FromStr e Display da biblioteca padrão do rust
// em nosso novo tipo de dado.
impl FromStr for PhoneNumber {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PhoneNumber(s.to_string()))
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Exemplo de utilização das traits implementadas.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse() uses FromStr
    let num: PhoneNumber = "555-1234".parse()?;

    // you can also call from_str directly
    let num = PhoneNumber::from_str("555-1234")?;

    // Display gives you a to_string function
    let num_as_string = num.to_string();

    // Display can also be called directly by println! or format!
    println!("Phone number is {}", num);
    Ok(())
}



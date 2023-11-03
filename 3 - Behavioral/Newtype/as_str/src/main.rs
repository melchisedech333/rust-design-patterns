
pub struct PhoneNumber(String);

use std::str::FromStr;
impl FromStr for PhoneNumber {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PhoneNumber(s.to_string()))
    }
}

impl PhoneNumber {
    fn as_str(&self) -> &str {
        &self.0
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num = PhoneNumber::from_str("555-1234")?;

    // Since we didn't implement Deref, the compiler can't convert to
    // a string implicitly, but it's still possible for us to do that
    // dereferencing explicitly.
    print_strings(num.as_str());
    Ok(())
}

fn print_strings(s: &str) {
    println!("I've been asked to print {}", s);
}



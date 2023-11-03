
// Define uma tuple struct.
pub struct PhoneNumber(String);

// Implementa o tipo.
impl PhoneNumber {
    pub fn new(s: String) -> PhoneNumber {
        PhoneNumber(s)
    }
    pub fn as_str(&self) -> &str {
        // We didn't name the inner type, so it follows the same
        // naming convention as tuples. In other words, the inner
        // field is called `0`.
        &self.0
    }
}

fn main() {
    let num = PhoneNumber::new("555-1234".to_string());
    println!("{}", num.as_str())
}



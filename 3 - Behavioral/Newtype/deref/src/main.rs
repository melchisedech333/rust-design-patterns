
/// Note que usar Deref é recomendado apenas para ponteiros inteligentes, mas se
/// você quer algo semelhante, veja o exemplo as_str. Em todo caso, você pode 
/// utilizar Deref pois ele simplesmente retorna uma referencia ao seu tipo, e
/// faz com que seu tipo fique mais integrado na API e uso da linguagem.

use std::str::FromStr;
use std::ops::Deref;
use std::fmt;

pub struct PhoneNumber(String);

impl FromStr for PhoneNumber {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PhoneNumber(s.to_string()))
    }
}

// Implementa a Deref.
impl Deref for PhoneNumber {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num = PhoneNumber::from_str("555-1234")?;

    // Utiliza a deref.
    //
    // Deref can be called when we take a reference. The function
    // takes a &str and our type can Deref from &PhoneNumber to &str.
    print_strings(&num);

    println!("I've been asked to print {}", num);

    Ok(())
}

fn print_strings(s: &str) {
    // Note que usamos o atributo str diretamente aqui.
    println!("I've been asked to print {}", s);
}



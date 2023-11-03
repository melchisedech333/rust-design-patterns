
/// A derive more implementa automaticamente os métodos como Add, Sub, etc,
/// bastando especifica-los na própria derive.

use derive_more::{FromStr, Display, From, Add, Sub};
use std::ops::{Mul, Div};
use std::str::FromStr;

#[derive(FromStr, Display)]
pub struct PhoneNumber(String);

#[derive(Clone, Copy, From, Display, Add, Sub)]
#[display(fmt = "{} years", _0)]
pub struct Years(usize);

impl Mul for Years {
    type Output = Years;
    fn mul(self, rhs: Years) -> Years {
        Years(self.0.mul(rhs.0))
    }
}

impl Div for Years {
    type Output = Years;
    fn div(self, rhs: Years) -> Years {
        Years(self.0.div(rhs.0))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Exemplo 1.
    let num: PhoneNumber = "555-1234".parse()?;
    let num = PhoneNumber::from_str("555-1234")?;
    let num_as_string = num.to_string();
    println!("Phone number is {}", num);
    
    // Exemplo 2.
    let age_1 = Years::from(5);
    let age_2 = Years::from(2);

    println!("{} + {} = {}", age_1, age_2, age_1 + age_2);
    println!("{} - {} = {}", age_1, age_2, age_1 - age_2);
    println!("{} * {} = {}", age_1, age_2, age_1 * age_2);
    println!("{} / {} = {}", age_1, age_2, age_1 / age_2);
    
    Ok(())
}



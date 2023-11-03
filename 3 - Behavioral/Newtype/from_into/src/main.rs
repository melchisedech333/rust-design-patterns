
use std::fmt;

#[derive(Clone, Copy)]
pub struct Years(u32);

impl From<u32> for Years {
    fn from(val: u32) -> Years {
        Years(val)
    }
}

impl fmt::Display for Years {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    // We can call from directly
    let years = Years::from(10);
    println!("value: {}", years);

    // By implementing `From<u32> for Years`, we also get 
    // `Into<Years> for u32` for free!
    let years: Years = 10.into();
    println!("value: {}", years);
}



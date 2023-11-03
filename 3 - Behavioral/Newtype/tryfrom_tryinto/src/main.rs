
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Years(u32);

impl TryFrom<u64> for Years {
    type Error = &'static str;

    fn try_from(val: u64) -> Result<Years, Self::Error> {
        if val > u32::MAX as u64 {
            Err("Number out of range")
        } else {
            Ok(Years(val as u32))
        }
    }
}

impl fmt::Display for Years {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    // We can call from directly
    let years = Years::try_from(30 as u64);
    println!("value: {}", years.unwrap());

    // By implementing `From<u32> for Years`, we also get 
    // `Into<Years> for u32` for free!
    let error: Result<Years, &'static str> = u64::MAX.try_into();
    println!("value: {:?}", error);
}



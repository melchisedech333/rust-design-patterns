
use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Clone, Copy)]
pub struct Years(u32);

impl From<u32> for Years {
    fn from(val: u32) -> Years {
        Years(val)
    }
}

impl fmt::Display for Years {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} years", self.0)
    }
}

impl Add for Years {
    type Output = Years;
    fn add(self, rhs: Years) -> Years {
        Years(self.0.add(rhs.0))
    }
}

impl Sub for Years {
    type Output = Years;
    fn sub(self, rhs: Years) -> Years {
        Years(self.0.sub(rhs.0))
    }
}

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

fn main() {
    let age_1 = Years::from(5);
    let age_2 = Years::from(2);

    println!("{} + {} = {}", age_1, age_2, age_1 + age_2);
    println!("{} - {} = {}", age_1, age_2, age_1 - age_2);
    println!("{} * {} = {}", age_1, age_2, age_1 * age_2);
    println!("{} / {} = {}", age_1, age_2, age_1 / age_2);
}



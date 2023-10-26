
mod factory;
use factory::*;

fn main() {
    
    // Cliente 1.
    let factory = SedanFactory::new();
    let car = factory.make_car();
    println!("Car: {}", car.get_type());

    // Cliente 2.
    let factory = JdmFactory::new();
    let car = factory.make_car();
    println!("Car: {}", car.get_type());
}



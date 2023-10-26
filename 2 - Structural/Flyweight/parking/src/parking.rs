
/// Aqui, em conjunto com o Car, é realizado a parte do conceito referente ao campo do que 
/// é extrínseco, ou seja, dos dados mutáveis e que vão ser replicados na memória em muitas
/// instancias de objetos isolados.
/// 
/// Note que aqui fazemos a ligação das instâncias dos objetos dos carros, com as instâncias
/// dos recursos que possui natureza estática/intrínseca (Car Factory).

use crate::car_factory::CarFactory;
use crate::car_type::{Body, Colour};
use crate::car::Car;

pub struct Parking {

    // Armazena as instâncias dos objetos de natureza extrínseca.
    cars: Vec<Car>,

    // Armazena as instâncias dos objetos de natureza intrínseca (Flyweights).
    car_factory: CarFactory,
}

impl Parking {
    pub fn new() -> Parking {
        Parking {
            cars: Vec::new(),
            car_factory: CarFactory::new(),
        }
    }

    pub fn add_car(
        &mut self,
        license_plate: &str,
        parking_place_number: u8,
        body: Body,
        colour: Colour,
    ) {
        self.cars.push(Car::new(
            license_plate.to_string(),
            parking_place_number,
            self.car_factory.get_car_type_id(body, colour),
        ));
    }

    pub fn print(&mut self) {
        for car in &self.cars {
            let car_type = self.car_factory.get_car_type(car.car_type_id).unwrap();
            car.print(car_type);
        }

        println!("\nNumber of cars: {}", self.cars.len());
        self.car_factory.print();
    }
}

impl Default for Parking {
    fn default() -> Self {
        Self::new()
    }
}



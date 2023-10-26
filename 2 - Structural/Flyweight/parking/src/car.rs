
/// Este código trabalha em conjunto com o Parking, onde ambos realizam a parte do
/// conceito referente ao campo do que é extrínseco, ou seja, aqueles dados que vão
/// ser instanciados em objetos separados e em grande número na memória, e que são
/// mutáveis.

use crate::car_type::CarType;

pub struct Car {
    pub license_plate: String,
    pub parking_place_number: u8,
    pub car_type_id: u8,
}

impl Car {
    pub fn new(license_plate: String, parking_place_number: u8, car_type_id: u8) -> Car {
        Car {
            license_plate,
            parking_place_number,
            car_type_id,
        }
    }

    pub fn print(&self, car_type: &CarType) {
        println!(
            "{} - {:?} {:?}, {}",
            self.parking_place_number, car_type.colour, car_type.body, self.license_plate
        )
    }
}



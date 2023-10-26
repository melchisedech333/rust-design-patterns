
/// Implementação Concreta de um produto. O mesmo é utilizado internamente em
/// uma Builder Concreta, como por exemplo a CarManualBuilder.

use crate::products::car::components::{ CarType, Transmission, Engine, GpsNavigator };

pub struct Manual {
    car_type: CarType,
    seats: u16,
    engine: Engine,
    transmission: Transmission,
    gps_navigator: Option<GpsNavigator>,
}

impl Manual {
    pub fn new(
        car_type: CarType,
        seats: u16,
        engine: Engine,
        transmission: Transmission,
        gps_navigator: Option<GpsNavigator>,
    ) -> Self {
        Self {
            car_type,
            seats,
            engine,
            transmission,
            gps_navigator,
        }
    }
}

// Implementa a Trait Display para Manual, desta maneira é possível imprimir utilizando println!.
impl std::fmt::Display for Manual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\tType of car: {:?}", self.car_type)?;
        writeln!(f, "\tCount of seats: {}", self.seats)?;
        writeln!(
            f,
            "\tEngine: volume - {}; mileage - {}",
            self.engine.volume(),
            self.engine.mileage()
        )?;
        writeln!(f, "\tTransmission: {:?}", self.transmission)?;

        match self.gps_navigator {
            Some(_) => writeln!(f, "\tGPS Navigator: Functional")?,
            None => writeln!(f, "\tGPS Navigator: N/A")?,
        };

        Ok(())
    }
}



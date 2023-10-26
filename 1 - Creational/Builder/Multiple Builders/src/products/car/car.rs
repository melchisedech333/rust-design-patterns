
/// Implementação Concreta de um produto. O mesmo é utilizado internamente em
/// uma Builder Concreta, como por exemplo a CarBuilder.

use crate::products::car::components::{ CarType, Transmission, Engine, GpsNavigator };

pub struct Car {
    car_type: CarType,
    seats: u16,
    engine: Engine,
    transmission: Transmission,
    gps_navigator: Option<GpsNavigator>,
    fuel: f64,
}

impl Car {
    pub fn new(
        car_type: CarType,
        seats: u16,
        engine: Engine,
        transmission: Transmission,
        gps_navigator: Option<GpsNavigator>,
        fuel: f64,
    ) -> Self {
        Self {
            car_type,
            seats,
            engine,
            transmission,
            gps_navigator,
            fuel,
        }
    }

    // Setters.
    pub fn set_fuel(&mut self, fuel: f64) {
        self.fuel = fuel;
    }

    // Getters.
    pub fn car_type(&self) -> CarType {
        self.car_type
    }

    pub fn fuel(&self) -> f64 {
        self.fuel
    }

    pub fn seats(&self) -> u16 {
        self.seats
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn transmission(&self) -> &Transmission {
        &self.transmission
    }

    pub fn gps_navigator(&self) -> &Option<GpsNavigator> {
        &self.gps_navigator
    }
}


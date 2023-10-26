
/// Componente separado pois seu desenvolvimento é mais abrangente que
/// um mero tipo simples global. Note que o componente possui total
/// independência das outras partes do sistema que vão o utilizar.
/// Note também que os métodos manipulam de fato o Engine.

pub struct Engine {
    volume: f64,
    mileage: f64,
    started: bool,
}

impl Engine {
    pub fn new(volume: f64, mileage: f64) -> Self {
        Self {
            volume,
            mileage,
            started: false
        }
    }

    // Setters.
    pub fn on(&mut self) {
        self.started = true;
    }

    pub fn off(&mut self) {
        self.started = false;
    }

    // Getters.
    pub fn started(&self) -> bool {
        self.started
    }

    pub fn volume(&self) -> f64 {
        self.volume
    }

    pub fn mileage(&self) -> f64 {
        self.mileage
    }

    // Actions...
    pub fn go(&mut self, mileage: f64) {
        if self.started() {
            self.mileage += mileage;
        } else {
            println!("Cannot go(), you must start engine first!");
        }
    }
}



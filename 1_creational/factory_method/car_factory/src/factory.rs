
///
/// Fábrica abstrata.
/// 
pub trait CarFactory {

    // Factory Method.
    fn make_car(&self) -> Box<dyn Car>;
}

// Implementação concreta da Fábrica 1.
pub struct SedanFactory;

impl SedanFactory {
    pub fn new() -> Self {
        Self { }
    }
}

impl CarFactory for SedanFactory {
    fn make_car(&self) -> Box<dyn Car> {
        Box::new(Sedan::new("2023".to_string()))
    }
}

// Implementação concreta da Fábrica 2.
pub struct JdmFactory;

impl JdmFactory {
    pub fn new() -> Self {
        Self { }
    }
}

impl CarFactory for JdmFactory {
    fn make_car(&self) -> Box<dyn Car> {
        Box::new(Jdm::new("2022".to_string()))
    }
}

///
/// Produto abstrato.
/// 
pub trait Car {
    fn get_type(&self) -> String;
}

// Implementação concreta do Produto 1.
pub struct Sedan {
    model: String
}

impl Sedan {
    pub fn new(current_model: String) -> Self {
        Self {
            model: current_model
        }
    }
}

impl Car for Sedan {
    fn get_type(&self) -> String {
        format!("Sedan {}", self.model)
    }
}

// Implementação concreta do Produto 2.
pub struct Jdm {
    model: String
}

impl Jdm {
    pub fn new(current_model: String) -> Self {
        Self {
            model: current_model
        }
    }
}

impl Car for Jdm {
    fn get_type(&self) -> String {
        format!("JDM {}", self.model)
    }
}



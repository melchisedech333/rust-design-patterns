
/// Arquivo utilizado por tornar as importações dos módulos possíveis.

pub mod global;
pub mod engine;
pub mod gps;

pub use global::{ CarType, Transmission };
pub use engine::Engine;
pub use gps::GpsNavigator;



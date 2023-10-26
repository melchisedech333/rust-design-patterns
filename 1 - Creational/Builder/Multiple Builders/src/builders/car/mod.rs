
/// Arquivo utilizado por tornar as importações dos módulos possíveis.

pub mod builder;
pub mod car;
pub mod car_manual;

pub use builder::AbstractBuilder;
pub use car::CarBuilder;
pub use car_manual::CarManualBuilder;



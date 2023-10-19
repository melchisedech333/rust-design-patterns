
/// Builder abstrato. Ele pode ser utilizado por todos os tipos de produtos que o 
/// implementam concretamente, como por exemplo, o CarBuilder e o CarManualBuilder.

// Importa tipos dos componentes dos produtos, necessário para referenciar 
// nos métodos da Builder Abstrata. Métodos também abstratos que serão posteriomente
// implementados por Builders Concretas.
use crate::products::car::components::{ CarType, Transmission, Engine, GpsNavigator };

// Definição da Builder abstrata.
pub trait AbstractBuilder {
    type OutputType;

    // Métodos que configuram o produto. Todos são declarados de
    // maneira abstrata, pois cada Builder Concreto deve possuir
    // sua própria implementação.
    fn set_car_type(&mut self, car_type: CarType);
    fn set_seats(&mut self, seats: u16);
    fn set_engine(&mut self, engine: Engine);
    fn set_transmission(&mut self, transmission: Transmission);
    fn set_gsp_navigator(&mut self, gps_navigator: GpsNavigator);

    // Observe que é retornado um tipo genérico. Pois o Padrão Builder
    // permite que você trabalhe com tipos de produtos profundamente
    // diferente uns dos outros, podendo assim retornar produtos com
    // tipo específico.
    fn build(self) -> Self::OutputType;
}



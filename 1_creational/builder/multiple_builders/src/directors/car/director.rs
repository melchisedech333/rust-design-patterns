
/// Responsável por criar um Director para armazenar as etapas de montagem
/// de um determinado produto, o padrão Builder é interessante por permitir
/// criar produtos altamente customizáveis utilizando a mesma classe, abstraindo
/// assim a construção do produto para uma sequência de ações. O Director
/// é onde é armazenado essa sequência de ações.
/// 
/// Note que na assinatura dos métodos é sempre aceito alguma implementação
/// da Trait AbstractBuilder. Desse modo o mesmo Director pode ser utilizado
/// por uma variedade de implementações de Builders Concretos, uma vez que todos
/// eles implementam os métodos abstratos definidos na Trait AbstractBuilder. 
///
/// Outra característica interessante do Director é que ele nunca retorna um
/// produto concreto, pois quem deve fazer isso é a implementação de um Builder
/// concreto, pois assim é possível trabalhar com qualquer tipo de produto.
/// Ficando para o Director apenas o papel de receber uma instância de um
/// Builder concreto como parâmetro, e utilizá-lo/manuseá-lo.

use crate::builders::car::{ AbstractBuilder };
use crate::products::car::components::{ CarType, Transmission, Engine, GpsNavigator };

pub struct CarDirector;

impl CarDirector {
    pub fn construct_sports_car(builder: &mut impl AbstractBuilder) {
        builder.set_car_type(CarType::SportsCar);
        builder.set_seats(2);
        builder.set_engine(Engine::new(3.0, 0.0));
        builder.set_transmission(Transmission::SemiAutomatic);
        builder.set_gsp_navigator(GpsNavigator::new());
    }

    pub fn construct_city_car(builder: &mut impl AbstractBuilder) {
        builder.set_car_type(CarType::CityCar);
        builder.set_seats(2);
        builder.set_engine(Engine::new(1.2, 0.0));
        builder.set_transmission(Transmission::Automatic);
        builder.set_gsp_navigator(GpsNavigator::new());
    }

    pub fn construct_suv(builder: &mut impl AbstractBuilder) {
        builder.set_car_type(CarType::Suv);
        builder.set_seats(4);
        builder.set_engine(Engine::new(2.5, 0.0));
        builder.set_transmission(Transmission::Manual);
        builder.set_gsp_navigator(GpsNavigator::new());
    }
}



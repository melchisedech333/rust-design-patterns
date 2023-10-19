
// Define os módulos e as respectivas importações.
mod builders;
mod products;
mod directors;

use crate::builders::car::{ AbstractBuilder, CarBuilder, CarManualBuilder };
use crate::products::car::{ Car, Manual };
use crate::directors::car::{ CarDirector };

fn main() {

    //
    // Família de produtos: Carro
    // Produtos concretos: Carro e Manual do carro.
    // Produto específico: Sports Car.
    //

    // Instancia um Builder Concreto do carro, e passa ele para um Director.
    let mut car_builder = CarBuilder::default();
    CarDirector::construct_sports_car(&mut car_builder);

    // Retorna um Produto Concreto (Carro), construído pelo Builder Concreto, gestionado pelo Director.
    let car: Car = car_builder.build();
    println!("Car built: {:?}", car.car_type());

    // Instancia um Builder Concreto do manual (outro produto da mesma família de produtos).
    // E passa ele para o mesmo Director gestionar. Note que desta maneira a interface de uso
    // fica simplificada para o cliente, e a modificação/manutenção do código será feita sempre
    // nas implementações concretas dos builders e produtos, não necessitando alterar o Director
    // e nem afetar drasticamente na interface de uso do cliente, ao mesmo tempo que se mantem
    // a flexibilidade de poder utilizar produtos diferentes uns dos outros dentro da mesma
    // família de produtos.
    let mut manual_builder = CarManualBuilder::default();
    CarDirector::construct_sports_car(&mut manual_builder);
    
    // Retorna um Produto Concreto (Manual do carro), e exibe-o.
    let manual: Manual = manual_builder.build();
    println!("Car manual built:\n{}", manual);
}



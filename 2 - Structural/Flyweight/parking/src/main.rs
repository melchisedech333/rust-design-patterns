
/// Exemplo de código cliente que faz o uso do padrão. Observe que no lado do cliente
/// simplesmente criamos o Parking, e adicionamos carros com configurações diversas,
/// onde muitos possuem configurações repetidas. Segundo o conceitos não queremos que
/// essas informações repetidas fiquem duplicadas na memória, e para isso utilizamos
/// o Flyweight, onde realizamos a separação do que é extrínseco e intrínseco, e 
/// fazemos a ligação de uma coisa com a outra no momento devido, de modo que a
/// consequência é uma economia de recursos de memória RAM.

mod car;
mod car_type;
mod car_factory;
mod parking;

use parking::Parking;
use car_type::{
    Body::{Coupe, Sedan, Truck}, 
    Colour::{Black, Red, Yellow}
};

fn main() {

    let mut parking = Parking::new();
    parking.add_car("NFS MW", 13, Coupe, Red);
    parking.add_car("MW2017", 15, Truck, Yellow);
    parking.add_car("BIG11", 37, Sedan, Black);
    parking.add_car("KING", 64, Coupe, Red);
    parking.add_car("MAN", 73, Coupe, Red);
    parking.add_car("TE64G", 18, Truck, Yellow);
    parking.add_car("SMILE", 39, Sedan, Black);
    parking.add_car("DARK01", 24, Truck, Black);
    parking.add_car("DARK03", 25, Truck, Black);
    parking.add_car("DARK05", 26, Truck, Black);

    parking.print();
}



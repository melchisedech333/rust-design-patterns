
/// Realiza o controle dos decoradores, que realizam a ação de decorar os produtos concretos.

use crate::product::{Product};
use std::rc::Rc;

// Define classe abstrata do decorador. Note que assim como é possível possuir vários produtos 
// concretos, do mesmo modo é possível possuir vários decoradores concretos. Por isso é 
// necessário que exista uma trait para funcionar como classe abstrata para o Decorator.
//
// Note que também realizamos uma herança da trait Product, ligando assim o Decorator com o
// Product de algum modo. Em outras palavras podemos entender um Decorator como uma implementação
// concreta de um produto, mas que possui um caso de uso específico.
//
// Note também que definimos um método especial new(), onde o mesmo recebe como parâmetro um
// ponteiro Rc (Reference Counted, single-threaded reference-counting pointer) para qualquer 
// implementação da trait Product. E retornando uma instância do Decorator.
pub trait Decorator: Product {
    fn new(product: Rc<dyn Product>) -> Self;
}

// Define decorador concreto: decora estampa. Note que na struct existe um campo para armazenar 
// o product passado como parâmetro no método new() descrito acima. Método este que é implementado
// logo abaixo.
//
// Note que, se tratando o Decorator também de um produto concreto (de certo modo), então também
// realizamos a implementação do método get_value(), assim como para ProductBlueShirt e ProductWhiteShirt.
// Assim fazemos pois quando ocorre uma decoração especial no produto, o seu valor será alterado.
pub struct DecoratorShirtPrint {
    product: Rc<dyn Product>
}

impl Decorator for DecoratorShirtPrint {
    fn new(product: Rc<dyn Product>) -> Self {
        DecoratorShirtPrint {
            product
        }
    }
}

impl Product for DecoratorShirtPrint {
    fn get_value(&self) -> i32 {
        self.product.get_value() + 33
    }
}

// Outra implementação concreta de um Decorator, para expressar a flexibilidade do conceito.
// No caso sendo a customização de blusa bordada.
pub struct DecoratorShirtEmbroidered {
    product: Rc<dyn Product>
}

impl Decorator for DecoratorShirtEmbroidered {
    fn new(product: Rc<dyn Product>) -> Self {
        DecoratorShirtEmbroidered {
            product
        }
    }
}

impl Product for DecoratorShirtEmbroidered {
    fn get_value(&self) -> i32 {
        self.product.get_value() + 10
    }
}




// Remove warning de snake case.
#![allow(non_snake_case)]

use std::rc::Rc;

/// No padrão Decorator temos, normalmente, a nossa classe principal, que no caso
/// é o nosso Produto, que no nosso caso será uma camiseta. Nesse produtos iremos
/// adicionar várias decorações (sendo esses nossos Decorators).

mod product;
mod decorator;

use crate::product::{Product, ProductBlueShirt, ProductWhiteShirt};
use crate::decorator::{Decorator, DecoratorShirtPrint, DecoratorShirtEmbroidered};

fn main() {

    // Testa implementações dos produtos concretos.
    let product_blue_shirt_1 = ProductBlueShirt::new();
    let product_blue_shirt_2 = ProductBlueShirt::new();

    let product_white_shirt_1 = ProductWhiteShirt::new();
    let product_white_shirt_2 = ProductWhiteShirt::new();

    println!("Test concrete products:");
    println!("\tblue_shirt_1.: {}", product_blue_shirt_1.get_value());
    println!("\tblue_shirt_2.: {}", product_blue_shirt_2.get_value());
    println!("\twhite_shirt_1: {}", product_white_shirt_1.get_value());
    println!("\twhite_shirt_2: {}", product_white_shirt_2.get_value());

    // Testa implementações dos decoratores concretos - simple.
    println!("\nTest concrete decorators (simple):");

    let decorator_shirt_blue_print = DecoratorShirtPrint::new(Rc::new(product_blue_shirt_1.clone()));
    let decorator_shirt_blue_embroidered = DecoratorShirtEmbroidered::new(Rc::new(product_blue_shirt_1.clone()));

    let decorator_shirt_white_print = DecoratorShirtPrint::new(Rc::new(product_white_shirt_1.clone()));
    let decorator_shirt_white_embroidered = DecoratorShirtEmbroidered::new(Rc::new(product_white_shirt_1.clone()));

    println!("\tdecorator_shirt_blue_print.......: {}", decorator_shirt_blue_print.get_value());
    println!("\tdecorator_shirt_blue_embroidered.: {}", decorator_shirt_blue_embroidered.get_value());
    println!("\tdecorator_shirt_white_print......: {}", decorator_shirt_white_print.get_value());
    println!("\tdecorator_shirt_white_embroidered: {}", decorator_shirt_white_embroidered.get_value());

    // Testa implementações dos decoratores concretos - multi-layer.
    println!("\nTest concrete decorators (multi-layer):");

    let shirt_custom_1 = DecoratorShirtPrint::new(Rc::new(product_blue_shirt_1.clone()));
    let shirt_custom_1 = DecoratorShirtEmbroidered::new(Rc::new(shirt_custom_1));

    let shirt_custom_2 = DecoratorShirtPrint::new(Rc::new(product_white_shirt_1.clone()));
    let shirt_custom_2 = DecoratorShirtEmbroidered::new(Rc::new(shirt_custom_2));
    let shirt_custom_2 = DecoratorShirtEmbroidered::new(Rc::new(shirt_custom_2)); // ad-infinitum...

    println!("\tshirt_custom_1 (blue  + print + embroidered): {}", shirt_custom_1.get_value());
    println!("\tshirt_custom_2 (white + print + embroidered): {}", shirt_custom_2.get_value());
}



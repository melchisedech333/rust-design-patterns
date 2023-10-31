
#![allow(unused)]

mod visitor;

use visitor::{Visitor, TwoValuesStruct, TwoValuesArray};

/// O equivalente ao Element segundo o padrão Visitor. Note que temos uma classe
/// abstrata que será concretamente implementada por StringDeserializer e
/// VecDeserializer, cumprindo cada um o seu respectivo propósito.
/// 
/// `Deserializer` trait defines methods that can parse either a string or
/// a vector, it accepts a visitor which knows how to construct a new object
/// of a desired type (in our case, `TwoValuesArray` and `TwoValuesStruct`).
trait Deserializer<V: Visitor> {

    // Método especial correspondente ao nosso 'accept' segundo o conceito do
    // padrão Visitor. É por onde passaremos o objeto de nossa implementação
    // concreta da classe Visitor. E que será utilizada internamente pela
    // Deserializer (nossa classe Element).
    fn create(visitor: V) -> Self;
    
    fn parse_str(&self, input: &str) -> Result<V::Value, &'static str> {
        Err("parse_str is unimplemented")
    }
    fn parse_vec(&self, input: Vec<i32>) -> Result<V::Value, &'static str> {
        Err("parse_vec is unimplemented")
    }
}

/// Implementação concreta 1.
struct StringDeserializer<V: Visitor> {
    visitor: V,
}

impl<V: Visitor> Deserializer<V> for StringDeserializer<V> {
    fn create(visitor: V) -> Self {
        Self { visitor }
    }

    // Note que a entrada é uma string, ela é convertida e enviada como vetor
    // para a visit_vec(). Lembrando que em todos os casos nossas Visitors vão
    // invocar o método visit_vec(), mas o tipo retornado irá variador de acordo
    // com a implementação concreta da trait Visitor. Por exemplo, ao utilizar a
    // TwoValuesStruct, e invocarmos o método visit_vec(), teremos como retorno
    // uma estrutura contendo 2 elementos, diferente da TwoValuesArray que 
    // retornará um array contendo 2 elementos (o tipo é diferente).
    fn parse_str(&self, input: &str) -> Result<V::Value, &'static str> {

        // In this case, in order to apply a visitor, a deserializer should do
        // some preparation. The visitor does its stuff, but it doesn't do everything.
        let input_vec = input
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        Ok(self.visitor.visit_vec(input_vec))
    }
}

/// Implementação concreta 2.
struct VecDeserializer<V: Visitor> {
    visitor: V,
}

impl<V: Visitor> Deserializer<V> for VecDeserializer<V> {
    fn create(visitor: V) -> Self {
        Self { visitor }
    }

    fn parse_vec(&self, input: Vec<i32>) -> Result<V::Value, &'static str> {
        Ok(self.visitor.visit_vec(input))
    }
}

// Código cliente fazendo uso do Element e Visitor. Note que parse_str() e 
// parse_vec() tratam-se apenas da interface de uso da própria implementação
// concreta de Element (Deserializer<V>). Mas internamente as implementações dos
// métodos sempre acessam o método visit_vec() da instância da Visitor.
fn main() {
    let deserializer = StringDeserializer::create(TwoValuesStruct::default());
    let result = deserializer.parse_str("123 456");
    println!("{:?}", result);

    let deserializer = StringDeserializer::create(TwoValuesArray::default());
    let result = deserializer.parse_str("131 313");
    println!("{:?}", result);

    let deserializer = VecDeserializer::create(TwoValuesStruct::default());
    let result = deserializer.parse_vec(vec![123, 456]);
    println!("{:?}", result);

    let deserializer = VecDeserializer::create(TwoValuesArray::default());
    let result = deserializer.parse_vec(vec![123, 456]);
    println!("{:?}", result);

    println!(
        "Error: {}",
        deserializer.parse_str("123 456").err().unwrap()
    )
}



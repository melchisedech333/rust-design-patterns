
/// Exemplo de implementação da Visitor. Note que temos nossa classe abstrata
/// Visitor, que é implementada concretamente para TwoValuesStruct e
/// TwoValuesArray. Essas implementações concretas vão visitar o Element
/// definido na trait Deserializer<V: Visitor>. Note que nossas Visitors servem
/// para alterar o tipo de dado em nosso Element, retornando um tipo específico.

/// Visitor can visit one type, do conversions, and output another type.
///
/// It's not like all visitors must return a new type, it's just an example
/// that demonstrates the technique.
pub trait Visitor {
    type Value;

    /// Visits a vector of integers and outputs a desired type.
    fn visit_vec(&self, v: Vec<i32>) -> Self::Value;
}

/// Visitor implementation for a struct of two values.
/// Visita os dados de Element e retorna uma estrutura contendo 2 valores.
#[derive(Default, Debug)]
pub struct TwoValuesStruct {
    a: i32,
    b: i32,
}

impl Visitor for TwoValuesStruct {
    type Value = TwoValuesStruct;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value {
        TwoValuesStruct { a: v[0], b: v[1] }
    }
}

/// Visitor implementation for a struct of values array.
/// Visita os dados de Element e retorna um array contendo 2 valores.
#[derive(Default, Debug)]
pub struct TwoValuesArray {
    ab: [i32; 2],
}

impl Visitor for TwoValuesArray {
    type Value = TwoValuesArray;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value {
        let mut ab = [0i32; 2];

        ab[0] = v[0];
        ab[1] = v[1];

        TwoValuesArray { ab }
    }
}



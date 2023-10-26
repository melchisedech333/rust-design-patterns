
/// Essa classe corresponde a ConcreteFlyweight, ou seja, é uma implementação
/// concreta de um Flyweight, que deve ser utilizado em conjunto com a correspondente
/// de FlyweightFactory. Aqui é armazenado o dado que, em tese, consome muito recurso
/// de memória RAM ao ser duplicado em muitos objetos. Realizando assim a separação
/// dos dois campos intrínsecos e extrínsecos, sendo este parte do equivalente ao 
/// intrínseco, pois lida com os dados imutáveis/cached.

// Estrutura de dados que queremos que não seja replicada de modo descontrolado.
// O que implicaria em gastos significativos.
pub struct CarType {
    pub body: Body,
    pub colour: Colour,
}

// Simplesmente retorna uma instância de CarType (nosso ConcreteFlyweight).
impl CarType {
    pub fn new(body: Body, colour: Colour) -> CarType {
        CarType { body, colour }
    }
}

#[derive(Debug, PartialEq)]
pub enum Body {
    Sedan,
    Coupe,
    Truck,
}

#[derive(Debug, PartialEq)]
pub enum Colour {
    Black,
    Yellow,
    Red,
}




/// Armazena as implementações dos produtos.

// Definição da classe abstrata do Produto.
// Note que declaramos um método abstrato para retornar o valor do produto.
// Para cada implementação concreta do produto teremos um valor correspondente.
pub trait Product {
    fn get_value(&self) -> i32;
}

// Implementação concreta do produto: Blusa Azul.
#[derive(Clone)]
pub struct ProductBlueShirt {
    value: i32
}

impl ProductBlueShirt {
    pub fn new() -> Self {
        Self {
            value: 50
        }
    }
}

impl Product for ProductBlueShirt {
    fn get_value(&self) -> i32 {
        self.value
    }
}

// Implementação concreta do produto: Blusa Branca.
#[derive(Clone)]
pub struct ProductWhiteShirt {
    value: i32
}

impl ProductWhiteShirt {
    pub fn new() -> Self {
        Self {
            value: 100
        }
    }
}

impl Product for ProductWhiteShirt {
    fn get_value(&self) -> i32 {
        self.value
    }
}



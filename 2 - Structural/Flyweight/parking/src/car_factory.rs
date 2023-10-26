
/// É muito comum utilizar o Flyweight em conjunto com o Factory Method, em resumo
/// o intuito é possuir uma classe que possua um método para criação de Flyweights,
/// onde essa mesma classe realizará a centralização do acesso aos recursos 
/// armazenados em cache pelo padrão do próprio Flyweight. O que essa classe faz é
/// exatamente isso, ela é o correspondente a classe FlyweightFactory.
/// 
/// Note que essa parte do código corresponde ao campo intrínseco do conceito, onde
/// aqui fazemos o controle dos dados imutáveis.

use crate::car_type::{Body, CarType, Colour};

// Os Flyweights criados ficam armazenados em um vetor, empilhados um após o outro.
pub struct CarFactory {
    car_types: Vec<CarType>,
}

impl CarFactory {

    // Observe que a Fábrica é iniciada com zero Flyweights.
    pub fn new() -> CarFactory {
        CarFactory {
            car_types: Vec::new(),
        }
    }

    // Este método é o método fábrica, que corresponde ao método GetFlyweight().
    // O que ele faz é fundamental e é o que dá sentido ao próprio conceito do 
    // padrão Flyweight: ele verifica no vetor se já existe algum objeto em cache
    // que possa ser utilizado, caso exista ele retorna uma referência ao mesmo
    // para que ele possa ser utilizado por outras classes (essa referência pode
    // ser, por exemplo, um ID), caso não exista ele cria um novo objeto, armazena
    // no vetor e retorna o ID correspondente a esse novo objeto armazenado. 
    pub fn get_car_type_id(&mut self, body: Body, colour: Colour) -> u8 {

        // Realiza uma busca no vetor e verifica se existe algum item onde
        // o parâmetro 'body' e 'colour' sejam identicos aos passados via parâmetro.
        // Se existir então pega a posição do index no vetor (dentro de um Some(T)),
        // se não existir retorna um None.
        let position = self
            .car_types
            .iter()
            .position(|r| r.body == body && r.colour == colour);

        // Trata o retorno de position...
        match position {

            // Retorna o ID/index do item (Flyweight).
            Some(x) => x as u8, 

            // Como não existe nenhum item (Flyweight) correspondente, cria um novo e retorna
            // o seu ID.
            None => {
                let car_type = CarType::new(body, colour);
                self.car_types.push(car_type);
                (self.car_types.len() - 1) as u8
            }
        }
    }

    // Retorna um item do vetor de acordo com o ID/index.
    pub fn get_car_type(&mut self, id: u8) -> Option<&CarType> {
        self.car_types.get(id as usize)
    }

    // Exibe quantos itens estão em cache.
    pub fn print(&self) {
        println!("Number of car types: {}", self.car_types.len());
    }
}

impl Default for CarFactory {
    fn default() -> Self {
        Self::new()
    }
}



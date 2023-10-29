
/// Implementação do Mediator, o mesmo será utilizado para mediar a comunicação
/// entre os componentes.

use std::collections::{HashMap, VecDeque};

use crate::trains::Train;

// Interface abstrata do mediator. Note que ele possui seus 'notify methods', 
// eles são utilizados pelos componentes para comunicarem entre si. Por exemplo,
// para saber se existe trem na plataforma ou não, através do método 
// notify_about_arrival().
pub trait Mediator {
    fn notify_about_arrival(&mut self, train_name: &str) -> bool;
    fn notify_about_departure(&mut self, train_name: &str);
}

#[derive(Default)]
pub struct TrainStation {

    // Atributo para controle de todos os trens existentes na estação.
    // Ficando o nome do trem junto com o objeto instanciado.
    trains: HashMap<String, Box<dyn Train>>,

    // Atributos para controles internos da estação.
    train_queue: VecDeque<String>,
    train_on_platform: Option<String>,
}

impl Mediator for TrainStation {

    // Este método refere-se a chegada de um trem na plataforma de parada.
    // Quando um componente chama este método, ele verifica se já existe trem
    // estacionado na plataforma de parada dos trens. Se já existir, ele 
    // adiciona o trem numa fila (train_queue) e retorna false. Se não existir
    // nenhum trem parado na plataforma, ele diz que agora o trem em questão
    // está parado na plataforma, colocando seu nome em train_on_platform, e por
    // fim retornando true.
    fn notify_about_arrival(&mut self, train_name: &str) -> bool {
        if self.train_on_platform.is_some() {
            self.train_queue.push_back(train_name.into());
            false
        } else {
            self.train_on_platform.replace(train_name.into());
            true
        }
    }

    // Este método refere-se a saída do trem da plataforma de parada.
    // Ele verifica se o trem em questão (o componente que invocou o método),
    // está parado na plataforma, se estiver ele libera a plataforma.
    //
    // Em seguida ele pega o próximo trem na fila de trens (train_queue),
    // verificando se existe algum trem nessa mesma fila. Pegando esse trem, ele
    // é então movido para a plataforma de parada (pois o trem que estava acabou
    // de sair).
    //
    // Note que realizamos o uso do mediator para armazenar informações próprias
    // referente a intercomunicação entre os componentes, como também invocamos
    // métodos dos próprios componentes a partir de dentro do mediator, como é
    // o caso da linha: next_train.arrive(self). Expressando assim as duas 
    // peculiaridades do conceito do mediator (poder armazenar informações 
    // internamente, bem como invocar métodos dos objetos componentes para assim
    // realizar a comunicação entre os mesmos).
    fn notify_about_departure(&mut self, train_name: &str) {
        if Some(train_name.into()) == self.train_on_platform {
            self.train_on_platform = None;

            if let Some(next_train_name) = self.train_queue.pop_front() {
                let mut next_train = self.trains.remove(&next_train_name).unwrap();
                next_train.arrive(self);
                self.trains.insert(next_train_name.clone(), next_train);

                self.train_on_platform = Some(next_train_name);
            }
        }
    }
}

// Controle interno da estação, fornecendo uma interface de uso para o cliente.
// Note que é invocado os métodos internos dos componentes, e passado para
// parametros a eles o próprio mediator. Onde por sua vez, tais componentes
// vão internamente fazer o uso do mediator.
impl TrainStation {
    pub fn accept(&mut self, mut train: impl Train + 'static) {
        if self.trains.contains_key(train.name()) {
            println!("{} has already arrived", train.name());
            return;
        }

        train.arrive(self);
        self.trains.insert(train.name().clone(), Box::new(train));
    }

    pub fn depart(&mut self, name: &'static str) {
        let train = self.trains.remove(name);
        if let Some(mut train) = train {
            train.depart(self);
        } else {
            println!("'{}' is not on the station!", name);
        }
    }
}



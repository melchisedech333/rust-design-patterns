
/// O Editor faz o papel da classe que será monitorada pelo Observer.
/// Note que um de seus atributos é a classe Publisher, que é a classe que
/// controla todas as publicações. A mesma possui internamente uma lista de 
/// Subscribers.

use crate::observer::{Event, Publisher};

#[derive(Default)]
pub struct Editor {

    // Instância para classe Publisher, que possui lista de Subscribers.
    publisher: Publisher, 

    // Parâmetro de uso interno do Editor.
    file_path: String,
}

impl Editor {

    // Retorna referência para a instância da Publisher.
    pub fn events(&mut self) -> &mut Publisher {
        &mut self.publisher
    }

    // Os métodos abaixo exemplificam o uso da Observer, ou seja, ao serem
    // chamados, os métodos load e save vão gerar suas respectivas notificações.
    // Notificações estas que serão processadas pela classe Publisher, onde
    // serão invocados os devidos assinantes/subscribers.
    pub fn load(&mut self, path: String) {
        self.file_path = path.clone();
        self.publisher.notify(Event::Load, path);
    }

    pub fn save(&self) {
        self.publisher.notify(Event::Save, self.file_path.clone());
    }
}



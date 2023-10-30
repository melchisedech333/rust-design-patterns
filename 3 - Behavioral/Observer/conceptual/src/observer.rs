
/// Implementação concreta de um Observer (Publisher e Subscriber).

use std::collections::HashMap;

/// Tipo do evento (utilizado em várias partes da aplicação).
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Event {
    Load,
    Save,
}

/// Implementação concreta do Subscriber (assinante).
/// Define o assinante como um tipo função, isto é útil para chamarmos via
/// lambda functions, bem como passando o nome da função no registro dos
/// assinantes.
pub type Subscriber = fn(file_path: String);

/// Implementação concreta da Publisher (Publicadora).
#[derive(Default)]
pub struct Publisher {
    events: HashMap<Event, Vec<Subscriber>>, // Lista de assinantes.
}

impl Publisher {

    // Inscreve o assinante na lista de assinantes.
    pub fn subscribe(&mut self, event_type: Event, listener: Subscriber) {
        self.events.entry(event_type.clone()).or_default();
        self.events.get_mut(&event_type).unwrap().push(listener);
    }

    // Remove o assinante da lista de assinantes.
    pub fn unsubscribe(&mut self, event_type: Event, listener: Subscriber) {
        self.events
            .get_mut(&event_type)
            .unwrap()
            .retain(|&x| x != listener);
    }
    
    // Notifica o assinante de acordo com os parâmetros de invocação.
    // Note que quem define qual assinante deve ser invocado é o cliente (no 
    // caso o Editor). O evento é identificado na lista de assinantes e o
    // respectivo Subscriber inscrito é invocado.
    pub fn notify(&self, event_type: Event, file_path: String) {
        let listeners = self.events.get(&event_type).unwrap();
        for listener in listeners {
            listener(file_path.clone());
        }
    }
}



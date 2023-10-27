
/// Classe abstrata que representa um servidor. Isto é, um cliente irá utilizar 
/// alguma implementação concreta da mesma.

pub trait Server {
    fn handle_request(&mut self, url: &str, method: &str) -> (u16, String);
}



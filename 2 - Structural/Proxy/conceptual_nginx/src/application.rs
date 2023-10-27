
/// Implementação concreta da trait Server. Esta é a classe que o cliente 
/// pode utilizar diretamente, mas que será intercalada pelo Proxy.

use crate::server::Server;

pub struct Application;

impl Application {
    pub fn new() -> Self {
        Self { }
    }
}

impl Server for Application {
    fn handle_request(&mut self, url: &str, method: &str) -> (u16, String) {
        if url == "/app/status" && method == "GET" {
            return (200, "Ok".into());
        }

        if url == "/create/user" && method == "POST" {
            return (201, "User Created".into());
        }

        (404, "Not Ok".into())
    }
}



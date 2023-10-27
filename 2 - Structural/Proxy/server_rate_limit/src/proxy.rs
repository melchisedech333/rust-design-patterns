
/// Demonstração do conceito do Proxy.

use std::collections::HashMap;

use crate::server::Server;
use crate::application::Application;

// Assim como Application é uma implementação concreta de Server, o
// Proxy também é. Mas como estamos no Proxy, podemos fazer mais coisas.
// 
// IMPORTANTE: note que temos o campo 'application' que contem uma instância
// da classe Application. Isso é uma COMPOSIÇÃO, e um ponto importante do conceito.
// O Proxy realiza sua ações e no final o que ele faz é sempre chamar métodos de
// uma instância real do serviço, afinal, ele é apenas um Proxy e não uma implementação
// concreta de algo equivalente ao serviço. Por isso é necessário esse campo 'application'.
pub struct ProxyApplication {
    application: Application,

    // Campos para controle das ações do próprio Proxy.
    max_allowed_requests: i32,
    rate_limiter: HashMap<String, i32>,
}

impl ProxyApplication {
    pub fn new() -> Self {
        Self {
            application: Application,
            max_allowed_requests: 2,
            rate_limiter: HashMap::new()
        }
    }

    // Método para controle de funcionalidades próprias do Proxy.
    pub fn check_rate_limiting(&mut self, url: &str) -> bool {

        // Código que realiza o controle do rate limit.
        let mut found = false;
        let mut allow = true;

        for (path, requests) in &mut self.rate_limiter {
            if path.eq(&url.to_string()) {
                found = true;
                
                if *requests >= self.max_allowed_requests {
                    allow = false;
                } else {
                    *requests += 1;
                }

                break;
            }
        }

        if !found {
            self.rate_limiter.insert(url.to_string(), 1);
        }

        allow

        // Modo mais elegante:
        /*
        let rate = self.rate_limiter.entry(url.to_string()).or_insert(1);

        if *rate > self.max_allowed_requests {
            return false;
        }

        *rate += 1;
        true
        */
    }
}

impl Server for ProxyApplication {

    // Note que na implementação concreta simplesmente repassamos a chamada
    // para o mesmo método na instância 'application'. Pois somos apenas um
    // Proxy e não uma implementação concreta propriamente dita, da classe Server.
    fn handle_request(&mut self, url: &str, method: &str) -> (u16, String) {

        // Realiza o rate limit para expressar o conceito.
        if !self.check_rate_limiting(url) {
            return (403, "Not Allowed".into());
        }

        self.application.handle_request(url, method)
    }
}




/// Exemplo de uso no lado do cliente, note que é realizado a demonstração
/// com o uso direto de Application, e depois com o uso do Proxy, onde com 
/// o Proxy realizamos alterações para demonstrar a efetividade do Proxy.
/// Note que a interface de uso permanece a mesma.

mod server;
mod application;
mod proxy;

use server::Server;
use application::{Application};
use proxy::{ProxyApplication};

fn main() {

    // Exemplo de uso diretamente.
    println!("Application (direct):\n");
    let mut server = Application::new();
    test_object(&mut server);

    println!("+++++++++\n\nApplication (Proxy):\n");
    let mut proxy = ProxyApplication::new();
    test_object(&mut proxy);
}

fn test_object(server: &mut impl Server) {
    let app_status = &"/app/status".to_string();
    let create_user = &"/create/user".to_string();

    let (code, body) = server.handle_request(app_status, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

    let (code, body) = server.handle_request(app_status, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

    let (code, body) = server.handle_request(app_status, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

    let (code, body) = server.handle_request(create_user, "POST");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", create_user, code, body);

    let (code, body) = server.handle_request(create_user, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", create_user, code, body);
}



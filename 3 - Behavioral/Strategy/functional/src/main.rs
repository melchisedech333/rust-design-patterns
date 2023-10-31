
/// Abordagem utilizando tipo function ao invés de utilizar interfaces mais
/// complexas como no exemplo "conceptual_trait".

/// Declara o tipo segundo a assinatura de função que todas as estratégias
/// devem possuir para poder ser utilizadas.
type RouteStrategy = fn(from: &str, to: &str);

/// Implementa as estratégias concretas.
fn walking_strategy(from: &str, to: &str) {
    println!("Walking route from {} to {}: 4 km, 30 min", from, to);
}

fn public_transport_strategy(from: &str, to: &str) {
    println!(
        "Public transport route from {} to {}: 3 km, 5 min",
        from, to
    );
}

/// Exemplo de classe Context, onde o atributo interno (composição) da
/// estratégia é do tipo função anteriormente declarado.
struct Navigator {
    route_strategy: RouteStrategy,
}

impl Navigator {
    pub fn new(route_strategy: RouteStrategy) -> Self {
        Self { route_strategy }
    }

    // Invoca a estratégia como uma chamada a função. Note que ainda a classe
    // Context não conhece a estratégia concreta, apenas a utiliza segundo uma
    // interface de uso padrão.
    pub fn route(&self, from: &str, to: &str) {
        (self.route_strategy)(from, to);
    }
}

/// Exemplo de cliente utilizando as estratégias. Note que tudo que muda é 
/// somente o nome da estratégia, que desta vez corresponde ao nome de uma
/// função declarada, ou o uso de uma lambda function.
fn main() {
    let navigator = Navigator::new(walking_strategy);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");

    let navigator = Navigator::new(public_transport_strategy);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");

    let navigator = Navigator::new(|from, to| println!("Specific route from {} to {}", from, to));
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");
}



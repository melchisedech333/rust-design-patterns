
/// Interface abstrata das estratégias.
trait RouteStrategy {
    fn build_route(&self, from: &str, to: &str);
}

/// Implementação concreta da estratégia 1.
struct WalkingStrategy;

impl RouteStrategy for WalkingStrategy {
    fn build_route(&self, from: &str, to: &str) {
        println!("Walking route from {} to {}: 4 km, 30 min", from, to);
    }
}

/// Implementação concreta da estratégia 2.
struct PublicTransportStrategy;

impl RouteStrategy for PublicTransportStrategy {
    fn build_route(&self, from: &str, to: &str) {
        println!(
            "Public transport route from {} to {}: 3 km, 5 min",
            from, to
        );
    }
}

/// Classe Context, onde dentro dela ficará um atributo apontando para um objeto
/// que implementa a trait RouteStrategy.
struct Navigator<T: RouteStrategy> {
    route_strategy: T, // Composição.
}

impl<T: RouteStrategy> Navigator<T> {
    pub fn new(route_strategy: T) -> Self {
        Self { route_strategy }
    }

    // O Context não conhece a estratégia, apenas a usa.
    pub fn route(&self, from: &str, to: &str) {
        self.route_strategy.build_route(from, to);
    }
}

/// Exemplo de cliente fazendo uso de estratégias diversas. Note que o que muda
/// é tão somente a estratégia.
fn main() {
    let navigator = Navigator::new(WalkingStrategy);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");

    let navigator = Navigator::new(PublicTransportStrategy);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");
}



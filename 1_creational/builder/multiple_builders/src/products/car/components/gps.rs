
/// Noção equivalente ao da implementação Engine, mas com o intuito de 
/// demonstrar o uso de múltiplos componentes de natureza mais abragente.

pub struct GpsNavigator {
    route: String,
}

impl GpsNavigator {
    pub fn new() -> Self {
        Self::from_route(
            "221b, Baker Street, London  to Scotland Yard, 8-10 Broadway, London".to_string(),
        )
    }

    pub fn from_route(route: String) -> Self {
        Self { route }
    }

    pub fn route(&self) -> &String {
        &self.route
    }
}



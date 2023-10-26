
/// Implementação concreta da trait Component. Sendo o equivalente
/// ao conceito de galho da árvore.

use super::Component;

pub struct Folder {
    name: &'static str,
    components: Vec<Box<dyn Component>>, // Vetor para manipulação dinâmica que aceita como item/elemento
                                         // qualquer implementação da trait Component.
}

impl Folder {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            components: vec![],
        }
    }

    pub fn add(&mut self, component: impl Component + 'static) {
        self.components.push(Box::new(component));
    }
}

impl Component for Folder {
    fn search(&self, keyword: &str) {
        println!(
            "Searching recursively for keyword {} in folder {}",
            keyword, self.name
        );

        // Realiza busca recursiva dentro de components.
        for component in self.components.iter() {
            component.search(keyword);
        }
    }
}



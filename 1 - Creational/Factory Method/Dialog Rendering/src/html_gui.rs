
use crate::gui::{Button, Dialog};

// Controle da implementação concreta do produto (Button).
pub struct HtmlButton;

impl Button for HtmlButton {

    // Implementa o render e on_click declarados na interface.
    fn render(&self) {
        println!("<button>Test Button</button>");
        self.on_click();
    }

    fn on_click(&self) {
        println!("Click! Button says - 'Hello World!'");
    }
}

// Controle da implementação concreta do Creator/base (Dialog).
pub struct HtmlDialog;

impl Dialog for HtmlDialog {

    // Implementa o método fábrica de acordo com a implementação concreta do produto (Button).
    // Retornando uma nova instância de qualquer implementação da Trait Button, no caso HtmlButton.
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(HtmlButton)
    }
}



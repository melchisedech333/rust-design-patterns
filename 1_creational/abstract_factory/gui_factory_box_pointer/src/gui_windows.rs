
use crate::gui::{ Button, Checkbox, GuiFactory };

/// Implementação concreta da Fábrica.
pub struct WindowsFactory;

impl GuiFactory for WindowsFactory {

    // Factory Methods.
    fn create_button(&self, id: i32) -> Box<dyn Button> {
        Box::new(WindowsButton {
            id: id
        })
    }

    fn create_checkbox(&self, id: i32) -> Box<dyn Checkbox> {
        Box::new(WindowsCheckbox {
            id: id
        })
    }
}

/// Implementação concreta do Produto.
pub struct WindowsButton {
    id: i32
}

pub struct WindowsCheckbox {
    id: i32
}

impl Button for WindowsButton {
    fn press(&self) {
        println!("Windows button {} pressed.", self.id);
    }
}

impl Checkbox for WindowsCheckbox {
    fn switch(&self) {
        println!("Windows checkbox {} switched.", self.id);
    }
}



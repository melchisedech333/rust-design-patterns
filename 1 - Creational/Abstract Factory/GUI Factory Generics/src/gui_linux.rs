
use crate::gui::{ Button, Checkbox, GuiFactory };

/// Implementação concreta da Fábrica.
pub struct LinuxFactory;

impl GuiFactory for LinuxFactory {
    type B = LinuxButton;
    type C = LinuxCheckbox;

    // Factory Methods.
    fn create_button(&self, id: i32) -> LinuxButton {
        LinuxButton {
            id: id
        }
    }

    fn create_checkbox(&self, id: i32) -> LinuxCheckbox {
        LinuxCheckbox {
            id: id
        }
    }
}

/// Implementação concreta do Produto.
pub struct LinuxButton {
    id: i32
}

pub struct LinuxCheckbox {
    id: i32
}

impl Button for LinuxButton {
    fn press(&self) {
        println!("Linux button {} pressed.", self.id);
    }
}

impl Checkbox for LinuxCheckbox {
    fn switch(&self) {
        println!("Linux checkbox {} switched.", self.id);
    }
}



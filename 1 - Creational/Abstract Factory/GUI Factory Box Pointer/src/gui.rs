
/// Produtos abstratos.
pub trait Button {
    fn press(&self);
}

pub trait Checkbox {
    fn switch(&self);
}

/// Fábrica abstrata definida utilizando ponteiro Box.
/// Quando assim fizemos, o compilador usa "Dynamic dispatch".
pub trait GuiFactory {

    // Factory Methods.
    fn create_button(&self, id: i32) -> Box<dyn Button>;
    fn create_checkbox(&self, id: i32) -> Box<dyn Checkbox>;
}



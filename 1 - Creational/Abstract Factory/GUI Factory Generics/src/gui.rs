
/// Produtos abstratos.
pub trait Button {
    fn press(&self);
}

pub trait Checkbox {
    fn switch(&self);
}

/// Fábrica abstrata definida utilizando Generics.
/// Quando assim fizemos, o compilador usa "Static dispatch".
pub trait GuiFactory {
    type B: Button;
    type C: Checkbox;

    // Factory Methods.
    fn create_button(&self, id: i32) -> Self::B;
    fn create_checkbox(&self, id: i32) -> Self::C;
}



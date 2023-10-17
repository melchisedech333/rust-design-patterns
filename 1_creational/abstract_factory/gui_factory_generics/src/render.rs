
// Este código é para demonstrar que realizamos uma espécie de desacoplamento das 
// partes do sistema de modo que não dependemos de nenhuma implementação concreta 
// para utilizar os recursos das Fábricas e Produtos suportados/implementados.

use crate::gui::{GuiFactory, Button, Checkbox};

// Renders GUI. Factory object must be passed as a parameter to such the
// generic function with factory invocation to utilize static dispatch.
pub fn render(factory: impl GuiFactory) {

    // Exemplo de utilização dos métodos de fabricação.
    // Observe que eles podem fabricar quantas instâncias precisarmos.
    let button1 = factory.create_button(1);
    let button2 = factory.create_button(2);
    let checkbox1 = factory.create_checkbox(1);
    let checkbox2 = factory.create_checkbox(2);

    button1.press();
    button2.press();
    checkbox1.switch();
    checkbox2.switch();
}



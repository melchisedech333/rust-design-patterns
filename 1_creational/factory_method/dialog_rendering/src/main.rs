
mod gui;
mod html_gui;
mod windows_gui;
mod init;

use init::initialize;

fn main() {

    // Esta interface de uso funciona independente das implementações da Trait Dialog.
    // Também é possível realizar novas implementações da Trait abstrata Dialog sempre
    // que for preciso, mantendo assim a interface de uso. Dessa maneira a aplicação
    // fica bem flexível, suportando uma gama de modificações, tornando-a mais extensível.
    let dialog = initialize();
    dialog.render();
    dialog.refresh();
}



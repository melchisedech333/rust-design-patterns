
// Controle para determinar qual será o ambiente utilizado (Windows ou Linux).

mod gui;
mod gui_linux;
mod gui_windows;
mod render;

use gui::GuiFactory;
use gui_linux::LinuxFactory;
use gui_windows::WindowsFactory;
use render::render;

fn main() {
    let linux = true; // Controle pela flag.

    let factory: &dyn GuiFactory = if linux {
        &LinuxFactory
    } else {
        &WindowsFactory
    };

    render(factory);
}



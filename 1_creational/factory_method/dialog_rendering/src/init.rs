
use crate::gui::Dialog;
use crate::html_gui::HtmlDialog;
use crate::windows_gui::WindowsDialog;

// Responsável por controlar o ambiente em que será exibido a UI (Windows ou Web).
// É retornado qualquer implementação da Trait Dialog (WindowsDialog ou HtmlDialog).
pub fn initialize() -> &'static dyn Dialog {

    // A implementação de Dialog é retornada de acordo com as configurações do ambiente.
    // Em tese o modo para definir qual o ambiente se deve utilizar poderia se qualquer
    // um, aqui foi utilizado esse modo apenas para fins de exemplo.
    if cfg!(windows) {
        println!("-- Windows detected, creating Windows GUI --");
        &WindowsDialog
    } else {
        println!("-- No OS detected, creating the HTML GUI --");
        &HtmlDialog
    }
}




/// Implementação concreta do Command.

use cursive::{views::EditView, Cursive};

use super::Command;
use crate::AppContext;

#[derive(Default)]
pub struct CopyCommand;

impl Command for CopyCommand {

    // Implementação  concreta da execute(), note que cada implementação concre-
    // ta  da  trait Command terá que possuir a sua implementação concreta desse
    // método.
    fn execute(&mut self, app: &mut Cursive) -> bool {
        let editor = app.find_name::<EditView>("Editor").unwrap();
        let mut context = app.take_user_data::<AppContext>().unwrap();

        context.clipboard = editor.get_content().to_string();

        app.set_user_data(context);
        false
    }

    // Note que o comando Copy não faz nada em relação a undo, pois não é neces-
    // sário.
    fn undo(&mut self, _: &mut Cursive) {}
}



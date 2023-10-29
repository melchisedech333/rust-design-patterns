
/// Implementação concreta do Command.

use cursive::{views::EditView, Cursive};

use super::Command;
use crate::AppContext;

// Implementa  struct com atributo backup, para assim ser possível realizar a o-
// peração de undo (desfazer).
#[derive(Default)]
pub struct CutCommand {
    backup: String,
}

impl Command for CutCommand {
    fn execute(&mut self, app: &mut Cursive) -> bool {
        let mut editor = app.find_name::<EditView>("Editor").unwrap();

        app.with_user_data(|context: &mut AppContext| {
            self.backup = editor.get_content().to_string();
            context.clipboard = self.backup.clone();
            editor.set_content("".to_string());
        });

        true
    }

    // A undo puxa o backup do atributo da struct implementada.
    fn undo(&mut self, app: &mut Cursive) {
        let mut editor = app.find_name::<EditView>("Editor").unwrap();
        editor.set_content(&self.backup);
    }
}



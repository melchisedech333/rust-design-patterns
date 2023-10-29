
/// Código  cliente  fazendo uso do Command. Note que utilizamos a crate Cursive
/// para  implementar  uma janela, onde os elementos da janela faz o uso do con-
/// ceito do padrão Command.

mod command;

use cursive::{
    traits::Nameable,
    views::{Dialog, EditView},
    Cursive,
};

use command::{Command, CopyCommand, CutCommand, PasteCommand};

/// An application context to be passed into visual component callbacks.
/// It contains a clipboard and a history of commands to be undone.
/// 
/// Estrutura  de dados que é compartilhada entre os Command concretos implemen-
/// tados.
#[derive(Default)]
struct AppContext {
    clipboard: String,
    history: Vec<Box<dyn Command>>, // Vetor dinamicamente alocado.
}

fn main() {
    let mut app = cursive::default();

    app.set_user_data(AppContext::default());
    app.add_layer(
        Dialog::around(EditView::default().with_name("Editor"))
            .title("Type and use buttons")

            // Note como fazemos o uso da Command, utilizando a função execute()
            // que aqui da main.rs para chamar a execute() de nossos Commands.
            // Funcionando o método execute do Command como uma interface padro-
            // nizada.
            .button("Copy", |s| execute(s, CopyCommand::default()))
            .button("Cut", |s| execute(s, CutCommand::default()))
            .button("Paste", |s| execute(s, PasteCommand::default()))

            // Botão especial 'undo'.
            .button("Undo", undo)
            .button("Quit", |s| s.quit()),
    );

    app.run();
}

/// Executes a command and then pushes it to a history array.
fn execute(app: &mut Cursive, mut command: impl Command + 'static) {
    if command.execute(app) {

        // Todos os comandos executados tem sua ação salva em histórico.
        app.with_user_data(|context: &mut AppContext| {
            context.history.push(Box::new(command));
        });
    }
}

/// Pops the last command and executes an undo action.
/// 
/// Realiza o controle do histórico, manipulando o vetor dinamico.
fn undo(app: &mut Cursive) {
    let mut context = app.take_user_data::<AppContext>().unwrap();
    if let Some(mut command) = context.history.pop() {
        command.undo(app)
    }
    app.set_user_data(context);
}



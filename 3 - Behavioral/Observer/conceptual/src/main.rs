
/// Exemplo de cliente final utilizando uma classe (Editor) que faz o uso do
/// padrão Observer internamente para monitorar o seu estado interno.

use editor::Editor;
use observer::Event;

mod editor;
mod observer;

fn main() {
    let mut editor = Editor::default();

    // Exemplo de uso inscrevendo uma lambda function.
    editor.events().subscribe(Event::Load, |file_path| {
        let log = "/path/to/log/file.txt".to_string();
        println!("Save log to {}: Load file {}", log, file_path);
    });

    // Exemplo de uso inscrevendo uma função comum.
    editor.events().subscribe(Event::Save, save_listener);

    // Exemplo de uso das chamadas.
    editor.load("test1.txt".into());
    editor.load("test2.txt".into());
    editor.save();

    editor.events().unsubscribe(Event::Save, save_listener);
    editor.save();
}

fn save_listener(file_path: String) {
    let email = "admin@example.com".to_string();
    println!("Email to {}: Save file {}", email, file_path);
}



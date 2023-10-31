
/// Exemplo de uso em aplicação cliente final, no caso é usado em uma aplicação
/// TUI para exemplificar um player de áudio.

mod player;
mod state;

use cursive::{
    event::Key,
    view::Nameable,
    views::{Dialog, TextView},
    Cursive,
};
use player::Player;
use state::{State, StoppedState};

// Application context: a music player and a state.
// Estrutura de dados compartilhada pela TUI.
struct PlayerApplication {
    player: Player,
    state: Box<dyn State>,
}

fn main() {
    let mut app = cursive::default();

    // Instancia um objeto Player e inicia com o state contendo uma instância de
    // um objeto StoppedState.
    app.set_user_data(PlayerApplication {
        player: Player::default(),
        state: Box::new(StoppedState),
    });

    app.add_layer(
        Dialog::around(TextView::new("Press Play").with_name("Player Status"))
            .title("Music Player")
            .button("Play", |s| execute(s, "Play"))
            .button("Stop", |s| execute(s, "Stop"))
            .button("Prev", |s| execute(s, "Prev"))
            .button("Next", |s| execute(s, "Next")),
    );

    app.add_global_callback(Key::Esc, |s| s.quit());

    app.run();
}

fn execute(s: &mut Cursive, button: &'static str) {
    let PlayerApplication {
        mut player,
        mut state,
    } = s.take_user_data().unwrap();

    let mut view = s.find_name::<TextView>("Player Status").unwrap();

    // Here is how state mechanics work: the previous state
    // executes an action and returns a new state.
    // Each state has all 4 operations but reacts differently.
    state = match button {

        // Altera o comportamento a depender do estado atual. Lembrando que a
        // aplicação já inicia com o StoppedState.
        "Play" => state.play(&mut player),
        "Stop" => state.stop(&mut player),

        // As opções prev e next serão sempre as mesmas para todos os estados.
        "Prev" => state.prev(&mut player),
        "Next" => state.next(&mut player),
        _ => unreachable!(),
    };

    state.render(&player, &mut view);

    s.set_user_data(PlayerApplication { player, state });
}



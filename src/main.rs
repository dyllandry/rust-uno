mod card;
mod game;
mod user_input;
mod ui;

use game::Uno;
use user_input::get_user_input;

fn main() {
    let mut uno = Uno::new(1, 1);
    loop {
        uno.render();

        if let Some(user_input) = get_user_input() {
            uno.input(user_input);
        }

        if uno.game_over() {
            uno.render();
            break;
        }
    }
}

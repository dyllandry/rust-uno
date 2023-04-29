mod card;
mod game;
mod user_input;

use game::Uno;
use user_input::get_user_input;

fn main() {
    let mut uno = Uno::new(1,1);
    loop {
        uno.render();
        if uno.game_over() {
            break;
        }
        if let Some(user_input) = get_user_input() {
            uno.input(user_input);
        }
    }
}

mod card;
mod game;

use game::Game;

fn main() {
    let game = Game::new(2);
    game.render();
}

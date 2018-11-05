extern crate tictac;

use tictac::Game;

fn main() {
    let mut game = Game::new();
    // maybe start should return Result<(), E>?
    game.start();
}

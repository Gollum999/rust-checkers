mod frontend;
mod backend;

fn main() {
    let game = backend::Game::new();
    game.start();

    let window = frontend::window::Window::new(game.get_board());
}

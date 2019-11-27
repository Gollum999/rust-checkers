mod frontend;
mod backend;

fn main() {
    let game = backend::Game::new();
    let window = frontend::Window::new(game.get_board()).unwrap();

    game.start();
    window.run();
}

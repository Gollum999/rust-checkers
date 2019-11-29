mod backend;
mod channel;
mod frontend;

use std::thread;

fn main() {
    let (backend_endpoint, frontend_endpoint) = channel::make_two_way_channel();

    let render_thread = thread::spawn(move || {
        let window = frontend::Window::new(frontend_endpoint).unwrap();
        window.run();
    });

    let board = backend::Board::new();
    let game = backend::Game::new(&board, backend_endpoint);
    game.start();

    render_thread.join().unwrap();
}

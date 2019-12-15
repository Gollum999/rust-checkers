#[macro_use]
extern crate clap;

mod args;
mod backend;
mod channel;
mod frontend;

use std::thread;

fn main() {
    let (backend_args, frontend_args) = args::get_args();

    let (backend_endpoint, frontend_endpoint) = channel::make_two_way_channel();

    let render_thread = thread::spawn(move || {
        let mut window = frontend::Window::new(frontend_args, frontend_endpoint).unwrap();
        window.run();
    });

    let mut game = backend::Game::new(backend_args, backend_endpoint);
    game.start();

    render_thread.join().unwrap();
}

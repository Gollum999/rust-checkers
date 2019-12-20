#[macro_use]
extern crate clap;

mod args;
mod backend;
mod channel;
mod frontend;

use std::thread;

fn main() {
    let args = args::get_args();

    let (backend_endpoint, frontend_endpoint) = channel::make_two_way_channel();

    let render_thread = thread::spawn(move || {
        let mut window = frontend::CursesFrontend::new(args, frontend_endpoint);
        window.run().unwrap();
    });

    let mut game = backend::Game::new(args, backend_endpoint);
    game.start();

    render_thread.join().unwrap();
}

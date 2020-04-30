mod core;
mod cli;

use termion::event;
use termion::input::TermRead;
use std::io;
use std::thread;
use std::sync::mpsc;

enum Event {
    Input(event::Key)
}

// init
fn main() {
    let mut curstate: core::state::Game = core::state::init();

    // init user io
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();
    let mut buffer = String::new();

    // handle input thread
    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
        }
    });


    // main game loop
    loop {

        // redraw
        cli::update(&curstate);

        // get user input
        let mut comm: Option<String> = None;
        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => match input {
                event::Key::Char('\n') => {
                    comm = Some(buffer.drain(..).collect());
                }
                event::Key::Char(c) => {
                    buffer.push(c);
                }
                event::Key::Backspace => {
                    buffer.pop();
                }
                _ => {}
            },
        }

        // parse and run command
        if comm.is_some() {
            core::eval::exec(comm.unwrap(), &mut curstate);
        }
    }
}

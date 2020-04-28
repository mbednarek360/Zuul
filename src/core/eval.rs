use super::state;
use super::room;
use std::process;

pub fn exec(command: String, curstate: &mut state::Game, rooms: &Vec<room::Room>) {

    match &command[..] {
        "exit" => process::exit(0),
        _ => {}
    }


}
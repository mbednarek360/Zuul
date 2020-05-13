use super::state;
use super::room;
use std::process;

// evaluate and execute command
pub fn exec(command: String, curstate: &mut state::Game) -> Result<(), ()> {
    let command_vec: Vec<&str> = command.split_whitespace().collect::<Vec<&str>>();
    match command_vec[0] {
        "exit" => process::exit(0),
        "goto" => goto(command_vec, curstate),
        "redraw" => Ok(()),
        _ => Err(())
    }
}

// goto exit
fn goto(command_vec: Vec<&str>, curstate: &mut state::Game) -> Result<(), ()> {
    match command_vec[1] {
        "north" => curstate.goto(0),
        "south" => curstate.goto(1),
        "east" => curstate.goto(2),
        "west" => curstate.goto(3),
        _ => Err(())
    }
}

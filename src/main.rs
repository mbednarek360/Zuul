mod core;
mod tui;

// init
fn main() {
    let rooms: Vec<core::room::Room> = core::room::get_rooms();
    let mut curstate: core::state::Game = core::state::init(&rooms[0]);

    // main game loop
    loop {
        let command: Option<String> = tui::update(&curstate);
        if command.is_some() {
            core::eval::exec(command.unwrap(), &mut curstate, &rooms);
        }
    }
}

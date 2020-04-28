mod draw;
use super::core;
use termion::clear;





pub fn update(curstate: &core::state::Game) {

    // reset and draw frame
    print!("{}", clear::All);
    draw::draw_ui(curstate);
}
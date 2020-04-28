mod draw;
use super::core;
use termion::clear;

// reset and draw new frame
pub fn update(curstate: &core::state::Game) {
    draw::draw_ui(curstate);
    print!("{}", clear::All);
}
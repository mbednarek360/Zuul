mod draw;
use super::core;

pub fn update(curstate: &core::state::Game) -> Option<String> {

    // draw frame
    draw::draw_ui(curstate);

    // return input
    Some(String::new())


}
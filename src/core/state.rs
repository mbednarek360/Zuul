use super::room;
use std::io;
use tui::widgets::Text;

// hold text buffer as static
static BUF: Vec<Text> = Vec::new();

// hold mutable game variables
pub struct Game<'a> {
    rooms: Vec<room::Room>,
    room_id: usize,
    buffer: &'a Vec<Text<'a>>
}

// implementation for game
impl Game<'_> {

    // goto north south east west exit
    //      0     1     2    3 
    // will error if exit does not exist
    // current room is self.rooms[self.room_id]
    pub fn goto(&mut self, exit_id: usize) -> Result<(), ()> {
        let id: Option<usize> = self.rooms[self.room_id].get_exits()[exit_id];
        match id {
            Some(id) => Ok(self.room_id = id),
            None => Err(())
        }
    }

    // return text buffer for ui access
    pub fn get_buffer(&self) -> &Vec<Text> {
        self.buffer
    }
}

// setup new game
pub fn init() -> Game<'static> {
    Game {
        rooms: room::get_rooms(),
        room_id: 0,
        buffer: &BUF
    }
} 
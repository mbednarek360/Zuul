use super::room;
use std::io;

pub struct Game {
    rooms: Vec<room::Room>,
    room_id: usize,
    buffer: String
}

impl Game {

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
}

// setup new game
pub fn init() -> Game {
    Game {
        rooms: room::get_rooms(),
        room_id: 0,
        buffer: String::new()
    }
} 
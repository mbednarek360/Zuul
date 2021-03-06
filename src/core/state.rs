use super::room;
use super::item;
use std::io;
use tui::widgets::Text;

// hold mutable game variables
pub struct Game<'a> {
    rooms: Vec<room::Room>,
    room_id: usize,
    buffer: Vec<Text<'a>>,
    items: Vec<item::Item>
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
            Some(id) => {
                self.room_id = id;
                self.buffer.push(Text::raw("Moved!\n"));
                Ok(())
            },
            None => {
                self.buffer.push(Text::raw("No exit!\n"));
                Err(())
            }
        }
    }

    // return text buffer for ui access
    pub fn get_buffer(&self) -> &Vec<Text> {
        &self.buffer
    }

    // return current room
    pub fn get_room(&self) -> &room::Room {
        &self.rooms[self.room_id]
    }
}

// setup new game
pub fn init() -> Game<'static> {
    Game {
        rooms: room::get_rooms(),
        room_id: 0,
        buffer: Vec::new(),
        items: Vec::<item::Item>::new()
    }
} 
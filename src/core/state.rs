use super::room;

pub struct Game<'a> {
    room: &'a room::Room,
    buffer: String
}

pub fn init(startroom: &room::Room) -> Game {
    Game {
        room: startroom,
        buffer: String::new()
    }
} 
use yaml_rust::{YamlLoader, Yaml};

// holds values for each room
pub struct Room {
    id: u16,
    name: String,
    message: String,
    exits: [Option<usize>; 4]
}

// implementations for room struct
impl Room {
    pub fn to_string(&self) -> String {
        format!("{{id: {}, name: \"{}\", message: \"{}\", exits: {:?}}}", 
            self.id, self.name, self.message, self.exits)
    }

    pub fn get_exits(&self) -> [Option<usize>; 4] {
        return self.exits;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

// parses rooms from yaml
pub fn get_rooms() -> Vec<Room> {

    // init output
    let mut outp: Vec<Room> = Vec::new();

    // init and parse yaml from compile time config file
    let world = &YamlLoader::load_from_str(include_str!("../world.yml")).unwrap()[0];

    // build array and construct rooms
    for room in world["rooms"].clone() {
        outp.push( Room {
            id: room["id"].as_i64().unwrap() as u16,
            name: room["name"].as_str().unwrap().to_string(),
            message: room["message"].as_str().unwrap().to_string(),
            exits: [
                room["exits"]["north"].as_i64().map(|i| i as usize),
                room["exits"]["south"].as_i64().map(|i| i as usize),
                room["exits"]["east"].as_i64().map(|i| i as usize),
                room["exits"]["west"].as_i64().map(|i| i as usize)
            ]
        })
    }

    // return
    outp
}
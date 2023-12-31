
use super::game::{MazeGame, Room};

// Implementação do produto/Room.
#[derive(Clone)]
pub struct OrdinaryRoom {
    id: u32,
}

impl OrdinaryRoom {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

impl Room for OrdinaryRoom {
    fn render(&self) {
        println!("Ordinary Room: #{}", self.id);
    }
}

// Implementação da fábrica/creator/base.
pub struct OrdinaryMaze {
    rooms: Vec<OrdinaryRoom>,
}

impl OrdinaryMaze {
    pub fn new() -> Self {
        Self {
            rooms: vec![OrdinaryRoom::new(1), OrdinaryRoom::new(2)],
        }
    }
}

impl MazeGame for OrdinaryMaze {
    type RoomImpl = OrdinaryRoom;

    fn rooms(&self) -> Vec<Self::RoomImpl> {
        let mut rooms = self.rooms.clone();
        rooms.reverse();
        rooms
    }
}



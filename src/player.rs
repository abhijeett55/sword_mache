use crate::coord::{Coord, Direction};
use crate::floor::Floor;

pub struct Player {
    pub coord: Coord,
    pub facing: Direction,
    pub sword_coord: Coord,
    pub symbol: &'static str,
    pub dirty: bool,
    pub score: u64,
}


impl Player {
    pub fn new(coord: Coord ) -> Self{
        Self {
            coord,
            facing: Direction::Right,
            sword_coord: coord.to_the(Direction::Right),
            symbol: "0",
            dirty: true,
            score: 0,
        }
    }

    pub fn sword_symbol(&self) -> &'static str {
        match self.facing {
            Direction::Up => "⤉",
            Direction::Down => "⤈",
            Direction::Left => "↢",
            Direction::Right => "↣",
        }
    }


    pub fn travel (
        &mut self,
        direction: Direction,
        floor: &Floor,
        dirty_coords: &mut Vec<Coord>,

        ) -> bool {
        let mut moved = false;

        if self.facing != direction {
            self.dirty = true;
            dirty_coords.push(self.sword_coord);
            self.facing = direction;
        } else {
            let to_coord = self.coord.to_the(self.facing);

            if !floor.is_wall(to_coord) {
                self.dirty = true;
                moved = true;
                dirty_coords.push(self.coord);
                dirty_coords.push(self.sword_coord);
                self.coord = to_coord;
            }
        }
        self.sword_coord = self.coord.to_the(self.facing);
        moved
    }
}
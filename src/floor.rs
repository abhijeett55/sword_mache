use crate::coord::Coord;

pub struct Floor {
    pub rows: usize,
    pub cols: usize,
    pub tiles: Vec<Vec<&'static str>>,
}


impl Floor {
    pub fn new(rows: usize, cols: usize) -> Self {
        let horizonrtal = "-";
        let vertical = "|";
        let top_left = "┌";
        let top_right = "┐";
        let bottom_left = "└";
        let bottom_right = "┘";
        let blank = " ";


        let mut tiles = Vec::<Vec<&'static str>>::with_capacity(rows);
        for _ in 0..rows {
            tiles.push(Vec::<&'static str>::with_capacity(cols));
        }


        tiles[0].push(top_left);
        for _ in 0..cols -1 {
            tiles[0].push(horizonrtal);
        }

        tiles[0].push(top_right);
        for row in 1..rows - 1{
            tiles[row].push(vertical);

            for _ in 1..cols -1  {
                tiles[row].push(blank);
            }
            tiles[row].push(vertical);
        }

        tiles[rows - 1].push(bottom_left);
        for _ in 1..cols- 1 {
            tiles[rows - 1].push(horizonrtal);
        }

        tiles[rows - 1].push(bottom_right);

        Self { rows, cols, tiles }

    }


    pub fn get_symbol(&self, coord:Coord) -> &'static str {
        self.tiles[coord.row][coord.col]
    }
    pub fn is_wall(&self , coord: Coord) -> bool {
        self.tiles[coord.row][coord.col] != " "
    }

}
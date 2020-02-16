
pub enum Tile {
    Empty,
    Value(usize),
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Board {
    width: usize,
    height: usize,
    data: Vec<Vec<Tile>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {

        // initialize the data with empty tiles
        let mut data = Vec::with_capacity(height);
        for ri in 0..height {
            let mut row = Vec::with_capacity(width);
            for ci in 0..width {
                row.push(Tile::Empty);
            }
            data.push(row);
        }

        Board {
            width: width,
            height: height,
            data: data,
        }
    }

    pub fn new_square(size: usize) -> Board {
        Board::new(size, size)
    }

    pub fn is_within(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.data[y][x] = tile;
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.data[y][x]
    }

    pub fn make_move(&mut self, dir: Direction) -> bool {

        let mut has_changed = false;

        let dir_vec = match dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            _ => (0, 0)
        };

        // if the move vector is zero then nothing will occur on the game board
        if dir_vec == (0, 0) {
            return false;
        }

        // pull out dx, dy
        let (dx, dy) = dir_vec;

        // iterate over all tiles moving them
        let mut xi;
        let mut xlb;
        let mut xub =  ;
        let mut xinc = if (dx > 0) {1} else {-1};
        
        let mut yi;
        let mut ylb;
        let mut yub;
        let mut yinc = if (dy > 0) {1} else {-1};

        for txi in 0..self.width {
            for tyi in 0..self.height {
                
                // flip the ranges to go the opposite way for the moveing/merging
                xi = if dir_vec.0 < 0 { self.width - txi - 1 } else { txi };
                yi = if dir_vec.1 < 0 { self.height - tyi - 1 } else { tyi };

                // ignore last row/col
                
                
                // 
                match self.get_tile(xi, yi) {
                    Tile::Value(v) => {},
                    _ => {},
                }


            }
        }

        return has_changed;
    }

    pub fn print(&self) {

        let mut character: &str;
        for yi in 0..self.height {
            for xi in 0..self.width {
                
                character = match self.get_tile(xi, yi) {
                    Tile::Empty => "_",
                    Tile::Value(v) => "A",
                    _ => "?",
                };
                print!("{}", character);
                if xi != self.width - 1 {
                    print!(" ");
                }
            }
            println!("");
        } 
    }
}
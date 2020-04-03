extern crate rand;

use crate::iterrange::{ IteratorRange };
use rand::{ Rng };
use rand::rngs::{ ThreadRng };

#[derive(Copy, Clone)]
pub enum Tile {
    Empty,
    Block,
    Value(usize),
}

impl Tile {
    pub fn is_empty(&self) -> bool {
        return match self {
            Tile::Empty => true,
            _ => false,
        };
    }

    pub fn to_character(&self) -> char {
        return match self {
            Tile::Empty => '_',
            Tile::Block => '#',
            Tile::Value(v) => (v.to_owned() as u8 + 64) as char,
        };
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        return match (self, other) {
            (Tile::Empty, Tile::Empty) => true,
            (Tile::Block, Tile::Block) => true,
            (Tile::Value(u), Tile::Value(v)) => (u == v),
            _ => false,
        }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_string(&self) -> &str {
        return match self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
        };
    }
}

pub struct Board {
    width: usize,
    height: usize,
    rng: ThreadRng,
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
            rng: rand::thread_rng(),
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

    pub fn pickup_tile(&mut self, x: usize, y: usize) -> Tile {
        let tile: Tile = self.get_tile(x, y).clone();
        match tile {
            Tile::Empty => return Tile::Empty,
            _ => self.set_tile(x, y, Tile::Empty),
        }
        return tile;
    }

    pub fn add_random_tile(&mut self) {

        // get random coord of empty
        let mut tx: usize;
        let mut ty: usize;
        'find_empty: loop {

            tx = self.rng.gen_range(0, self.width);
            ty = self.rng.gen_range(0, self.height);

            match self.get_tile(tx, ty) {
                Tile::Value(_) => continue,
                _ => break 'find_empty,
            }
        }

        // place tile
        self.set_tile(tx, ty, Tile::Value(1));
    }

    pub fn make_move(&mut self, dir: Direction) -> bool {

        let mut has_changed = false;

        let dir_vec = match dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        // pull out dx, dy
        let (dx, dy) = dir_vec;

        let mut x_iter = IteratorRange::new(0, (self.width as isize) - 1);
        let mut y_iter = IteratorRange::new(0, (self.height as isize) - 1);

        // used to iterate backward over the tiles
        match dir {
            Direction::Right => {
                x_iter.reverse();
            },
            Direction::Down => {
                y_iter.reverse();
            },
            _ => {},
        }

        // create a matrix to keep track of which peices have been merged
        let mut been_merged = vec![vec![false; self.width]; self.height];

        // iterate over the board where there
        // could be peices that could be moved
        for txi in x_iter.clone() {
            for tyi in y_iter.clone() {

                // pick up the tile and replace with empty
                let tile = self.pickup_tile(txi as usize, tyi as usize);
                let value: usize;
                match tile {

                    // if the picked up tile is empty skip it
                    Tile::Empty => continue,

                    // pull the value out otherwise
                    Tile::Value(v) => {
                        value = v;
                    },

                    Tile::Block => {
                        self.set_tile(txi as usize, tyi as usize, tile);
                        continue;
                    },
                }

                
                // attempt to move the peice until placement
                let mut dxi = txi;
                let mut dyi = tyi;
                'move_check: loop {
                    
                    let next_xi = (dxi + dx) as usize;
                    let next_yi = (dyi + dy) as usize;

                    // if the next space is off the board halt the movement
                    // and place the tile where it stopped
                    if !self.is_within(dxi + dx, dyi + dy) {
                        self.set_tile(dxi as usize, dyi as usize, tile);
                        break 'move_check;
                    }
                    
                    // if the next space is another tile
                    let next_tile = self.get_tile(next_xi, next_yi);
                    match next_tile {

                        // if the next tile is empty continue checking
                        Tile::Empty => {
                            
                            // move to next
                            dxi += dx;
                            dyi += dy;

                            // update change
                            has_changed = true;
                        },

                        // if the next tile is not empty
                        Tile::Value(next_value) => {

                            // if the next has the same value
                            // and hasnt been formed from a merge this turn
                            if (next_value == &value) && (!been_merged[next_yi][next_xi]) {
                                
                                // mark as merged
                                been_merged[next_yi][next_xi] = true;

                                // replace with incremented tile
                                self.set_tile(next_xi, next_yi, Tile::Value(value + 1));

                                // update change
                                has_changed = true;
                            
                            } else {

                                // otherwise place it normally
                                self.set_tile(dxi as usize, dyi as usize, tile);
                            }

                            break 'move_check;
                        },

                        // if the next tile is a block place it before normally
                        Tile::Block => {
                            self.set_tile(dxi as usize, dyi as usize, tile);
                            break 'move_check;
                        }

                    }

                }
            }
        }

        return has_changed;
    }

    pub fn take_turn(&mut self, dir: Direction) -> bool {
        let has_changed = self.make_move(dir);
        if has_changed {
            self.add_random_tile();
        }
        return has_changed;
    }

    pub fn print(&self) {

        for yi in 0..self.height {
            for xi in 0..self.width {
                print!("{}", self.get_tile(xi, yi).to_character());

                if xi != self.width - 1 {
                    print!(" ");
                }
            }
            println!("");
        } 
    }

    pub fn take_verbose_turn(&mut self, dir: Direction) {
        print!("Made move: {}", dir.to_string());
        let result: bool = self.take_turn(dir);
        print!(" | Greatest value: {}", self.greatest_tile_value());
        println!(" | Has changed: {}", result);
        self.print();
    }

    pub fn greatest_tile_value(&self) -> usize {

        let mut largest: usize = 0;

        for xi in 0..self.width {
            for yi in 0..self.height {
                match self.get_tile(xi, yi) {
                    Tile::Value(v) => {
                        if v > &largest {
                            largest = v.to_owned();
                        }
                    },
                    _ => {}
                }
            }
        }

        return largest;
    }

    //todo: implement can_move (using flood search top to btm maybe)
    pub fn can_move(&self) -> bool {
        for xi in 0..self.width {
            for yi in 0..self.height {

                // get the current tile
                let tile = self.get_tile(xi, yi);                
                match tile {

                    // if the current tile is a value tile check:
                    // ...
                    // .cx
                    // .x.
                    Tile::Value(v) => {
                        
                        let mut tile_over: &Tile;
                        if self.is_within(xi as isize + 1, yi as isize){
                            tile_over = self.get_tile(xi+1, yi);
                            if tile_over == tile {
                                return true;
                            }
                        }
                        if self.is_within(xi as isize, yi as isize + 1){
                            tile_over = self.get_tile(xi, yi+1);
                            if tile_over == tile {
                                return true;
                            }
                        }
                    },
                    // if there is an empty tile there is a possible move
                    Tile::Empty => {
                        return true;
                    }
                    _ => continue,
                }
            }
        }
        return false;
    }
}

enum Tile {
    Empty,
    Value(usize),
}

struct Board {
    data: Vec<Vec<Tile>>,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {

        let mut data = Vec::with_capacity(height);
        for ri in 0..height {
            let mut row = Vec::with_capacity(width);
            for ci in 0..width {
                row.push(Tile::Empty);
            }
            data.push(row);
        }


        Board {
            data: data,
        }
    }
}
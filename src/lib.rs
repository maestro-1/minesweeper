use std;
use std::{collections::HashSet, fmt::Write, fmt::Display};
mod random;
pub use crate::random::random_range;

type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8),
}

pub struct MineSweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
}


impl Display for MineSweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x,y);

                if !self.open_fields.contains(&pos){
                    f.write_str("⬛️")?;    
                } else if self.mines.contains(&pos) {
                    f.write_str("💣")?;
                } else {
                    write!(f, " {} ", self.neighbouring_min(pos))?;
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl MineSweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> MineSweeper {
        let mut mines = HashSet::new();
        while mines.len() < mine_count {
            mines.insert((random_range(0, width), random_range(0, height)));
        };

        MineSweeper {
            width,
            height,
            open_fields: HashSet::new(),
            flagged_fields: HashSet::new(),
            mines,
        }
    }

    fn iter_neighbour(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;

        return (x.min(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.min(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y));
    }

    fn neighbouring_min(&self, pos: Position) -> usize {
        self.iter_neighbour(pos)
            .filter(|pos| self.mines.contains(pos)).count() 
    }

    pub fn open(&mut self, position: Position) -> OpenResult {
        self.open_fields.insert(position);
        let is_min = self.mines.contains(&position);

        if is_min {
            return OpenResult::Mine;
        }
        OpenResult::NoMine(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::MineSweeper;

    #[test]
    fn test() {
        let mut ms = MineSweeper::new(10, 10, 5);
        ms.open((5,5));
        print!("{}", ms)
    }
}

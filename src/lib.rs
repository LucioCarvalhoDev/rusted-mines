use std::collections::HashSet;

mod msgr;
mod random;
use random::random_range;

pub type Position = (usize, usize);
pub enum OpenResult {
    Mine,
    Near(u8),
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();

                while mines.len() < mine_count {
                    mines.insert((random_range(0, width), random_range(0, height)));
                }

                mines
            },
            flagged_fields: HashSet::new(),
        }
    }

    pub fn open(&self, pos: &Position) -> OpenResult {
        if self.mines.contains(pos) {
            OpenResult::Mine
        } else {
            // println!("sem mina: {:?}", pos);
            OpenResult::Near(self.count_surround_mines(pos))
        }
    }

    fn count_surround_mines(&self, pos: &Position) -> u8 {
        const NEIGHBOR_POSITIONS: [(isize, isize); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let mut n: u8 = 0;

        for relative_neigh in NEIGHBOR_POSITIONS {
            let abs_x = (pos.0 as isize) + relative_neigh.0;
            if abs_x < 0 || abs_x >= (self.width as isize) {
                continue;
            }

            let abs_y = (pos.1 as isize) + relative_neigh.1;
            if abs_y < 0 || abs_y >= (self.height as isize) {
                continue;
            }

            let neighbor: Position = (abs_x as usize, abs_y as usize);

            if self.mines.contains(&neighbor) {
                n = n + 1;
            }
        }

        n
    }
}

#[cfg(test)]
mod tests {
    use crate::{msgr::cin, Minesweeper, OpenResult, Position};

    fn init(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper::new(width, height, mine_count)
    }

    #[test]
    fn mines_generation() {
        let ms = init(4, 4, 2);
        println!("{:?}", ms.mines);
    }

    #[test]
    fn open_tile() {
        let ms = init(3, 3, 2);

        println!("Minas: {:?}", ms.mines);

        let mut x = String::new();
        let mut y = String::new();
        cin("Insira um valor x:".to_string(), &mut x);
        cin("Insira um valor y:".to_string(), &mut y);
        let x: usize = x.trim().parse().unwrap();
        let y: usize = y.trim().parse().unwrap();

        let pos: Position = (x, y);

        match ms.open(&pos) {
            OpenResult::Mine => println!("Mine"),
            OpenResult::Near(n) => println!("Nearby mines: {}", n),
        }
    }

    #[test]
    fn draw_map() {
        let ms = init(8, 5, 5);

        println!("Minas: {:?}", ms.mines);

        for row in 0..ms.height {
            let mut line = String::new();

            for col in 0..ms.width {
                let pos: Position = (col, row);
                match ms.open(&pos) {
                    OpenResult::Mine => {
                        line.push_str(&format!(" # "));
                        //format!(" ({},{}) ", col, row);
                    }
                    OpenResult::Near(q) => {
                        line.push_str(&format!(" {} ", q));
                        //println!(" ({},{}) ", col, row);
                    }
                }
            }

            println!("{}", line);
        }
    }
}

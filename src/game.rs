use std::{array::from_fn, collections::HashMap};
use lazy_static::lazy_static;
use rand;

lazy_static! {
    static ref ORDERS: HashMap<i8, [[usize; 4]; 4]> = {
        HashMap::from(
            [
                (1, [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]]),
                (-1, [[3, 2, 1, 0], [7, 6, 5, 4], [11, 10, 9, 8], [15, 14, 13, 12]]),
                (4, [[0, 4, 8, 12], [1, 5, 9, 13], [2, 6, 10, 14], [3, 7, 11, 15]]),
                (-4, [[12, 8, 4, 0], [13, 9, 5, 1], [14, 10, 6, 2], [15, 11, 7, 3]]),
            ]
        )
    };
}

lazy_static! {
    static ref VS: HashMap<i8, [usize; 4]> = {
        HashMap::from([
            (-4, [0, 1, 2, 3]),
            (4, [12, 13, 14, 15]),
            (-1, [0, 4, 8, 12]),
            (1, [3, 7, 11, 15]),
        ])
    };
}

struct Game {
    grid: [u32; 16],
    zero: Vec<u32>,
    score: u32,
}

impl Game {
    fn new() -> Game {
        Game {
            grid: from_fn(|_| 0),
            zero: (0..=15).collect(),
            score: 0,
        }
    }

    fn len(&self) -> usize {
        self.grid.len()
    }

    fn restart(&mut self) {
        self.grid = from_fn(|_| 0);
        self.zero = (0..=15).collect();
        self.score = 0;
    }

    fn set_first_elements(&mut self) {
        let a = rand::random_range(0..=15) as usize;
        self.grid[a] = self.random_2_4();
        let b = rand::random_range(0..=15) as usize;
        self.grid[b] = self.random_2_4();
        let c = rand::random_range(0..=15) as usize;
        self.grid[c] = self.random_2_4();

        if a != b {
            self.zero.remove(b);
        } else if a != c && b != c {
            self.zero.remove(c);
        }
    }

    fn random_2_4(&self) -> u32 {
        if rand::random::<f32>() < 0.8 { 2 } else { 4 }
    }

    fn move_zero(&mut self, order: &[[usize; 4]; 4]) {
        for suborder in order {
            let mut index: i8 = 3;
            let mut end: i8 = 0;
            while self.grid[suborder[end as usize]] != 0 && index > -1 {
                end = index;
                index -= 1;
            }
            if index == -1 {
                continue;
            } else {
                self.moving(0, end, &suborder);
            }
        }
    }

    fn moving(&mut self, start: i8, end: i8, suborder: &[usize; 4]) {
        let next = start + 1;
        if next > 3 {
            return;
        }
        let suborder_next = suborder[next as usize];
        let suborder_start = suborder[start as usize];
        let grid_next = self.grid[next as usize];
        let grid_start = self.grid[start as usize];
        if grid_next != 0 ||  grid_next == grid_start {
            self.moving(next, end, suborder);
        } else {
            self.grid[suborder_start] = grid_start;
            self.grid[suborder_next] = grid_next;
            self.zero.remove(suborder_next);
            self.zero.push(suborder_start as u32);
            if start > 0 && self.grid[suborder[start as usize - 1]] != 0 {
                self.moving(start - 1, end, suborder);
            } else {
                self.moving(next, end, suborder);
            }
        }
    }

    fn compare(&mut self, order: &[[usize; 4]; 4]) {
        for suborder in order {
            for i in 0..3 {
                let start = suborder[i];
                let end = suborder[i + 1];
                let grid_start = self.grid[start];
                if grid_start != 0 && grid_start == self.grid[end] {
                    self.grid[start] += grid_start;
                    self.score += grid_start * 2;
                    self.grid[end] = 0;
                    self.zero.push(end as u32);
                }
            }
        }
    }

    fn random(&mut self) {
        use rand::seq::IndexedRandom;
        let mut rng = rand::rng();
        let r = *(self.zero.choose(&mut rng).unwrap()) as usize;
        self.grid[r] = self.random_2_4();
        self.zero.remove(r);
    }

    fn action(&mut self, key: i8, rd: bool) -> bool {
        if self.partial_move(key) {
            self.move_zero(&ORDERS[&key]);
            self.compare(&ORDERS[&(-key)]);
            self.move_zero(&ORDERS[&key]);
            if rd {
                self.random();
            }
            return true;
        }
        false
    }

    fn r#move(&self) -> bool {
        if self.zero.len() != 0 {
            return true;
        }
        let mut i = 0;
        let mut condition = false;
        let right_border = [3, 7, 14];
        let bottom_border = [12, 13, 14];
        while i < 15 && !condition {
            if right_border.contains(&i) {
                if self.grid[i] == self.grid[i + 4] {
                    condition = true;
                } else {
                    i += 1;
                }
            } else if bottom_border.contains(&i) {
                if self.grid[i] == self.grid[i + 4] {
                    condition = true;
                } else {
                    i += 1;
                }
            } else if self.grid[i] == self.grid[i + 1] || self.grid[i] == self.grid[i + 4] {
                condition = true;
            } else {
                i += 1;
            }
        }
        condition
    }

    fn condition<'a>(&'a self, suborder: &'a [usize; 4], n: usize) -> impl Fn(&usize) -> bool + 'a {
        return move |i| !suborder.contains(i) && (self.grid[*i] == self.grid[i + n] || self.grid[i + n] == 0)
    }

    fn partial_move(&self, movement: i8) -> bool {
        let filled_cells: Vec<u32> = (0..=15).filter(|i| self.zero.contains(i)).collect();
        let mut condition = false;
        let mut j = 0;
        while j < filled_cells.len() && !condition {
            let i = filled_cells[j];
            if self.condition(&VS[&movement], movement as usize)(&(i as usize)) {
                condition = true;
            } else {
                j += 1;
            }
        }
        condition
    }
}

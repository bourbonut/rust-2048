use std::collections::HashMap;
use lazy_static::lazy_static;
use rand;

lazy_static! {
    pub static ref ORDERS: HashMap<i8, [[usize; 4]; 4]> = {
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

pub struct Game {
    grid: [u32; 16],
    zero: Vec<u32>,
    score: u32,
}

impl Game {
    fn new() -> Game {
        Game {
            grid: [0; 16],
            zero: (0..=15).collect(),
            score: 0,
        }
    }

    #[allow(dead_code)]
    fn restart(&mut self) {
        self.grid = [0; 16];
        self.zero = (0..=15).collect();
        self.score = 0;
    }

    fn remove_zero(&mut self, zero_value: usize) {
        self.zero.retain(|&x| x != zero_value as u32);
    }

    pub fn init_first_elements() -> Game {
        let mut game = Game::new();
        let a = rand::random_range(0..=15) as usize;
        game.grid[a] = game.random_2_4();
        let b = rand::random_range(0..=15) as usize;
        game.grid[b] = game.random_2_4();
        let c = rand::random_range(0..=15) as usize;
        game.grid[c] = game.random_2_4();

        if a != b {
            game.zero.remove(b);
        } else if a != c && b != c {
            game.zero.remove(c);
        }
        game
    }

    fn random_2_4(&self) -> u32 {
        if rand::random::<f32>() < 0.8 { 2 } else { 4 }
    }

    pub fn move_zero(&mut self, order: &[[usize; 4]; 4]) {
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
        let grid_next = self.grid[suborder_next];
        let grid_start = self.grid[suborder_start];
        if grid_next != 0 || grid_next == grid_start {
            self.moving(next, end, suborder);
        } else {
            self.grid[suborder_start] = grid_next;
            self.grid[suborder_next] = grid_start;
            self.remove_zero(suborder_next);
            self.zero.push(suborder_start as u32);
            if start > 0 && self.grid[suborder[start as usize - 1]] != 0 {
                self.moving(start - 1, end, suborder);
            } else {
                self.moving(next, end, suborder);
            }
        }
    }

    pub fn compare(&mut self, order: &[[usize; 4]; 4]) {
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

    pub fn random(&mut self) {
        use rand::seq::IndexedRandom;
        let mut rng = rand::rng();
        let r = *(self.zero.choose(&mut rng).unwrap()) as usize;
        self.grid[r] = self.random_2_4();
        self.remove_zero(r);
    }

    fn r#move(&self) -> bool {
        if self.zero.len() != 0 {
            return true;
        }
        let mut i = 0;
        let mut condition = false;
        let right_border = [3, 7, 11];
        let bottom_border = [12, 13, 14];
        while i < 15 && !condition {
            if right_border.contains(&i) {
                if self.grid[i] == self.grid[i + 4] {
                    condition = true;
                } else {
                    i += 1;
                }
            } else if bottom_border.contains(&i) {
                if self.grid[i] == self.grid[i + 1] {
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

    pub fn partial_move(&self, movement: i8) -> bool {
        let filled_cells: Vec<u32> = (0..=15).filter(|i| !self.zero.contains(i)).collect();
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

    pub fn is_gameover(&self) -> bool {
        self.zero.len() == 0 && !self.r#move()
    }

    pub fn copy_grid(&self) -> [u32; 16] {
        self.grid.clone()
    }

    pub fn copy_zero(&self) -> Vec<u32> {
        self.zero.clone()
    }
}


#[cfg(test)]
mod test_game {
    use super::*;

    #[test]
    fn simple_addition_up() {
        // Grid input
        //
        // [2, 2, 2, 2]
        // [0, 0, 0, 0]
        // [0, 0, 0, 0]
        // [2, 2, 2, 2]
        //
        // Expected output
        //
        // [4, 4, 4, 4]
        // [0, 0, 0, 0]
        // [0, 0, 0, 0]
        // [0, 0, 0, 0]

        let mut game = Game::new();
        for i in 0..4 {
            game.grid[i] = 2;
            game.grid[i + 12] = 2;
            game.remove_zero(i);
            game.remove_zero(i + 12);
        }
        let action = -4; // Up
        game.move_zero(&ORDERS[&action]);
        game.compare(&ORDERS[&(-action)]);
        game.move_zero(&ORDERS[&action]);
        for i in 0..4 {
            assert_eq!(game.grid[i], 4);
        }
        for i in 4..16 {
            assert_eq!(game.grid[i], 0);
        }
        game.zero.sort();
        assert_eq!(game.zero, (4..16).collect::<Vec<u32>>());
    }

    #[test]
    fn simple_addition_down() {
        // Grid input
        //
        // [2, 2, 2, 2]
        // [0, 0, 0, 0]
        // [0, 0, 0, 0]
        // [2, 2, 2, 2]
        //
        // Expected output
        //
        // [0, 0, 0, 0]
        // [0, 0, 0, 0]
        // [0, 0, 0, 0]
        // [4, 4, 4, 4]

        let mut game = Game::new();
        for i in 0..4 {
            game.grid[i] = 2;
            game.grid[i + 12] = 2;
            game.remove_zero(i);
            game.remove_zero(i + 12);
        }
        let action = 4; // Down
        game.move_zero(&ORDERS[&action]);
        game.compare(&ORDERS[&(-action)]);
        game.move_zero(&ORDERS[&action]);
        for i in 0..12 {
            assert_eq!(game.grid[i], 0);
        }
        for i in 12..16 {
            assert_eq!(game.grid[i], 4);
        }
        game.zero.sort();
        assert_eq!(game.zero, (0..12).collect::<Vec<u32>>());
    }

    #[test]
    fn simple_addition_left() {
        // Grid input
        //
        // [2, 0, 0, 2]
        // [2, 0, 0, 2]
        // [2, 0, 0, 2]
        // [2, 0, 0, 2]
        //
        // Expected output
        //
        // [4, 0, 0, 0]
        // [4, 0, 0, 0]
        // [4, 0, 0, 0]
        // [4, 0, 0, 0]

        let mut game = Game::new();
        for i in 0..4 {
            game.grid[4 * i] = 2;
            game.grid[4 * i + 3] = 2;
            game.remove_zero(4 * i);
            game.remove_zero(4 * i + 3);
        }
        let action = -1; // Left
        game.move_zero(&ORDERS[&action]);
        game.compare(&ORDERS[&(-action)]);
        game.move_zero(&ORDERS[&action]);
        for i in 0..4 {
            assert_eq!(game.grid[4 * i], 4);
        }
        for i in 0..4 {
            for j in 1..4 {
                assert_eq!(game.grid[4 * i + j], 0);
            }
        }
        game.zero.sort();
        assert_eq!(game.zero, (0..16).filter(|i| i % 4 != 0).collect::<Vec<u32>>());
    }

    #[test]
    fn simple_addition_right() {
        // Grid input
        //
        // [2, 0, 0, 2]
        // [2, 0, 0, 2]
        // [2, 0, 0, 2]
        // [2, 0, 0, 2]
        //
        // Expected output
        //
        // [0, 0, 0, 4]
        // [0, 0, 0, 4]
        // [0, 0, 0, 4]
        // [0, 0, 0, 4]

        let mut game = Game::new();
        for i in 0..4 {
            game.grid[4 * i] = 2;
            game.grid[4 * i + 3] = 2;
            game.remove_zero(4 * i);
            game.remove_zero(4 * i + 3);
        }
        let action = 1; // Right
        game.move_zero(&ORDERS[&action]);
        game.compare(&ORDERS[&(-action)]);
        game.move_zero(&ORDERS[&action]);
        for i in 0..4 {
            assert_eq!(game.grid[4 * i + 3], 4);
        }
        for i in 0..4 {
            for j in 0..3 {
                assert_eq!(game.grid[4 * i + j], 0);
            }
        }
        game.zero.sort();
        assert_eq!(game.zero, (0..16).filter(|i| i % 4 != 3).collect::<Vec<u32>>());
    }

    #[test]
    fn simple_move(){
        // Move : Left
        //
        // Grid input
        //
        // [0, 0, 0, 0]
        // [0, 0, 0, 0]
        // [0, 0, 0, 4]
        // [0, 0, 4, 8]
        //
        // Expected output
        // 
        // [0, 0, 0, 0]
        // [0, 0, 0, 0]
        // [4, 0, 0, 0]
        // [4, 8, 0, 0]

        let mut game = Game::new();
        game.grid[11] = 4;
        game.grid[14] = 4;
        game.grid[15] = 8;
        game.remove_zero(11);
        game.remove_zero(14);
        game.remove_zero(15);
        let action = -1;
        game.move_zero(&ORDERS[&action]);
        game.compare(&ORDERS[&(-action)]);
        game.move_zero(&ORDERS[&action]);
        for i in 0..16 {
            if i == 8 || i == 12 {
                assert_eq!(game.grid[i], 4);
            } else if i == 13 {
                assert_eq!(game.grid[i], 8);
            } else {
                assert_eq!(game.grid[i], 0);
            }
        }

        game.zero.sort();
        let mut expected_zero = (0..16).collect::<Vec<u32>>();
        expected_zero.retain(|&i| i != 8 && i != 12 && i != 13);
        expected_zero.sort();
        assert_eq!(game.zero, expected_zero);
    }

    #[test]
    fn complex_addition() {
        // Grid input
        //
        // [2, 2, 2, 2]
        // [2, 2, 2, 2]
        // [2, 2, 2, 2]
        // [2, 2, 2, 2]
        //
        // Expected output
        //
        // [0, 0, 0, 0]
        // [0, 0, 0, 0]
        // [4, 4, 4, 4]
        // [4, 4, 4, 4]

        let mut game = Game::new();
        for i in 0..16 {
            game.grid[i] = 2;
            game.remove_zero(i);
        }
        let action = 4; // Down
        game.move_zero(&ORDERS[&action]);
        game.compare(&ORDERS[&(-action)]);
        game.move_zero(&ORDERS[&action]);
        for i in 0..8 {
            assert_eq!(game.grid[i], 0);
        }
        for i in 8..16 {
            assert_eq!(game.grid[i], 4);
        }
        game.zero.sort();
        assert_eq!(game.zero, (0..8).collect::<Vec<u32>>());
    }
}

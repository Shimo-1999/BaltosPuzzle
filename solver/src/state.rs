use crate::input::Input;
use crate::utils::bases;
use crate::utils::change;
use crate::utils::hash;
use crate::utils::BASE;

pub struct State {
    n: usize,
    pub board: Vec<Vec<i32>>,
    pub tile_positions: Vec<(usize, usize)>,
    pub target_positions: Vec<(usize, usize)>,
    pub zero_position: (usize, usize),
    mismatch_i: Vec<u64>,
    mismatch_j: Vec<u64>,
    mismatch_k: Vec<u64>, // diagonal, idx: (n - 1) + j - i
    pub bases: Vec<u64>,
    pub hash: u64,
    ope_count: i32,
}

impl State {
    pub fn new(input: &Input, tile_positions: Vec<(usize, usize)>, target_positions: Vec<(usize, usize)>) -> Self {
        let mut board = vec![vec![-1; 2 * input.n - 1]; 2 * input.n - 1];
        for (idx, &(i, j)) in tile_positions.iter().enumerate() {
            board[i][j] = idx as i32;
        }
        let zero_position = tile_positions[0];
        let mut mismatch_i = vec![0; 2 * input.n - 1];
        let mut mismatch_j = vec![0; 2 * input.n - 1];
        let mut mismatch_k = vec![0; 2 * input.n - 1];
        for i in 0..2 * input.n - 1 {
            for j in 0..2 * input.n - 1 {
                if board[i][j] != -1 {
                    let num = board[i][j] as usize;
                    if tile_positions[num] != target_positions[num] {
                        mismatch_i[i] += 1;
                        mismatch_j[j] += 1;
                        mismatch_k[(input.n - 1) + j - i] += 1;
                    }
                }
            }
        }

        let bases = bases(BASE, (2 * input.n - 1) * (2 * input.n - 1) - input.n * (input.n - 1) + 1);
        let mut tile_vec = tile_positions.iter().map(|&(i, j)| (i * (2 * input.n - 1) + j) as u64).collect::<Vec<_>>();
        tile_vec.push(0); // last op
        let hash = hash(&tile_vec, BASE);

        State {
            n: input.n,
            board,
            tile_positions,
            target_positions,
            zero_position,
            mismatch_i,
            mismatch_j,
            mismatch_k,
            bases,
            hash,
            ope_count: 0,
        }
    }
    fn is_valid_coordinates(&self, i: i32, j: i32) -> bool {
        0 <= i && i < 2 * self.n as i32 - 1 && 0 <= j && j < 2 * self.n as i32 - 1 && i < j + self.n as i32 && j < i + self.n as i32
    }
    fn wrap_coordinates(&self, i: i32, j: i32) -> (usize, usize) {
        let mut wrapped_i = i;
        let mut wrapped_j = j;
        let offsets: [(i32, i32); 7] = [
            (0, 0),
            (-(self.n as i32) + 1, self.n as i32),
            (self.n as i32, 2 * self.n as i32 - 1),
            (2 * self.n as i32 - 1, self.n as i32 - 1),
            (self.n as i32 - 1, -(self.n as i32)),
            (-(self.n as i32), -2 * self.n as i32 + 1),
            (-2 * self.n as i32 + 1, -(self.n as i32) + 1),
        ];
        for &(di, dj) in offsets.iter() {
            if self.is_valid_coordinates(i + di, j + dj) {
                wrapped_i = i + di;
                wrapped_j = j + dj;
                break;
            }
        }
        (wrapped_i as usize, wrapped_j as usize)
    }
    fn rotate_tiles(&mut self, a_i: usize, a_j: usize, b_i: usize, b_j: usize, c_i: usize, c_j: usize) {
        let hash = change(self.hash, &self.bases, self.board[a_i][a_j] as usize, (a_i * (2 * self.n - 1) + a_j) as u64, 0);
        let hash = change(hash, &self.bases, self.board[b_i][b_j] as usize, (b_i * (2 * self.n - 1) + b_j) as u64, 0);
        let hash = change(hash, &self.bases, self.board[c_i][c_j] as usize, (c_i * (2 * self.n - 1) + c_j) as u64, 0);
        for &(i, j) in &[(a_i, a_j), (b_i, b_j), (c_i, c_j)] {
            let num = self.board[i][j] as usize;
            if self.target_positions[num] != self.tile_positions[num] {
                self.mismatch_i[i] -= 1;
                self.mismatch_j[j] -= 1;
                self.mismatch_k[(self.n - 1) + j - i] -= 1;
            }
        }
        (self.board[a_i][a_j], self.board[b_i][b_j], self.board[c_i][c_j]) = (self.board[c_i][c_j], self.board[a_i][a_j], self.board[b_i][b_j]);
        self.tile_positions[self.board[b_i][b_j] as usize] = (b_i, b_j);
        self.tile_positions[self.board[c_i][c_j] as usize] = (c_i, c_j);
        self.tile_positions[self.board[a_i][a_j] as usize] = (a_i, a_j);
        for &(i, j) in &[(a_i, a_j), (b_i, b_j), (c_i, c_j)] {
            let num = self.board[i][j] as usize;
            if self.target_positions[num] != self.tile_positions[num] {
                self.mismatch_i[i] += 1;
                self.mismatch_j[j] += 1;
                self.mismatch_k[(self.n - 1) + j - i] += 1;
            }
        }
        let hash = change(hash, &self.bases, self.board[a_i][a_j] as usize, 0, (a_i * (2 * self.n - 1) + a_j) as u64);
        let hash = change(hash, &self.bases, self.board[b_i][b_j] as usize, 0, (b_i * (2 * self.n - 1) + b_j) as u64);
        let hash = change(hash, &self.bases, self.board[c_i][c_j] as usize, 0, (c_i * (2 * self.n - 1) + c_j) as u64);
        self.hash = hash;
    }
    pub fn apply(&mut self, m: char) {
        let (a_i, a_j) = self.tile_positions[0];
        let ((b_i, b_j), (c_i, c_j)) = match m {
            '1' => (self.wrap_coordinates(a_i as i32 - 1, a_j as i32), self.wrap_coordinates(a_i as i32, a_j as i32 + 1)),
            '2' => (self.wrap_coordinates(a_i as i32, a_j as i32 + 1), self.wrap_coordinates(a_i as i32 + 1, a_j as i32 + 1)),
            '3' => (self.wrap_coordinates(a_i as i32 + 1, a_j as i32 + 1), self.wrap_coordinates(a_i as i32 + 1, a_j as i32)),
            '4' => (self.wrap_coordinates(a_i as i32 + 1, a_j as i32), self.wrap_coordinates(a_i as i32, a_j as i32 - 1)),
            '5' => (self.wrap_coordinates(a_i as i32, a_j as i32 - 1), self.wrap_coordinates(a_i as i32 - 1, a_j as i32 - 1)),
            '6' => (self.wrap_coordinates(a_i as i32 - 1, a_j as i32 - 1), self.wrap_coordinates(a_i as i32 - 1, a_j as i32)),
            'A' => (self.wrap_coordinates(a_i as i32, a_j as i32 + 1), self.wrap_coordinates(a_i as i32 - 1, a_j as i32)),
            'B' => (self.wrap_coordinates(a_i as i32 + 1, a_j as i32 + 1), self.wrap_coordinates(a_i as i32, a_j as i32 + 1)),
            'C' => (self.wrap_coordinates(a_i as i32 + 1, a_j as i32), self.wrap_coordinates(a_i as i32 + 1, a_j as i32 + 1)),
            'D' => (self.wrap_coordinates(a_i as i32, a_j as i32 - 1), self.wrap_coordinates(a_i as i32 + 1, a_j as i32)),
            'E' => (self.wrap_coordinates(a_i as i32 - 1, a_j as i32 - 1), self.wrap_coordinates(a_i as i32, a_j as i32 - 1)),
            'F' => (self.wrap_coordinates(a_i as i32 - 1, a_j as i32), self.wrap_coordinates(a_i as i32 - 1, a_j as i32 - 1)),
            _ => unreachable!(),
        };
        self.rotate_tiles(a_i, a_j, b_i, b_j, c_i, c_j);
        let is_clockwise = match m {
            '1' | '2' | '3' | '4' | '5' | '6' => true,
            'A' | 'B' | 'C' | 'D' | 'E' | 'F' => false,
            _ => unreachable!(),
        };
        let mut hash = self.hash;
        if self.ope_count != 0 {
            if is_clockwise {
                hash = change(hash, &self.bases, self.bases.len() - 1, 2, 0);
            } else {
                hash = change(hash, &self.bases, self.bases.len() - 1, 1, 0);
            }
        }
        if is_clockwise {
            hash = change(hash, &self.bases, self.bases.len() - 1, 0, 1);
        } else {
            hash = change(hash, &self.bases, self.bases.len() - 1, 0, 2);
        }
        self.hash = hash;
        self.ope_count += 1;
        self.zero_position = self.tile_positions[0];
    }
    pub fn revert(&mut self, m: char) {
        let (a_i, a_j) = self.tile_positions[0];
        let ((b_i, b_j), (c_i, c_j)) = match m {
            'C' => (self.wrap_coordinates(a_i as i32 - 1, a_j as i32), self.wrap_coordinates(a_i as i32, a_j as i32 + 1)),
            'D' => (self.wrap_coordinates(a_i as i32, a_j as i32 + 1), self.wrap_coordinates(a_i as i32 + 1, a_j as i32 + 1)),
            'E' => (self.wrap_coordinates(a_i as i32 + 1, a_j as i32 + 1), self.wrap_coordinates(a_i as i32 + 1, a_j as i32)),
            'F' => (self.wrap_coordinates(a_i as i32 + 1, a_j as i32), self.wrap_coordinates(a_i as i32, a_j as i32 - 1)),
            'A' => (self.wrap_coordinates(a_i as i32, a_j as i32 - 1), self.wrap_coordinates(a_i as i32 - 1, a_j as i32 - 1)),
            'B' => (self.wrap_coordinates(a_i as i32 - 1, a_j as i32 - 1), self.wrap_coordinates(a_i as i32 - 1, a_j as i32)),
            '5' => (self.wrap_coordinates(a_i as i32, a_j as i32 + 1), self.wrap_coordinates(a_i as i32 - 1, a_j as i32)),
            '6' => (self.wrap_coordinates(a_i as i32 + 1, a_j as i32 + 1), self.wrap_coordinates(a_i as i32, a_j as i32 + 1)),
            '1' => (self.wrap_coordinates(a_i as i32 + 1, a_j as i32), self.wrap_coordinates(a_i as i32 + 1, a_j as i32 + 1)),
            '2' => (self.wrap_coordinates(a_i as i32, a_j as i32 - 1), self.wrap_coordinates(a_i as i32 + 1, a_j as i32)),
            '3' => (self.wrap_coordinates(a_i as i32 - 1, a_j as i32 - 1), self.wrap_coordinates(a_i as i32, a_j as i32 - 1)),
            '4' => (self.wrap_coordinates(a_i as i32 - 1, a_j as i32), self.wrap_coordinates(a_i as i32 - 1, a_j as i32 - 1)),
            _ => unreachable!(),
        };
        self.rotate_tiles(a_i, a_j, b_i, b_j, c_i, c_j);
        let is_clockwise = match m {
            '1' | '2' | '3' | '4' | '5' | '6' => true,
            'A' | 'B' | 'C' | 'D' | 'E' | 'F' => false,
            _ => unreachable!(),
        };
        self.ope_count -= 1;
        let mut hash = self.hash;
        if is_clockwise {
            hash = change(hash, &self.bases, self.bases.len() - 1, 1, 0);
        } else {
            hash = change(hash, &self.bases, self.bases.len() - 1, 2, 0);
        }
        if self.ope_count != 0 {
            if is_clockwise {
                hash = change(hash, &self.bases, self.bases.len() - 1, 0, 2);
            } else {
                hash = change(hash, &self.bases, self.bases.len() - 1, 0, 1);
            }
        }
        self.hash = hash;
        self.zero_position = self.tile_positions[0];
    }
    pub fn mismatch_cost(&self) -> u32 {
        let mut left_i = usize::MAX;
        let mut right_i = 0;
        let mut left_j = usize::MAX;
        let mut right_j = 0;
        let mut left_k = usize::MAX;
        let mut right_k = 0;

        for idx in 0..2 * self.n - 1 {
            if self.mismatch_i[idx] > 0 {
                if left_i == usize::MAX {
                    left_i = idx;
                }
                right_i = idx;
            }
            if self.mismatch_j[idx] > 0 {
                if left_j == usize::MAX {
                    left_j = idx;
                }
                right_j = idx;
            }
            if self.mismatch_k[idx] > 0 {
                if left_k == usize::MAX {
                    left_k = idx;
                }
                right_k = idx;
            }
        }
        let left_i = if left_i == usize::MAX { 0 } else { left_i };
        let right_i = if right_i == 0 { 0 } else { right_i };
        let left_j = if left_j == usize::MAX { 0 } else { left_j };
        let right_j = if right_j == 0 { 0 } else { right_j };
        let left_k = if left_k == usize::MAX { 0 } else { left_k };
        let right_k = if right_k == 0 { 0 } else { right_k };
        (right_i - left_i + right_j - left_j + right_k - left_k) as u32
    }
    pub fn distance(&self, i: i32, j: i32, i2: i32, j2: i32) -> u32 {
        if i < i2 && j < j2 {
            (i2 - i).max(j2 - j) as u32
        } else if i > i2 && j > j2 {
            (i - i2).max(j - j2) as u32
        } else {
            (i - i2).unsigned_abs() + (j - j2).unsigned_abs()
        }
    }
    pub fn surrounding(&self, i: usize) -> Vec<usize> {
        let (now_i, now_j) = self.tile_positions[i];
        let mut result = vec![];
        for &(di, dj) in &[(0, 0), (0, 1), (1, 1), (1, 0), (0, -1), (-1, -1), (-1, 0)] {
            let (wrapped_i, wrapped_j) = self.wrap_coordinates(now_i as i32 + di, now_j as i32 + dj);
            result.push(self.board[wrapped_i][wrapped_j] as usize);
        }
        result
    }
    pub fn raw_distance(&self, i: usize) -> u32 {
        let (now_i, now_j) = self.tile_positions[i];
        let (target_i, target_j) = self.target_positions[i];
        let target_i = target_i as i32;
        let target_j = target_j as i32;
        let offsets: [(i32, i32); 7] = [
            (0, 0),
            (-(self.n as i32) + 1, self.n as i32),
            (self.n as i32, 2 * self.n as i32 - 1),
            (2 * self.n as i32 - 1, self.n as i32 - 1),
            (self.n as i32 - 1, -(self.n as i32)),
            (-(self.n as i32), -2 * self.n as i32 + 1),
            (-2 * self.n as i32 + 1, -(self.n as i32) + 1),
        ];
        let mut min_distance = std::u32::MAX;
        for &(di, dj) in offsets.iter() {
            let wrapped_i = now_i as i32 + di;
            let wrapped_j = now_j as i32 + dj;
            let distance = self.distance(wrapped_i, wrapped_j, target_i, target_j);
            min_distance = min_distance.min(distance);
        }
        min_distance
    }
    pub fn weighted_distance(&self, i: usize) -> u32 {
        if i == 0 {
            return 0;
        }
        (self.raw_distance(i) as f64).powf(1.7).round() as u32
    }
}

#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::{collections::HashSet, ops::RangeBounds};
use svg::node::{
    element::{Circle, Group, Line, Rectangle, Style, Title},
    Text,
};
use web_sys::console;

#[derive(Clone, Debug)]
pub struct Input {
    pub n: usize,
    pub board: Vec<Vec<u32>>,
}

pub fn parse_input(f: &str) -> Input {
    let mut lines = f.lines();
    let n: usize = lines.next().unwrap().trim().parse().expect("Failed to parse n");
    let mut board = Vec::new();
    for i in 0..(2 * n - 1) {
        let len = (n + i).min(n * 3 - i);
        let board_row: Vec<u32> = lines.next().unwrap().split_whitespace().take(len).map(|s| s.parse().expect("Failed to parse number")).collect();
        board.push(board_row);
    }
    Input { n, board }
}

pub struct Output {
    pub moves: Vec<char>,
}

pub fn parse_output(f: &str) -> Output {
    let moves = f.chars().collect();
    Output { moves }
}

pub struct State {
    pub h: usize,
    pub w: usize,
    pub n: usize,
    pub blank_tile: (usize, usize),
    pub board: Vec<Vec<i32>>,
}

impl State {
    pub fn new(input: &Input) -> Self {
        let h = 2 * input.n - 1;
        let w = 2 * input.n - 1;
        let mut board = vec![vec![-1; w]; h];
        for i in 0..input.n {
            for j in 0..input.board[i].len() {
                board[i][j] = input.board[i][j] as i32;
            }
        }
        for i in input.n..h {
            for j in 0..input.board[i].len() {
                board[i][j + (i - input.n) + 1] = input.board[i][j] as i32;
            }
        }
        let mut blank_tile = (0, 0);
        for i in 0..h {
            for j in 0..w {
                if board[i][j] == 0 {
                    blank_tile = (i, j);
                }
            }
        }
        State { h, w, n: input.n, blank_tile, board }
    }
    pub fn is_valid_coordinates(&self, i: i32, j: i32) -> bool {
        0 <= i && i < 2 * self.n as i32 - 1 && 0 <= j && j < 2 * self.n as i32 - 1 && i <= j + self.n as i32 - 1 && j <= i + self.n as i32 - 1
    }
    pub fn wrap_coordinates(&self, i: i32, j: i32) -> (usize, usize) {
        let mut wrapped_i = i;
        let mut wrapped_j = j;
        for (di, dj) in vec![
            (0, 0),
            (-(self.n as i32) + 1, self.n as i32),
            (self.n as i32, 2 * self.n as i32 - 1),
            (2 * self.n as i32 - 1, self.n as i32 - 1),
            (self.n as i32 - 1, -(self.n as i32)),
            (-(self.n as i32), -2 * self.n as i32 + 1),
            (-2 * self.n as i32 + 1, -(self.n as i32) + 1),
        ] {
            if self.is_valid_coordinates(i + di, j + dj) {
                wrapped_i = i + di;
                wrapped_j = j + dj;
            }
        }
        (wrapped_i as usize, wrapped_j as usize)
    }
    pub fn apply(&mut self, m: char) {
        match m {
            '1' => {
                let (a_i, a_j) = self.blank_tile;
                let (b_i, b_j) = self.wrap_coordinates(a_i as i32 - 1, a_j as i32);
                let (c_i, c_j) = self.wrap_coordinates(a_i as i32, a_j as i32 + 1);
                (self.board[a_i][a_j], self.board[b_i][b_j], self.board[c_i][c_j]) = (self.board[c_i][c_j], self.board[a_i][a_j], self.board[b_i][b_j]);
                self.blank_tile = (b_i, b_j);
            }
            '2' => {
                let (a_i, a_j) = self.blank_tile;
                let (b_i, b_j) = self.wrap_coordinates(a_i as i32, a_j as i32 + 1);
                let (c_i, c_j) = self.wrap_coordinates(a_i as i32 + 1, a_j as i32 + 1);
                (self.board[a_i][a_j], self.board[b_i][b_j], self.board[c_i][c_j]) = (self.board[c_i][c_j], self.board[a_i][a_j], self.board[b_i][b_j]);
                self.blank_tile = (b_i, b_j);
            }
            '3' => {
                let (a_i, a_j) = self.blank_tile;
                let (b_i, b_j) = self.wrap_coordinates(a_i as i32 + 1, a_j as i32 + 1);
                let (c_i, c_j) = self.wrap_coordinates(a_i as i32 + 1, a_j as i32);
                (self.board[a_i][a_j], self.board[b_i][b_j], self.board[c_i][c_j]) = (self.board[c_i][c_j], self.board[a_i][a_j], self.board[b_i][b_j]);
                self.blank_tile = (b_i, b_j);
            }
            '4' => {
                let (a_i, a_j) = self.blank_tile;
                let (b_i, b_j) = self.wrap_coordinates(a_i as i32 + 1, a_j as i32);
                let (c_i, c_j) = self.wrap_coordinates(a_i as i32, a_j as i32 - 1);
                (self.board[a_i][a_j], self.board[b_i][b_j], self.board[c_i][c_j]) = (self.board[c_i][c_j], self.board[a_i][a_j], self.board[b_i][b_j]);
                self.blank_tile = (b_i, b_j);
            }
            '5' => {
                let (a_i, a_j) = self.blank_tile;
                let (b_i, b_j) = self.wrap_coordinates(a_i as i32, a_j as i32 - 1);
                let (c_i, c_j) = self.wrap_coordinates(a_i as i32 - 1, a_j as i32 - 1);
                (self.board[a_i][a_j], self.board[b_i][b_j], self.board[c_i][c_j]) = (self.board[c_i][c_j], self.board[a_i][a_j], self.board[b_i][b_j]);
                self.blank_tile = (b_i, b_j);
            }
            '6' => {
                let (a_i, a_j) = self.blank_tile;
                let (b_i, b_j) = self.wrap_coordinates(a_i as i32 - 1, a_j as i32 - 1);
                let (c_i, c_j) = self.wrap_coordinates(a_i as i32 - 1, a_j as i32);
                (self.board[a_i][a_j], self.board[b_i][b_j], self.board[c_i][c_j]) = (self.board[c_i][c_j], self.board[a_i][a_j], self.board[b_i][b_j]);
                self.blank_tile = (b_i, b_j);
            }
            'A' => {
                let (a_i, a_j) = self.blank_tile;
                let (b_i, b_j) = self.wrap_coordinates(a_i as i32, a_j as i32 + 1);
                let (c_i, c_j) = self.wrap_coordinates(a_i as i32 - 1, a_j as i32);
                (self.board[a_i][a_j], self.board[b_i][b_j], self.board[c_i][c_j]) = (self.board[c_i][c_j], self.board[a_i][a_j], self.board[b_i][b_j]);
                self.blank_tile = (b_i, b_j);
            }
            'B' => {
                let (a_i, a_j) = self.blank_tile;
                let (b_i, b_j) = self.wrap_coordinates(a_i as i32 + 1, a_j as i32 + 1);
                let (c_i, c_j) = self.wrap_coordinates(a_i as i32, a_j as i32 + 1);
                (self.board[a_i][a_j], self.board[b_i][b_j], self.board[c_i][c_j]) = (self.board[c_i][c_j], self.board[a_i][a_j], self.board[b_i][b_j]);
                self.blank_tile = (b_i, b_j);
            }
            'C' => {
                let (a_i, a_j) = self.blank_tile;
                let (b_i, b_j) = self.wrap_coordinates(a_i as i32 + 1, a_j as i32);
                let (c_i, c_j) = self.wrap_coordinates(a_i as i32 + 1, a_j as i32 + 1);
                (self.board[a_i][a_j], self.board[b_i][b_j], self.board[c_i][c_j]) = (self.board[c_i][c_j], self.board[a_i][a_j], self.board[b_i][b_j]);
                self.blank_tile = (b_i, b_j);
            }
            'D' => {
                let (a_i, a_j) = self.blank_tile;
                let (b_i, b_j) = self.wrap_coordinates(a_i as i32, a_j as i32 - 1);
                let (c_i, c_j) = self.wrap_coordinates(a_i as i32 + 1, a_j as i32);
                (self.board[a_i][a_j], self.board[b_i][b_j], self.board[c_i][c_j]) = (self.board[c_i][c_j], self.board[a_i][a_j], self.board[b_i][b_j]);
                self.blank_tile = (b_i, b_j);
            }
            'E' => {
                let (a_i, a_j) = self.blank_tile;
                let (b_i, b_j) = self.wrap_coordinates(a_i as i32 - 1, a_j as i32 - 1);
                let (c_i, c_j) = self.wrap_coordinates(a_i as i32, a_j as i32 - 1);
                (self.board[a_i][a_j], self.board[b_i][b_j], self.board[c_i][c_j]) = (self.board[c_i][c_j], self.board[a_i][a_j], self.board[b_i][b_j]);
                self.blank_tile = (b_i, b_j);
            }
            'F' => {
                let (a_i, a_j) = self.blank_tile;
                let (b_i, b_j) = self.wrap_coordinates(a_i as i32 - 1, a_j as i32);
                let (c_i, c_j) = self.wrap_coordinates(a_i as i32 - 1, a_j as i32 - 1);
                (self.board[a_i][a_j], self.board[b_i][b_j], self.board[c_i][c_j]) = (self.board[c_i][c_j], self.board[a_i][a_j], self.board[b_i][b_j]);
                self.blank_tile = (b_i, b_j);
            }
            _ => {}
        }
    }
    pub fn raw_distance(&self, now_i: i32, now_j: i32, target_i: i32, target_j: i32) -> i32 {
        let offsets: [(i32, i32); 7] = [
            (0, 0),
            (-(self.n as i32) + 1, self.n as i32),
            (self.n as i32, 2 * self.n as i32 - 1),
            (2 * self.n as i32 - 1, self.n as i32 - 1),
            (self.n as i32 - 1, -(self.n as i32)),
            (-(self.n as i32), -2 * self.n as i32 + 1),
            (-2 * self.n as i32 + 1, -(self.n as i32) + 1),
        ];
        let mut min_distance = std::i32::MAX;
        for &(di, dj) in offsets.iter() {
            let wrapped_i = now_i as i32 + di;
            let wrapped_j = now_j as i32 + dj;
            let distance = if wrapped_i < target_i && wrapped_j < target_j {
                (target_i - wrapped_i).max(target_j - wrapped_j)
            } else if wrapped_i > target_i && wrapped_j > target_j {
                (wrapped_i - target_i).max(wrapped_j - target_j)
            } else {
                (wrapped_i - target_i).unsigned_abs() as i32 + (wrapped_j - target_j).unsigned_abs() as i32
            };
            min_distance = min_distance.min(distance);
        }
        min_distance
    }
}

pub fn vis(input: &Input, output: &Output) -> (String, String) {
    let mut state = State::new(input);
    for &m in &output.moves {
        state.apply(m);
    }
    let mut target_positions = vec![(state.n - 1, state.n - 1)];
    for i in 0..state.h {
        for j in 0..state.w {
            if state.is_valid_coordinates(i as i32, j as i32) && (i, j) != (state.n - 1, state.n - 1) {
                target_positions.push((i, j));
            }
        }
    }

    let mut err = String::new();
    let D = 600.0 / (state.h + 2) as f64;
    let W = 600.0;
    let H = 600.0;
    let mut doc = svg::Document::new().set("id", "vis").set("viewBox", (-5.0, -5.0, W + 10.0, H + 10.0)).set("width", W + 10.0).set("height", H + 10.0).set("style", "background-color:white");
    for i in -1..=state.h as i32 {
        for j in -1..=state.w as i32 {
            let (board_i, board_j) = state.wrap_coordinates(i, j);
            let mut g = Group::new().set("id", format!("({},{})", i, j));
            if state.board[board_i][board_j] == -1 {
                continue;
            }
            let target_i = target_positions[state.board[board_i][board_j] as usize].0 as i32;
            let target_j = target_positions[state.board[board_i][board_j] as usize].1 as i32;
            if state.is_valid_coordinates(i, j) {
                if state.board[board_i][board_j] == 0 {
                    g = g.add(Rectangle::new().set("x", (j + 1) as f64 * D).set("y", (i + 1) as f64 * D).set("width", D).set("height", D).set("fill", "black").set("stroke", "black").set("stroke-width", 1));
                    g = g.add(
                        svg::node::element::Text::new(format!("{}", state.board[board_i][board_j]))
                            .set("x", ((j + 1) as f64 + 0.5) * D)
                            .set("y", ((i + 1) as f64 + 0.5) * D)
                            .set("font-size", D * 1.0 / 3.0)
                            .set("fill", "white")
                            .set("text-anchor", "middle")
                            .set("dominant-baseline", "central"),
                    );
                } else {
                    g = g.add(
                        Rectangle::new()
                            .set("x", (j + 1) as f64 * D)
                            .set("y", (i + 1) as f64 * D)
                            .set("width", D)
                            .set("height", D)
                            .set("fill", color(state.raw_distance(i, j, target_i, target_j) as f64 / (state.n) as f64))
                            .set("stroke", "black")
                            .set("stroke-width", 1),
                    );
                    g = g.add(
                        svg::node::element::Text::new(format!("{}", state.board[board_i][board_j]))
                            .set("x", ((j + 1) as f64 + 0.5) * D)
                            .set("y", ((i + 1) as f64 + 0.5) * D)
                            .set("font-size", D * 1.0 / 3.0)
                            .set("fill", "black".to_string())
                            .set("text-anchor", "middle")
                            .set("dominant-baseline", "central"),
                    );
                }
            } else {
                g = g.add(Rectangle::new().set("x", (j + 1) as f64 * D).set("y", (i + 1) as f64 * D).set("width", D).set("height", D).set("fill", "white").set("stroke", "black").set("stroke-width", 1));
                g = g.add(
                    svg::node::element::Text::new(format!("{}", state.board[board_i][board_j]))
                        .set("x", ((j + 1) as f64 + 0.5) * D)
                        .set("y", ((i + 1) as f64 + 0.5) * D)
                        .set("font-size", D * 1.0 / 3.0)
                        .set("fill", "gray".to_string())
                        .set("text-anchor", "middle")
                        .set("dominant-baseline", "central"),
                );
            }
            doc = doc.add(g);
        }
    }
    (err, doc.to_string())
}

/// 0 <= val <= 1
pub fn color(mut val: f64) -> String {
    val = val.min(1.0);
    val = val.max(0.0);
    let (r, g, b) = if val < 0.5 {
        let x = val * 2.0;
        (30. * (1.0 - x) + 144. * x, 144. * (1.0 - x) + 255. * x, 255. * (1.0 - x) + 30. * x)
    } else {
        let x = val * 2.0 - 1.0;
        (144. * (1.0 - x) + 255. * x, 255. * (1.0 - x) + 30. * x, 30. * (1.0 - x) + 70. * x)
    };
    format!("#{:02x}{:02x}{:02x}", r.round() as i32, g.round() as i32, b.round() as i32)
}

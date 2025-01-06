use proconio::input;

pub struct Input {
    pub n: usize,
    pub board: Vec<Vec<i32>>,
}

pub fn read_input() -> Input {
    input! {
        n: usize,
    }
    let mut board = vec![];
    for i in 0..2 * n - 1 {
        let left_length = (i as i32 - (n as i32 - 1)).max(0) as usize;
        let right_length = (2 * n as i32 - 1 - i as i32 - n as i32).max(0) as usize;
        input! {
            row: [i32; 2 * n - 1 - (n as i32 - 1 - i as i32).unsigned_abs() as usize],
        }
        let left_vec = vec![-1; left_length];
        let right_vec = vec![-1; right_length];
        let row = left_vec.into_iter().chain(row.into_iter()).chain(right_vec.into_iter()).collect();
        board.push(row);
    }
    Input { n, board }
}

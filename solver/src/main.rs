mod beam_search;
mod input;
mod state;
mod utils;

fn main() {
    let input = input::read_input();

    let mut tile_positions = vec![(0, 0); (2 * input.n - 1) * (2 * input.n - 1) - input.n * (input.n - 1)];
    let mut target_positions = vec![(0, 0); (2 * input.n - 1) * (2 * input.n - 1) - input.n * (input.n - 1)];
    let mut num = 0;
    for i in 0..2 * input.n - 1 {
        for j in 0..2 * input.n - 1 {
            if input.board[i][j] != -1 {
                tile_positions[input.board[i][j] as usize] = (i, j);
                if (i, j) != (input.n - 1, input.n - 1) {
                    num += 1;
                    target_positions[num] = (i, j);
                } else {
                    target_positions[0] = (i, j);
                }
            }
        }
    }

    let state = state::State::new(&input, tile_positions, target_positions);
    let mut beam_search = beam_search::BeamSearch::new(state, '!');
    let output = beam_search.solve(&input);

    for op in output.iter() {
        print!("{}", op);
    }
    println!();
}

use sudoku::{make_solver, Board, Input};

fn main() {
    let mut stdin = std::io::stdin();
    let mut board = Board::read(&mut stdin).unwrap();
    println!("{}", board);
    let solver = make_solver();
    solver.solve(&mut board).unwrap();
    println!("{}", board);
}

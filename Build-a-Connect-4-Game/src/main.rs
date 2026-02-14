// task 1 Connect 4 Introduction: panic!
pub enum Square {
    Occupied,
    Unused,
}
// task 1..end..

// task 3

const ROWS: usize = 6;
const COLS: usize = 7;

#[derive(Debug, Copy, Clone)]
pub enum Color {
    Yellow,
    Red,
}
#[derive(Debug, Copy, Clone)]
pub enum Square {
    Occupied(Color),
    Unused,
}

// task 3..end..

// task 6

pub type Board = [[Square; COLS]; ROWS];
// task 6..end..
fn main() {
    // task 1 Connect 4 Introduction: panic!
    let mut sq = Square::Occupied;
    make_occupied(&mut sq);
    // task 1..end..

    // task 2 Arrays declaration and access may feel “backward”
    let a: [[u8; 4]; 2] = [[10, 20, 30, 40], [50, 60, 70, 80]];
    
    let row = 0;
    let col = 3;
    
    let result = access(a, row, col);
    println!("{}", result);

    // task 2..end..

    // task 3 Create Board
     let mut board = create_board();
    println!("{:?}", board);
    // task 3..end..

    // task 4 Print Board
    let mut board = create_board();
    board[0][3] = Square::Occupied(Color::Yellow);
    let result = printable_board(&board);
    println!("{}", result);
    // task 4..end..
}
// task 1 Connect 4 Introduction: panic!
pub fn make_occupied(square: &mut Square) {
    match square {
        Square::Occupied => panic!("Square already occupied"),
        Square::Unused => *square = Square::Occupied,
    }
}
// task 1..end..

// task 3 Create Board
pub fn create_board() -> [[Square; COLS]; ROWS] {
    [[Square::Unused; COLS]; ROWS]
}
// task 3..end..

// task 4 Print Board

pub fn create_board() -> Board {
    let mut board = [[Square::Unused; COLS]; ROWS];
    for col in 0..COLS {
        for row in 0..ROWS {
            board[row][col] = Square::Unused;
        }
    }
    board
}

pub fn printable_board(board: &Board) -> String {
    let mut result = String::new();
   for row in board.iter().rev(){
    let mut row_str = String::from("|");
    for square in row.iter(){
        match square {
    Square::Occupied(Color::Yellow) => row_str.push('Y'),
    Square::Occupied(Color::Red) => row_str.push('R'),
    Square::Unused => row_str.push(' '),
}
 row_str.push('|');
    } 
    result.push_str(&row_str);
    result.push_str("\n");
    }
    result
}
// task 4..end..

// task 5 place piece
pub fn place_piece(board: &mut Board, col: usize, color: Color) {
    if col >= COLS {
        panic!("Column is full");
    }
    if let Square::Occupied(_) = board[ROWS - 1][col] {
        panic!("Column is full");
    }
    for row in 0..ROWS {
        match board[row][col] {
            Square::Unused => {
                board[row][col] = Square::Occupied(color);
                return; 
            }
            Square::Occupied(_) => {
                
            }
        }
    }
}
// task 5..end..
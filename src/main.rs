/* Rust Tic-Tac-Toe
 * ----------------
 * my first ever rust project!!!!!
 * wow!!!!!! it's crazy!!!!!
 */
use std::cmp;
use std::fmt;
use text_io::read;

#[derive(Copy, Clone, Debug, PartialEq)]
enum CellState {
    X,
    O,
    None,
}

// toString for CellState enum
impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CellState::X => write!(f, "X"),
            CellState::O => write!(f, "O"),
            CellState::None => write!(f, " "),
        }
    }
}

#[derive(Copy, Clone)]
struct Table {
    cells: [CellState; 9],
}

// Implimentation of Table struct
impl Table {
    // Constructor
    fn new_empty() -> Self {
        Self {
            cells: [CellState::None; 9],
        }
    }

    // Sets the cell in the given row / column to the given state
    fn set_cell(&mut self, row: u32, col: u32, state: CellState) {
        let index: usize = usize::try_from(col + row * 3).unwrap();
        self.cells[index] = state;
    }

    // Checks an input to make sure it is valid
    fn is_valid(self, row: u32, col: u32) -> bool {
        let index: usize = usize::try_from(col + row * 3).unwrap();
        match self.cells[index] {
            CellState::None => true,
            _ => false,
        }
    }

    // Checks if the specified player won
    fn check_winner(self, state: CellState) -> bool {
        // Checks diagonals
        if self.cells[0] == state && self.cells[4] == state && self.cells[8] == state {
            return true;
        }
        if self.cells[2] == state && self.cells[4] == state && self.cells[6] == state {
            return true;
        }

        // Checks rows
        for r in 0..=2 {
            let mut won: bool = true;
            for c in 0..=2 {
                if self.cells[r * 3 + c] != state {
                    won = false;
                }
            }
            if won {
                return true;
            }
        }

        // Checks columns
        for c in 0..=2 {
            let mut won: bool = true;
            for r in 0..=2 {
                if self.cells[r * 3 + c] != state {
                    won = false;
                }
            }
            if won {
                return true;
            }
        }
        return false;
    }

    // Checks if the table has any spaces left
    fn is_full(self) -> bool {
        for i in 0..9 {
            if self.cells[i] == CellState::None {
                return false;
            }
        }
        return true;
    }
}

// toString for Table struct
impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
     [1] [2] [3]
            
[A]   {} | {} | {} 
     ---|---|--- 
[B]   {} | {} | {} 
     ---|---|--- 
[C]   {} | {} | {} ",
            self.cells[0].to_string(),
            self.cells[1].to_string(),
            self.cells[2].to_string(),
            self.cells[3].to_string(),
            self.cells[4].to_string(),
            self.cells[5].to_string(),
            self.cells[6].to_string(),
            self.cells[7].to_string(),
            self.cells[8].to_string()
        )
    }
}

// Main
fn main() {
    let mut table: Table = Table::new_empty();
    let mut x: bool = false;
    let mut player_turn: bool = false;
    print!("\nChoose X or O: ");
    loop {
        let input: String = read!("{}\n");
        if input.chars().nth(0) == Some('X') {
            x = true;
            player_turn = true;
            break;
        }
        if input.chars().nth(0) == Some('O') {
            break;
        }
        print!("Please enter \'X\' or \'O\':");
    }

    // Game loop
    loop {
        println!("\n---------------------\n{}\n", table);

        // Check for winners
        if table.check_winner(CellState::X) {
            if x {
                // lmao you can't win
                println!("Player wins!\n");
            } else {
                println!("CPU wins!\n");
            }
            break;
        } else if table.check_winner(CellState::O) {
            if !x {
                // impossible!
                println!("Player wins!\n");
            } else {
                println!("CPU wins!\n");
            }
            break;
        } else if table.is_full() {
            println!("Tie game.\n");
            break;
        }

        // Finding out who's turn it is
        if player_turn {
            table = get_player_move(table, x);
            player_turn = false;
        } else {
            table = get_cpu_move(table, x);
            player_turn = true;
        }
    }
}

// Gets move from player
fn get_player_move(mut table: Table, x: bool) -> Table {
    let mut row: u32;
    let mut col: u32;
    print!("Enter your move: ");
    loop {
        let input: String = read!("{}\n");
        match input.chars().nth(0) {
            Some('A') => row = 0,
            Some('B') => row = 1,
            Some('C') => row = 2,
            _ => row = 3,
        }
        match input.chars().nth(1) {
            Some('1') => col = 0,
            Some('2') => col = 1,
            Some('3') => col = 2,
            _ => col = 3,
        }
        if table.is_valid(row, col) && row < 3 && col < 3 {
            if x {
                table.set_cell(row, col, CellState::X);
            } else {
                table.set_cell(row, col, CellState::O);
            }
            break;
        }
        print!("Invalid input. Enter a valid pair (A1, B2, C1, etc.): ");
    }
    return table;
}

// Gets move for computer player
fn get_cpu_move(mut table: Table, x: bool) -> Table {
    let cpu_move: (u32, u32) = find_best_move(table, x);
    if x {
        table.set_cell(cpu_move.0, cpu_move.1, CellState::O);
    } else {
        table.set_cell(cpu_move.0, cpu_move.1, CellState::X);
    }
    let cpu_row;
    let cpu_col;
    match cpu_move.0 {
        0 => cpu_row = 'A',
        1 => cpu_row = 'B',
        2 => cpu_row = 'C',
        _ => cpu_row = '.',
    }
    match cpu_move.1 {
        0 => cpu_col = '1',
        1 => cpu_col = '2',
        2 => cpu_col = '3',
        _ => cpu_col = '.',
    }

    println!("CPU move: {}{}.", cpu_row, cpu_col);
    return table;
}

// Evaluates a position for use in the minimax algorithm
fn evaluate(table: Table, x: bool) -> i32 {
    if x {
        if table.check_winner(CellState::X) {
            return -10;
        } else if table.check_winner(CellState::O) {
            return 10;
        }
    } else {
        if table.check_winner(CellState::X) {
            return 10;
        } else if table.check_winner(CellState::O) {
            return -10;
        }
    }
    return 0;
}

// A recursive method for finding the best move
fn minimax(mut table: Table, depth: u32, is_max: bool, x: bool) -> i32 {
    let score: i32 = evaluate(table, x);

    // If maximizer or minimizer has won the game, return score
    if score == 10 || score == -10 {
        return score;
    }
    // Return 0 score for tie
    if table.is_full() {
        return 0;
    }

    if is_max {
        let mut best = -1000;

        // Traverse all cells
        for r in 0..3 {
            for c in 0..3 {
                if table.cells[r * 3 + c] == CellState::None {
                    if x {
                        table.set_cell(
                            u32::try_from(r).unwrap(),
                            u32::try_from(c).unwrap(),
                            CellState::O,
                        );
                    } else {
                        table.set_cell(
                            u32::try_from(r).unwrap(),
                            u32::try_from(c).unwrap(),
                            CellState::X,
                        );
                    }
                    best = cmp::max(best, minimax(table, depth + 1, !is_max, x));
                    table.set_cell(
                        u32::try_from(r).unwrap(),
                        u32::try_from(c).unwrap(),
                        CellState::None,
                    );
                }
            }
        }
        return best;
    } else {
        let mut best = 1000;

        // Traverse all cells
        for r in 0..3 {
            for c in 0..3 {
                if table.cells[r * 3 + c] == CellState::None {
                    if x {
                        table.set_cell(
                            u32::try_from(r).unwrap(),
                            u32::try_from(c).unwrap(),
                            CellState::X,
                        );
                    } else {
                        table.set_cell(
                            u32::try_from(r).unwrap(),
                            u32::try_from(c).unwrap(),
                            CellState::O,
                        );
                    }
                    best = cmp::min(best, minimax(table, depth + 1, !is_max, x));
                    table.set_cell(
                        u32::try_from(r).unwrap(),
                        u32::try_from(c).unwrap(),
                        CellState::None,
                    );
                }
            }
        }
        return best;
    }
}

// Uses the minimax algorithm to find best possible move
fn find_best_move(mut table: Table, x: bool) -> (u32, u32) {
    let mut best_val: i32 = -1000;
    let mut best_move: (u32, u32) = (3, 3);

    // Traverses all empty cells
    for r in 0..3 {
        for c in 0..3 {
            if table.cells[r * 3 + c] == CellState::None {
                if x {
                    table.set_cell(
                        u32::try_from(r).unwrap(),
                        u32::try_from(c).unwrap(),
                        CellState::O,
                    );
                } else {
                    table.set_cell(
                        u32::try_from(r).unwrap(),
                        u32::try_from(c).unwrap(),
                        CellState::X,
                    );
                }

                let move_val = minimax(table, 0, false, x);
                table.set_cell(
                    u32::try_from(r).unwrap(),
                    u32::try_from(c).unwrap(),
                    CellState::None,
                );

                if move_val > best_val {
                    best_move = (u32::try_from(r).unwrap(), u32::try_from(c).unwrap());
                    best_val = move_val;
                }
            }
        }
    }
    return best_move;
}

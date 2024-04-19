use std::io;
use std::io::{stdout, Write};

const BOARD_SIZE: usize = 3;

// Mapping of place numbers to board positions
const PLACES: &[(i32, [usize; 2])] = &[
    (1, [0, 0]),
    (2, [0, 1]),
    (3, [0, 2]),
    (4, [1, 0]),
    (5, [1, 1]),
    (6, [1, 2]),
    (7, [2, 0]),
    (8, [2, 1]),
    (9, [2, 2]),
];

/// Draws the game board to the provided output.
///
/// # Arguments
///
/// * `out` - The output to draw the board to.
/// * `board` - The current state of the game board.
fn draw_board(out: &mut impl Write, board: &Vec<Vec<char>>) {
    writeln!(out, "").unwrap();

    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            match board[i][j] {
                character => write!(out, " {} ", character).unwrap(),
            }
            if j != BOARD_SIZE - 1 {
                write!(out, "|").unwrap();
            }
        }
        if i != BOARD_SIZE - 1 {
            writeln!(out, "\n{}", "---+".repeat(BOARD_SIZE - 1) + "---").unwrap();
        }
    }

    out.flush().unwrap();
    writeln!(out, "\n").unwrap();
}


/// Checks if the same player has played at both positions.
///
/// # Arguments
///
/// * `a` - The first position to check.
/// * `b` - The second position to check.
/// * `board` - The current state of the game board.
///
/// # Returns
///
/// * `bool` - Whether the same player has played at both positions.
fn is_same_play(a: &usize, b: &usize, board: &Vec<Vec<char>>) -> bool {
    board[a / BOARD_SIZE][a % BOARD_SIZE] == board[b / BOARD_SIZE][b % BOARD_SIZE]
}

/// Checks if there is a winner on the board.
///
/// # Arguments
///
/// * `board` - The current state of the game board.
///
/// # Returns
///
/// * `char` - The character of the winner, or ' ' if there is no winner.
fn check_winner(board: &Vec<Vec<char>>) -> char {
    let mut winner = ' ';
    let positions_winner = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8],
        [0, 3, 6], [1, 4, 7], [2, 5, 8],
        [0, 4, 8], [2, 4, 6],
    ];

    for pos in positions_winner.iter() {
        let [a, b, c] = pos;
        if is_same_play(a, b, board) && is_same_play(a, c, board) {
            winner = board[a / BOARD_SIZE][a % BOARD_SIZE];
            break;
        }
    }

    winner
}

/// Reads a move from the player and updates the board.
///
/// # Arguments
///
/// * `board` - The current state of the game board.
/// * `player` - The current player.
fn read_move(board: &mut Vec<Vec<char>>, player: i32) {
    let mut input = String::new();
    print!("Enter your move from 1 to 9: ");
    stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    // if input.trim().is_empty() || input.trim().len() > 1 || !input.trim().parse::<f64>().is_ok() {
    //     println!("Invalid input {}!", input.trim());
    //     read_move(board, player);
    // }

    let place = input.trim().parse::<i32>().unwrap();

    match read_place(place) {
        Err(_) => {
            println!("Invalid move {}!", place);
            read_move(board, player);
        }
        Ok([row, col]) if board[row][col] != ' ' => {
            println!("Cell {} is already taken!", place);
            read_move(board, player);
        }
        _ => {
            let [row, col] = read_place(place).unwrap();
            let draw_player = if player % 2 == 0 { 'X' } else { 'O' };
            board[row][col] = draw_player;
        }
    }
}

/// Converts a place number to a board position.
///
/// # Arguments
///
/// * `place` - The place number to convert.
///
/// # Returns
///
/// * `Result<[usize; 2], ()>` - The board position, or an error if the place number is invalid.
fn read_place(place: i32) -> Result<[usize; 2], ()> {
    match PLACES.iter().find(|&&(p, _)| p == place) {
        Some(&(_, pos)) => Ok(pos),
        _ => Err(()),
    }
}

fn main() {
    let mut board: Vec<Vec<char>> = vec![vec![' '; BOARD_SIZE], vec![' '; BOARD_SIZE], vec![' '; BOARD_SIZE]];
    let mut player = 0;

    loop {
        if player == 9 {
            println!("\nIt's a draw!");
            break;
        }

        read_move(&mut board, player);
        draw_board(&mut stdout(), &board);

        let winner = check_winner(&board);
        if winner != ' ' {
            println!("Player {} wins!", winner);
            break;
        }

        player += 1;
    }
}
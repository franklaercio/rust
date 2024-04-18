use std::io;
use std::io::{stdout, Write};

fn draw_board(out: &mut impl Write, board: &Vec<Vec<Option<char>>>) {
    let board_size = board.len();
    for i in 0..board_size {
        for j in 0..board_size {
            match board[i][j] {
                Some(character) => write!(out, " {} ", character).unwrap(),
                None => write!(out, "   ").unwrap(),
            }
            if j != board_size - 1 {
                write!(out, "|").unwrap();
            }
        }
        if i != board_size - 1 {
            writeln!(out, "\n{}", "---+".repeat(board_size - 1) + "---").unwrap();
        }
    }

    out.flush().unwrap();
}

fn check_winner(board: &Vec<Vec<Option<char>>>) -> Option<char> {
    let board_size = board.len();
    let mut winner = None;

    for i in 0..board_size {
        if let Some(character) = board[i][0] {
            if board[i].iter().all(|&cell| cell == Some(character)) {
                winner = Some(character);
            }
        }
        if let Some(character) = board[0][i] {
            if (0..board_size).all(|j| board[j][i] == Some(character)) {
                winner = Some(character);
            }
        }
    }

    if let Some(character) = board[0][0] {
        if (0..board_size).all(|i| board[i][i] == Some(character)) {
            winner = Some(character);
        }
    }
    if let Some(character) = board[0][board_size - 1] {
        if (0..board_size).all(|i| board[i][board_size - 1 - i] == Some(character)) {
            winner = Some(character);
        }
    }

    winner
}

fn read_move(board: &mut Vec<Vec<Option<char>>>, player: usize) {
    let mut input = String::new();
    print!("Enter your move (row column): ");
    stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let row = parts[0].parse::<usize>().unwrap();
    let col = parts[1].parse::<usize>().unwrap();

    let player = if player % 2 == 0 { 'X' } else { 'O' };
    board[row][col] = Some(player);
}

fn main() {
    let mut board: Vec<Vec<Option<char>>> = vec![vec![None; 3]; 3];

    let mut player = 0;

    loop {
        read_move(&mut board, player);

        draw_board(&mut stdout(), &board);

        match check_winner(&board) {
            Some(winner) => {
                println!("\nPlayer {} has won!", winner);
                break;
            }
            None => (),
        }

        player += 1;
    }
}
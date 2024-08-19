use std::{
    io::{self, Write},
    process::exit,
};

const BOARD_HEIGHT: usize = 3;
const BOARD_WIDTH: usize = 3;
const INPUT_VALUE_X: char = 'X';
const INPUT_VALUE_O: char = 'O';
const INPUT_VALUE_NOTHING: char = '-';

struct GameState {
    board: [[char; BOARD_WIDTH]; BOARD_HEIGHT],
}

fn display_initial_message() {
    println!("This is a tic tac toe game.");
    println!("You will be playing against the computer.");
    println!("Below is a board with its coordinates displayed for you.");
    show_coordinates();
}

fn show_coordinates() {
    for i in 0..BOARD_HEIGHT {
        for j in 0..BOARD_WIDTH {
            print!("{},{} ", i, j)
        }
        println!();
        if i < BOARD_HEIGHT {
            println!("---|---|---");
        }
    }
}

fn display_state(state: &GameState) {
    println!();
    println!("=======================================");
    for i in 0..BOARD_HEIGHT {
        for j in 0..BOARD_WIDTH {
            print!(" {} ", state.board[i][j]);
        }
        println!();
    }
    println!("=======================================");
}

fn user_move(state: &mut GameState) {
    let (x, y) = loop {
        println!("Enter x, y coordinates for your move (0-indexed):");
        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Parse input into two integers
        let input = input.trim();
        let coords: Vec<&str> = input.split(',').collect();

        if coords.len() == 2 {
            if let (Ok(x), Ok(y)) = (coords[0].parse::<usize>(), coords[1].parse::<usize>()) {
                if x < BOARD_HEIGHT && y < BOARD_WIDTH {
                    break (x, y);
                } else {
                    println!("Coordinates out of bounds. Please try again.");
                }
            } else {
                println!("Invalid input. Please enter valid numbers.");
            }
        } else {
            println!("Please enter the coordinates in the format: x,y");
        }
    };

    // Check if the selected cell is already occupied
    if state.board[x][y] != INPUT_VALUE_NOTHING {
        println!("Invalid move. Cell is already occupied. Try again.");
        user_move(state);
    } else {
        state.board[x][y] = INPUT_VALUE_X; // Mark the move on the board
    }
}

fn computer_move(state: &mut GameState) {
    let mut is_draw: bool = true;

    for i in 0..BOARD_HEIGHT {
        for j in 0..BOARD_WIDTH {
            if state.board[i][j] == INPUT_VALUE_NOTHING {
                is_draw = false;
                state.board[i][j] = INPUT_VALUE_O;
                break;
            }
        }
        if !is_draw {
            break;
        }
    }

    if is_draw {
        println!("draw!");
        exit(0);
    }
}

fn check(state: &GameState) -> char {
    let rows = state.board.len();
    let cols = state.board[0].len();

    // Check rows
    for i in 0..rows {
        let first = state.board[i][0];
        if first != INPUT_VALUE_NOTHING && state.board[i].iter().all(|&c| c == first) {
            return first;
        }
    }

    // Check columns
    for i in 0..cols {
        let first = state.board[0][i];
        if first != INPUT_VALUE_NOTHING && (0..rows).all(|j| state.board[j][i] == first) {
            return first;
        }
    }

    // Check main diagonal
    if rows == cols
        && (0..rows).all(|i| {
            state.board[i][i] == state.board[0][0] && state.board[0][0] != INPUT_VALUE_NOTHING
        })
    {
        return state.board[0][0];
    }

    // Check anti-diagonal
    if rows == cols
        && (0..rows).all(|i| {
            state.board[i][cols - 1 - i] == state.board[0][cols - 1]
                && state.board[0][cols - 1] != INPUT_VALUE_NOTHING
        })
    {
        return state.board[0][cols - 1];
    }

    // No winner
    INPUT_VALUE_NOTHING
}

fn main() {
    let mut state = GameState {
        board: [[INPUT_VALUE_NOTHING; BOARD_WIDTH]; BOARD_HEIGHT],
    };

    display_initial_message();

    println!("Press ENTER to continue...");
    let mut input = String::new();
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).unwrap();

    let mut winner: char;

    loop {
        display_state(&state);

        user_move(&mut state);
        display_state(&state);
        winner = check(&state);
        if winner != INPUT_VALUE_NOTHING {
            break;
        }

        computer_move(&mut state);
        winner = check(&state);
        if winner != INPUT_VALUE_NOTHING {
            break;
        }
    }

    if winner == INPUT_VALUE_X {
        println!("You won!");
    } else {
        display_state(&state);
        println!("You lost!");
    }
}

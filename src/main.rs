use std::io;

fn main() {
    let mut board = [' '; 9];
    let mut current_player = 'X';

    loop {
        print_board(&board);
        let position = get_player_input(current_player);
        board[position] = current_player;

        if check_win(&board, current_player) {
            println!("Player {} wins!", current_player);
            break;
        }

        if board.iter().all(|&x| x != ' ') {
            println!("It's a tie!");
            break;
        }

        current_player = switch_player(current_player);
    }
}

fn print_board(board: &[char; 9]) {
    println!(" {} | {} | {}", board[0], board[1], board[2]);
    println!("---+---+---");
    println!(" {} | {} | {}", board[3], board[4], board[5]);
    println!("---+---+---");
    println!(" {} | {} | {}", board[6], board[7], board[8]);
}

fn get_player_input(player: char) -> usize {
    loop {
        println!("Player {}: enter your move (1-9):", player);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let position = match input.trim().parse() {
            Ok(num) if num >= 1 && num <= 9 => num - 1,
            _ => {
                println!("Invalid input, try again.");
                continue;
            }
        };

        if position_is_available(position, player) {
            return position;
        } else {
            println!("That position is already taken, try again.");
        }
    }
}

fn position_is_available(position: usize, player: char) -> bool {
    position >= 0 && position < 9 && board[position] == ' '
}

fn check_win(board: &[char; 9], player: char) -> bool {
    (board[0] == player && board[1] == player && board[2] == player) ||
    (board[3] == player && board[4] == player && board[5] == player) ||
    (board[6] == player && board[7] == player && board[8] == player) ||
    (board[0] == player && board[3] == player && board[6] == player) ||
    (board[1] == player && board[4] == player && board[7] == player) ||
    (board[2] == player && board[5] == player && board[8] == player) ||
    (board[0] == player && board[4] == player && board[8] == player) ||
    (board[2] == player && board[4] == player && board[6] == player)
}

fn switch_player(player: char) -> char {
    if player == 'X' {
        'O'
    } else {
        'X'
    }
}

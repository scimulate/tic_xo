mod board;
mod marker;
mod rc_flatten;

use marker::marker;
use rc_flatten::rc_flatten;

fn main()
{
    let mut board = [0i8; 9];
    let mut player = 1;

    let mut moves = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    board[3] = 1;
    board[0] = 1;
    board[8] = 1;
    board[4] = -1;
    //board[6] = 1;

    //println!("Test, {}, {}, {}", marker(-1), marker(0), marker(1));

    board::print_board(& board);
    board::check_board(& board);

    /*
    let mut ct = 0;
    while !board::check_board(& board)
    {
        board[ct] = player;
        ct += 1;
        player *= -1;
        println!("Player {}", marker::marker(player));
    }
    */

    /*
    play();

    for ct in -1..=1
    {
        println!("{}", marker(ct));
    }

    board[2] = 1;
    board[4] = 1;
    board[6] = 1;
    board[5] = 1;
    //board::print_board(& board);
    board::check_board(& board);

    //check_board(&board);
    */
}

fn play()
{
    //print_board();
}

fn play_headless(board: &[i8], player: i8)
{
    //print_board();
}
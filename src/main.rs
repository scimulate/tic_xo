mod board;
mod marker;
mod rc_flatten;

use marker::marker;
use rc_flatten::rc_flatten;
use std::io;

fn main()
{
    let mut board = [0i8; 9];
    let mut player = -1;

    /*
    let mut moves = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    board[3] = 1;
    board[0] = 1;
    board[8] = 1;
    board[4] = -1;
    board[6] = 1;
    */

    let mut ct = 0;
    while !board::check_board(& board) && ct < 9
    {
        player *= -1;

        ct += 1;
    }


    //println!("Test, {}, {}, {}", marker(-1), marker(0), marker(1));

    board::print_board(& board);
    board::check_board(& board);
}
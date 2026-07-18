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
    let mut row = 0;
    let mut col = 0;
    let mut ct = 0;
    let mut input = String::new();
    let mut parts: Vec<&str>;// = input.split(',').collect();

    while !board::check_board(& board) && ct < 9
    {
    /*
    let mut moves = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    board[3] = 1;
    board[0] = 1;
    board[8] = 1;
    board[4] = -1;
    board[6] = 1;
    */

        player *= -1;
        input = String::new();
        println!("Player {}, [row,col]: ", marker(player));
        io::stdin().read_line(&mut input).expect("Failed to read line");
        parts = input.split(',').collect();
        println!("You typed: {}", input.trim());
        
        println!("Row: {}, Col: ", parts.get(0).and_then(|s| s.trim().parse::<i32>().ok()));

        ct += 1;
    }


    //println!("Test, {}, {}, {}", marker(-1), marker(0), marker(1));

    board::print_board(& board);
    board::check_board(& board);
}
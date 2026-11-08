mod board;
mod marker;
mod rc_flatten;

use marker::marker;
use rc_flatten::rc_flatten;
use std::io;

//use crate::board::{check_board, print_board};

fn main()
{
    let mut board = [0i8; 9];
    let mut player = -1;
    let mut row;
    let mut col;
    let mut ct = 0;
    let mut input;
    let mut parts: Vec<&str>;

    while !board::check_board(& board) && ct < 9
    {
        player *= -1;
        input = String::new();
        println!("Player {}, [row,col]: ", marker(player));
        io::stdin().read_line(&mut input).expect("Failed to read line");
        parts = input.split(',').collect();

        row = parts.get(0).expect("Missing Row").trim().parse().expect("Invalid Row");
        col = parts.get(1).expect("Missing Col").trim().parse().expect("Invalid Col");
        board[rc_flatten(row, col)] = player;
        board::print_board(&board);

        ct += 1;
    }

    board::print_board(& board);
    board::check_board(& board);
}
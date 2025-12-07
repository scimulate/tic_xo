use crate::marker;
use crate::rc_flatten;

pub fn check_board(board: &[i8]) -> bool
{
    // The board data is designed to make it trivial to check for a winner. If
    // any row, column, or diagonal adds to |3|, there is a winner. Otherwise,
    // no winner yet. If the board is full and the game is a draw, have that 

    let mut game_won = false;
    let mut total = 0;

    // Check horizontals
    for ct in 1..=3
    {
        total += board[rc_flatten(0, (ct-1).try_into().unwrap())];
        if total.abs() == 3
        {
            game_won = true;
        }
    }

    println!("{}", total);
    // Check verticals
    // Check diagonals

    true
}

pub fn print_board(board: &[i8])
{
    println!("Tic_XO Current Board:");
    for ct in 0..9
    {
        print!("{}", marker::marker(board[ct]));
        if (ct+1) % 3 == 0
        {
            println!();
        }
    }
}
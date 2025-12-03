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
    //total = board[rc_flatten(0,0)];

    for ct in 1..=3
    {
        board[ct];
        //i8::try_from(ct).ok();
        //rc_flatten(0, i8::try_from(ct).unwrap());// .ok());
        board(rc_flatten(0, i8::try_from(ct).unwrap())];
        //total += rc_flatten(0, ct);
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
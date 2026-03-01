use crate::marker;
use crate::rc_flatten;

pub fn check_board(board: &[i8]) -> bool
{
    // The board data is designed to make it trivial to check for a winner. If
    // any row, column, or diagonal adds to |3|, there is a winner. Otherwise,
    // no winner yet.

    let mut total;

    // Check Horizontals & Verticals
    for flipped in [true, false]
    {
        for dim0 in 0..3
        {
            total = 0;
            for dim1 in 0..3
            {
                if flipped
                {
                    total += board[rc_flatten(dim0, dim1)];
                }
                else
                {
                    total += board[rc_flatten(dim1, dim0)];
                }
            }
            if total.abs() == 3
            {
                println!("Game won, player {}", marker::marker(total/3));
                return true;
            }
        }
    }

    // Check diagonals (negative slope)
    total = 0;
    for ct in 0..3
    {
        total += board[rc_flatten(ct, ct)];
    }
    if total.abs() == 3
    {
        println!("Game won, player {}", marker::marker(total/3));
        return true;
    }

    // Check diagonals (positive slope)
    total = 0;
    for ct in 0..3
    {
        total += board[2*ct];
    }
    if total.abs() == 3
    {
        println!("Game won, player {}", marker::marker(total/3));
        return true;
    }

    return false;
}

pub fn print_board(board: &[i8])
{
    println!("Tic_XO Current Board:");
    for ct in 0..3
    {
        println!("{} {} {}", marker::marker(board[3*ct]),
                             marker::marker(board[3*ct+1]),
                             marker::marker(board[3*ct+2]));
    }
}
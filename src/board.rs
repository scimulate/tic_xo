use crate::marker;
use crate::rc_flatten;

pub fn check_board(board: &[i8]) -> bool
{
    // The board data is designed to make it trivial to check for a winner. If
    // any row, column, or diagonal adds to |3|, there is a winner. Otherwise,
    // no winner yet.

    let mut game_won = false;
    let mut total = 0;

    // Check horizontals
    for row in 0..3
    {
        total = 0;
        for col in 0..3
        {
            total += board[rc_flatten(row, col)];
            if total.abs() == 3
            {
                println!("Game won, row {}, player {}", row, marker::marker(total/3));
                game_won = true;
            }
        }
    }

    // Check verticals
    for col in 0..3
    {
        total = 0;
        for row in 0..3
        {
            total += board[rc_flatten(row, col)];
            if total.abs() == 3
            {
                println!("Game won, col {}, player {}", col, marker::marker(total/3));
                game_won = true;
            }
        }
    }

    // Check diagonals (negative slope)
    total = 0;
    for ct in 0..3
    {
        total += board[rc_flatten(ct, ct)];
        if total.abs() == 3
        {
            println!("Game won, diag 1, player {}", marker::marker(total/3));
            game_won = true;
        }
    }

    // Check diagonals (positive slope)
    total = 0;
    for ct in 0..3
    {
        total += board[rc_flatten(ct, 2-ct)];
        if total.abs() == 3
        {
            println!("Game won, diag 1, player {}", marker::marker(total/3));
            game_won = true;
        }
    }

    

    game_won
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
use crate::marker;

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
mod board;
mod marker;
mod rc_flatten;

use marker::marker;
use rc_flatten::rc_flatten;

fn main()
{
    let mut board = [0i8; 9];
    //play(&board);

    for ct in -1..=1
    {
        println!("{}", marker(ct));
    }

    board[0] = -1;
    board[1] = -1;
    board[2] = -1;
    board[5] = 1;
    //board::print_board(& board);
    board::check_board(& board);

    /*
    for row in 0..3
    {
        for col in 0..3
        {
            println!("{}", rc_flatten(row, col));
        }
    }
    */

    //check_board(&board);
}

/*


fn play()
{
    //printBoard();
}

fn play_headless()
{

}
*/
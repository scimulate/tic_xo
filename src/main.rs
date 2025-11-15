mod board;
mod marker;

fn main()
{
    let mut board = [0i8; 9];
    //play(&board);
    board[0] = -1;
    board[5] = 1;
    board::print_board(& board);
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
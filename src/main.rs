fn main()
{
    let mut board = [0i8; 9];
    //play(&board);
    board[0] = 4;
    print_board(& board);
}

fn print_board(board: &[i8])
{
    println!("Tic_XO Current Board:");
    println!("{}{}{}\n{}{}{}\n{}{}{}", marker(board[0]),
                                       marker(board[1]),
                                       marker(board[2]),
                                       marker(board[3]),
                                       marker(board[4]),
                                       marker(board[5]),
                                       marker(board[6]),
                                       marker(board[7]),
                                       marker(board[8]));
}

fn play()
{
    //printBoard();
}

fn play_headless()
{

}

fn marker(val:i8) -> char
{
    if val < 0
    {
        'O'
    }
    else if val == 0
    {
        ' '
    }
    else
    {
        'X'
    }
}
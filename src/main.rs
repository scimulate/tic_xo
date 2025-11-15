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

fn check_board(board: &[i8])// -> bool
{
    // The board data is designed to make it trivial to check for a winner. If
    // any row, column, or diagonal adds to |3|, there is a winner. Otherwise,
    // no winner yet. If the board is full and the game is a draw, have that 

    // Check horizontals
    //let mut ct = 0;
    for ct in 1..=10
    {
        println!("{}", ct);
    }
    // Check verticals
    // Check diagonals
}

fn play()
{
    //printBoard();
}

fn play_headless()
{

}


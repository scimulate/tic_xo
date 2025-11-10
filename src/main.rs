mod marker;

fn main()
{
    let mut board = [0i8; 9];
    //play(&board);
    board[0] = -1;
    board[5] = 1;
    print_board(& board);
    //check_board(&board);
}

fn marker(index: i8) -> char
{
    let x_o = "O X";
    x_o.chars().nth((index+1) as usize).unwrap()
}

fn print_board(board: &[i8])
{
    println!("Tic_XO Current Board:");
    for ct in 0..9
    {
        print!("{}", marker(board[ct]));
        if (ct+1) % 3 == 0
        {
            println!();
        }
    }
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


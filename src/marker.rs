pub fn marker(index: i8) -> char
{
    /* 
    This function accepts a value (-1, 0, +1) and returns either tic-tac-toe
    character or a blank space: "O", " ", or "X".
    
    Future versions should probably error check since any i8 can be passed into
    this function.
    */
    
    let x_o = "O-X";
    x_o.chars().nth((index+1) as usize).unwrap()
}
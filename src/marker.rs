pub fn marker(index: i8) -> char
{
    let x_o = "O X";
    x_o.chars().nth((index+1) as usize).unwrap()
}
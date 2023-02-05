enum Direction{
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let player_direction:Direction = Direction::Up;
    //let player_direction:Direction = Direction::Down;
    //let player_direction:Direction = Direction::Left;
    //let player_direction:Direction = Direction::Right;

    // Hangisi aktif ise o match olacak ekranda ona ait yazı cıkacak

    match player_direction{
        Direction::Up => println!("we are heading up!"),
        Direction::Down => println!("we are falling down!"),
        Direction::Right => println!("we are going right!"),
        Direction::Left => println!("we are going left!"),
    }
}

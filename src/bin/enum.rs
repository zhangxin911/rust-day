enum Direction {
    Left,
    Right,
    Up,
}

enum Color {
    Red,
    Yellow,
    Blue,
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Yellow => println!("Yellow"),
        Color::Blue => println!("Blue"),
    }
}

fn main() {
    let go = Direction::Left;
    match go {
        Direction::Left => println!("go left!"),
        Direction::Right => println!("go right"),
        Direction::Up => println!("go up"),
    }
    print_color(Color::Blue);
}

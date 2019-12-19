// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!


#[derive(Debug)]
enum Message {
    Quit(i32),
    Move,
    Write,
    Echo(String),
    ChangeColor
    // TODO: define a few types of messages as used below
}

fn main() {
    println!("{:?}", Message::Quit(12));
    println!("{:?}", Message::Echo("apple".to_string()));
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}

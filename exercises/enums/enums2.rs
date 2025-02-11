// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    Move {x: i16, y: i16},
    Echo(String),
    ChangeColor(i16, i16, i16),
    Quit
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    println!("Testing message:");
    for message in &messages {
        message.call();
    }
}

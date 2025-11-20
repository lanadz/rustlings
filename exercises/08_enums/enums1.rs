#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize(f32),
    Move {x: i32, y: i32},
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit
}

fn main() {
    println!("{:?}", Message::Resize(3.1));
    println!("{:?}", Message::Move{x: 1, y: 3});
    println!("{:?}", Message::Echo(String::from("echo")));
    println!("{:?}", Message::ChangeColor(0,0,255));
    println!("{:?}", Message::Quit);
}

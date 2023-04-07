mod implement_none;
mod r#match;

// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// ? SHORTHAND METHOD
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// enum vs structs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// ! -----[Same thing in struct]
// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct
// ! [Problem : we can't use all these structs in one function]
// * PROS : we can also define methods 'impl' for enums same as structs

impl Message {
    fn call(&self) {
        // Anything
    }
}

fn main() {
    println!("Hello, world!");

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // ? SHORTHAND METHOD
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from(""));
    m.call();

    r#match::main();
    implement_none::main();
}

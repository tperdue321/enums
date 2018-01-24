// exploring enums
// taken from https://doc.rust-lang.org/book/second-edition/ch06-01-defining-an-enum.html
// author: Travis Perdue

#![allow(unused_variables)]
// basic enum
enum IpAddrKind {
    V4,
    V6,
}

// basic struct
#[allow(dead_code)]
struct IpAddr {
    kind: IpAddrKind,
    addr: String
}

// better enum to replace enum/struct combo
#[allow(dead_code)]
enum BetterIpAddr {
    V4(String),
    V6(String)
}

// V4 always has 4 nums 0-255 so we can make an enum with a tuple like so
#[allow(dead_code)]
enum OtherBetterIpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

// enums can have many values inside them including anonymous structs!
#[allow(dead_code)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

// example of using structs for the various data variants from the
// Message enum
// problem is that it isn't easy to create one function that accepts
// all these structs but we can easily implement a function for the
// Message enum
#[allow(dead_code)]
struct QuitMessage; // unit struct
#[allow(dead_code)]
struct MoveMessage {
    x: i32,
    y: i32,
}
#[allow(dead_code)]
struct WriteMessage(String); // tuple struct
#[allow(dead_code)]
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        println!("inside 'call' function for Message enum");
    }
}

fn main() {
    // basic assignments. no surprises here.
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1")
    }; 

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        addr: String::from("::1")
    };

    let better_home = BetterIpAddr::V4(String::from("127.0.0.1"));
    let better_loopback = BetterIpAddr::V6(String::from("::1"));


    let other_beter_home = OtherBetterIpAddr::V4(127,0,0,1);
    let other_better_loopback = OtherBetterIpAddr::V6(String::from("::1"));

    let message = Message::Write(String::from("message"));
    message.call();

    // variables using the Option enum
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // this will not compile essentially because you have to get the <T> out of Option<T>
    // before you can operate it so i8 + Option<i8> doesn't work.
    // let sum = x + y;
}


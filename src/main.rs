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


#[derive(Debug)] // So we can inspect the state
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

// use to explore Rust match
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
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

    let penny = Coin::Penny;
    let cents = value_in_cents(penny);
    println!("a penny's value is: {}", cents);

    let quarter = Coin::Quarter(UsState::Alaska);
    let cents = value_in_cents(quarter);

    // this will not compile essentially because you have to get the <T> out of Option<T>
    // before you can operate it so i8 + Option<i8> doesn't work.
    // let sum = x + y;

    // examples of pattern matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("some {:?}", five);
    println!("none {:?}", none);

    // dealing with some cases but not all by using _
    // without _ Rust expects an exhaustive match
    let some_u8_value: u8 = 0;  

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => ()
    }

    let some_u8_value = Some(3u8);
    // match only one value 
    match some_u8_value {
        Some(3) => println!("3"),
        _ => ()
    }

    let some_u8_value = Some(0u8);
    // same but more concise for matching only 1 value 
    if let Some(3) = some_u8_value {
        println!("3");
    }

    // can include an else that is comparable
    // to the code in the _ branch of a match expression
    if let Some(3) = some_u8_value {
        println!("3");
    } else {
        println!("not 3");
    }

}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>{
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

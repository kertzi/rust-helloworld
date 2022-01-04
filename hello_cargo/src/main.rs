#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size :u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

enum Message {
    Quit,
    Move { x:i32, y: i32 },
    Write(String)
}

impl Message {
    fn call(&self) {
        // plaplaa
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin  {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState)
}

fn value_in_coin(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    println!("Hello, world!");
    let mut s1 = String::from("Hellurei");

    let l = calc(&s1);

    add_str(&mut s1);

    let s2 = String::from("Apina lensi lentokoneella");

    println!("s1: {} l: {}", s1, l);

    let s3 = first_word(&s2);

    
    println!("s3: {}", s3);

    println!("Area of rect tuple {}", calc_rect_tuple((30, 50)));

    let rect = Rectangle {
        width: 30,
        height: 30
    };

    println!("Area of rect struct {}", calc_rect_struct(&rect));

    println!("Rect {:?}", rect);

    dbg!(&rect);

    println!("Area of rect {}", rect.area());

    let square = Rectangle::square(40);

    println!("Area of squre {}", square.area());

    let msg = Message::Write(String::from("hello"));

    let c = Coin::Quarter(UsState::Alaska);

    println!("Value in coin {}", value_in_coin(&c));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(value) = six {
        println!("Value {}", value);
    }

    let mut count = 0;

    if let Coin::Quarter(state) = c {
        println!("Coin is {:?} coin", state);
    }


}

fn calc(str: &String) -> usize {
    str.len()
}

fn add_str(str: &mut String) {
    str.push_str(" Added string");
}

fn first_word(str: &String) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }

    // Same as [0..len]
    &str[..]
}

fn calc_rect_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn calc_rect_struct(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
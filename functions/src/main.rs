fn main() {
    print_some(34, 'f');

    let y = {
        let x = 1;
        x + 4
    };

    println!("expression y {}", y);
    println!("five {}", five());
    println!("plus_one {}", plus_one(5));

    let num = 6;

    if num % 3 == 0 {
        println!("number is divided by 3")
    } else {
        println!("number is not divided by 3")
    }

    let _num2 = if true { 2 } else { 3 };
}

fn print_some(x: i32, c: char) {
    println!("x = {} c = {}",x, c);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x+1
}
const SOME_CONST: u32 = 4;

fn main() {
    let mut x = 5;
    x = 6;

    println!("const value {}, x val {}", SOME_CONST, x);

    let y = 5;
    let y = y + 2;

    {
        let y = 5 * 2;
        println!("Inner value {}", y);
    }

    println!("Outer value {}", y);

    let tup: (i32, f64, u8) = (500, 2.2, 2);

    let (x, y, z) = tup;

    println!("Tuple values {} {} {}", x, y, z);
    println!("Tuple values {} {} {}", tup.0, tup.1, tup.2);
}

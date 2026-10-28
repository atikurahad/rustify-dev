use std::convert::TryFrom;

fn map_rs() {
    let stock: u8 = 100;
    let result = i8::try_from(stock);

    let doubled = match result {
        Ok(value) => Ok(value * 2),
        Err(error) => Err(error),
    };

    println!("{:?}", doubled);
}

fn main() {
    let first: Result<i32, &str> = Ok(5);
    let second: Result<i32, &str> = Err("Failed");

    let a = first.map(|value| value * 2);
    let b = second.map(|value| value * 2);

    println!("{:?}", a);
    println!("{:?}", b);
}
fn main() {
    let a = String::from("Mouse");
    let b = a.clone();

    let x = 5;
    let y = x;

    println!("{a}, {b}, {x}, {y}");
}
let mut stock = 10;

let first = &mut stock;
let second = &mut stock; // Compile error

*first += 5;
*second += 2;

let mut stock = 10;

let read_stock = &stock;
let update_stock = &mut stock; 

println!("{read_stock}");
*update_stock += 5;


fn main() {
    let mut stock = 10;

    let read_stock = &stock;
    println!("{read_stock}"); // read_stock-এর শেষ ব্যবহার

    let update_stock = &mut stock;
    *update_stock += 5;

    println!("{stock}");
}

fn main() {
    let mut stock = 10;

    let first = &mut stock;
    *first += 5;

    let second = &mut stock;
    *second += 2;

    println!("{stock}");
}
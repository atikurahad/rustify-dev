use std::convert::TryFrom;

// Demonstrates fallible integer conversion and then carries the conversion's
// success or error through a match. The conversion succeeds for 100, but
// doubling that i8 value exceeds i8's range in builds that check overflow.
fn map_rs() {
    let stock: u8 = 100;
    let result = i8::try_from(stock);

    let doubled = match result {
        Ok(value) => Ok(value * 2),
        Err(error) => Err(error),
    };

    println!("{:?}", doubled);
}

// Shows how Result::map transforms only a successful value: the first result
// becomes Ok(10), while the Err value is passed through unchanged.
fn main() {
    let first: Result<i32, &str> = Ok(5);
    let second: Result<i32, &str> = Err("Failed");

    let a = first.map(|value| value * 2);
    let b = second.map(|value| value * 2);

    println!("{:?}", a);
    println!("{:?}", b);
}
// Demonstrates ownership behavior: cloning a String creates a separate owned
// value, whereas copying an integer leaves the original usable.
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

    // The immutable borrow's last use is above, so this mutable borrow is
    // allowed to begin and update the value.
    let update_stock = &mut stock;
    *update_stock += 5;

    println!("{stock}");
}

// Demonstrates that mutable borrows can be used one after another as long as
// each borrow's last use occurs before the next mutable borrow is created.
fn main() {
    let mut stock = 10;

    let first = &mut stock;
    *first += 5;

    let second = &mut stock;
    *second += 2;

    println!("{stock}");
}
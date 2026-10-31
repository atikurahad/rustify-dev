use std::convert::TryFrom;

fn conversion() {
    let number: u8 = 100;

    let converted = i8::try_from(number);

    println!("{:?}", converted);
}
fn TryFrom() {
    let number: u8 = 100;
    let result = i8::try_from(number);

    match result {
        Ok(value) => println!("Success: {}", value),
        Err(_) => println!("Conversion failed"),
    }

    println!("Program finished");
}

fn main() {
    erroTypeUnwrap();
    conversion();
    TryFrom();
    unwrapOrElse();
}

fn erroTypeUnwrap() {
    let stock: u8 = 200;

    let result = i8::try_from(stock);

    let quantity = result.unwrap_or_else(|_error| 20 + 5);
    println!("Quantity: {}", quantity);
    println!("Inventory checked");
}
fn unwrapOrElse() {
    let stock: u8 = 120;
    let result = i8::try_from(stock);

    let quantity = result.unwrap_or_else(|error| {
        println!("Error: {:?}", error);
        30
    });

    println!("Quantity: {}", quantity);
    println!("Inventory checked");
}

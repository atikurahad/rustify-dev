use std::convert::TryFrom;

fn conversion() {
    let number: u8 = 100;

    let converted = i8::try_from(number);

    println!("{:?}", converted);
}

fn main() {
    let a: i8 = 10;
    let b: u8 = 20;

    let result = a + b as i8;

    conversion();

    println!("{}", result);
}
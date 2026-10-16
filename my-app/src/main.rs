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
  

    // conversion();
    TryFrom();

}
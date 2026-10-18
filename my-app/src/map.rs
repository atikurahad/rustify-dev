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
fn get_stock() -> Result<i32, &str> {
    Err("Out of stock")
}

fn calculate_stock() -> Result<i32, &str> {
    let quantity = get_stock()?;
    Ok(quantity * 2)
}

fn main() {
    match calculate_stock() {
        Ok(value) => println!("Quantity: {value}"),
        Err(error) => println!("Error: {error}"),
    }
}
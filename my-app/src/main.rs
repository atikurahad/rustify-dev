fn main() {
    const TAX_RATE: f64 = 0.05; 
    let mut cart_total = 1000;

    cart_total = cart_total + 200; 
    println!("Cart total: {}", cart_total);
    println!("Tax rate: {}", TAX_RATE);
}


fn mini() {
    const student_id: u32 = 10; 
    let  gpa : f64 = 3.85;
    let is_passed : bool = true;
    let section : char=  'B';
    println!("{}", student_id,gpa,is_passed,section);
   
}


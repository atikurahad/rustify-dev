fn is_even(number: i32) -> bool {
    if number % 2 == 0 {
        true
    } else {
        false
    }
}

fn main() {
    let result1 = is_even(10);
    let result2 = is_even(7);
    println!("10 is even: {}", result1);
    println!("7 is even: {}", result2);
}


// fn mini() {
//     const student_id: u32 = 10; 
//     let  gpa : f64 = 3.85;
//     let is_passed : bool = true;
//     let section : char=  'B';
//     println!("{} {} {} {}", student_id, gpa, is_passed, section);
   
// }

// fn discount() {
//     let discount = "10"; 
//     let  discount : u32 = discount.parse().expect("Not a valid number ");
//     let discount = discount * 2;
    
//     println!("{}", discount);
   
// }


fn main (){
let mut count = 0;

while count == 5 {
    count += 1;
    println!("{}",count)
}
println!("Done!")
}

fn loopIteration(){

for i in 1..=10 {
    if i == 5 {
        continue;
    }
    if i == 8 {
        break;
    }
    println!("{}",i)
}

}

fn main() {
    let city = String::from("Chittagong");
    let backup = city.clone();

    println!("City: {}", city);
    println!("Backup: {}", backup);
}


fn is_valid_product_name(name: &String) -> bool {
    name.len() > 0 // name খালি না হলে valid
}

fn main() {
    let product = String::from("Rice Bag");
    let valid = is_valid_product_name(&product);

    println!("Product: {}", product); // এখনো ব্যবহার করা যাচ্ছে
    println!("Valid: {}", valid);

    // একই product আবার অন্য function-এও পাঠানো যাবে, ownership হারায়নি বলে
}

fn is_valid_product_name(name: &String) -> bool {
    name.len() > 0 // name খালি না হলে valid
}


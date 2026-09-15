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




একটা Rust program লেখো যেখানে:

for loop ব্যবহার করে 1 থেকে 10 পর্যন্ত (10 সহ) সংখ্যা iterate করবে।
যদি সংখ্যাটা 5 হয়, সেটা skip করবে (continue দিয়ে), print করবে না।
যদি সংখ্যাটা 8-এ পৌঁছায়, loop থেমে যাবে (break দিয়ে)।
বাকি সংখ্যাগুলো print করবে।

(Hint: inclusive range 1..=10 ব্যবহার করো, যেহেতু 10 সহ চাই)
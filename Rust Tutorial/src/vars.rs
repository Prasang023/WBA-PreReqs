// Var holds primitive data or references to data
// vars are immutable by default
// block-scoped language

pub fn run() {
    let name = "brad";
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);

    //Define consts
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign miltiple vars
    let (my_name, my_age) = ("brad", 37);
    println!("My name is {} and I am {}", my_name, my_age);

}
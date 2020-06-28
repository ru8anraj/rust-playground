// Variable hold primitive data or reference to a data
// Variables are immutable by default

pub fn run() {
    println!("------------vars.rs------------");
    // let - immutable and mutable
    let name = "Rubanraj";
    let mut age = 25;
    println!("My name is {} and aged {}", name, age);
    age = 26;
    println!("His new age is {}", age);

    // CONST
    // Usually const variable naming is given as all CAPITAL
    // const requires variable type or error is thrown
    const ID:i32 = 001;
    println!("ID: {}", ID);

    // multiple variables
    let (my_name, my_age) = ("Ruban", 25);
    println!("My name is {} and I'm {} years old", my_name, my_age);
}

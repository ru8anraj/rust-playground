pub fn run() {
    println!("------------print.rs------------");

    // Basic printing
    println!("Hello from print.rs");

    // Basic with args
    println!("{} is a {} boy", "Ruban", "good");

    // Positional arguments
    println!("{0} is a {1} boy and {0} likes {2}", "Ruban", "good", "cricket");

    // Named arguments
    println!("{name} is a {character} boy", name = "Ruban", character = "good");
}
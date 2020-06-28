/*
 * 4 primitive/Scalar types
 *   - integers (unsigned/signed numbers)
 *   - floating point
 *   - boolean
 *   - characters
 * 
 * 2 Compound Types
 *   - Arrays
 *   - Tuples
 * 
 */

pub fn run() {
    println!("------------types.rs------------");

    let x = 32;           // default i32
    let y: i64 = 32323;
    let z = 64.5;         // default f64
    let a: f32 = 42.3;

    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    println!("a = {}", a);
}

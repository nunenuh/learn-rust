fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup; // Destructuring the tuple

    // Print the values of x, y, and z if needed
    println!("Value of x: {}", x);
    println!("Value of y: {}", y);
    println!("Value of z: {}", z);
}

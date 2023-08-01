fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    println!("Value of x: {:?}", x);

    let five_hundred = x.0;
    println!("Value of five_hundred: {}", five_hundred);

    let six_point_four = x.1;
    println!("Value of six_point_four: {}", six_point_four);

    let one = x.2;
    println!("Value of one: {}", one);
}

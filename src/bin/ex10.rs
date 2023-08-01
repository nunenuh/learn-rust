fn main() {
    let a = [0, 1, 2, 3, 4, 5];

    let first = a[0];
    println!("first: {}", first);

    let second = a[1];
    println!("second: {}", second);

    let b: [i32; 3] = [1, 2, 3];
    println!("b: {:?}", b);

    let c = ["Jan", "Feb", "Mar"];
    println!("c: {:?}", c);

    let d = [3; 5];
    println!("d: {:?}", d);
}

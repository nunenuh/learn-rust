fn main(){
    // let reference_to_nothing = dangle();
    let reference_to = no_dangle();

    println!("reference_to: {}", reference_to);

}

// fn dangle() -> &String {
//     let s = String::from("Hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("Hello");

    s
}
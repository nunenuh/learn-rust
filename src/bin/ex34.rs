#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    // -- snip --
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }   
    }
}

fn main(){
    let c: Coin = Coin::Quarter(UsState::Alabama);

    let v = value_in_cents(c);
    println!("The value of v: {}",v);
}

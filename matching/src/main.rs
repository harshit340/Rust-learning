
#[derive(Debug)]
enum Dimevalue {
    Alabama,
    Alaska,
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime(Dimevalue), 
    Quarter,
}

fn coin_value(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime(_) => 10,
        Coin::Quarter => 25,
    }
}
fn main(){
    let coins = [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime(Dimevalue::Alabama),
        Coin::Quarter,
    ];

    let newcoins = [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime(Dimevalue::Alabama),  // Instantiate with a Dimevalue
        Coin::Dime(Dimevalue::Alaska),   // Another instance with a different Dimevalue
        Coin::Quarter,
    ];
   
   for coin in &coins{
    let value = coin_value(coin);
    println!("Value of {:?}:{}", coin , value);
   }
   for coin in &newcoins{
    let value = coin_value(coin);
    println!("Value of {:?}:{}", coin , value);
   }
   
}

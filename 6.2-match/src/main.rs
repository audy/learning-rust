#[derive(Debug)]
enum UsState {
    Florida,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    for coin in [Coin::Nickel, Coin::Dime, Coin::Quarter(UsState::Florida)].iter() {
        println!("{}", value_in_cents(&coin));
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);

    let dice_roll = 9;

    let res = match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    };

    println!("{:?}", res);
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}

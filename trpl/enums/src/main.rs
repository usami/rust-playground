#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    NewYork,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn main() {
    println!("{}", value_in_cents(Coin::Quarter(UsState::NewYork)));
    println!("{}", value_in_cents(Coin::Penny));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    match six {
        None => {},
        Some(i) => println!("{}", i),
    }

    match none {
        None => {},
        Some(i) => println!("{}", i),
    }

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(1) => println!("one"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }
}

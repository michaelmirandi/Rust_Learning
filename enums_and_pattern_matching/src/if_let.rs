#[derive(Debug)] 

enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny(UsState),
    Nickel,
    Dime,
    Quarter(UsState)
}

pub fn third() {
    let config_max = Some(15u8);
    match config_max {
        Some(max) => println!("THe maximum is configured to be {}", max),
        _ => ()
    }

    // write it this way instead...
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        // need the debug statement at the top to print within this control flow.
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }

    println!("Count {}", count);
    let coin_two = Coin::Dime;
    println!("Coin value... {}", get_coin_value(coin_two));

    // if let is for only one specific case, but it is not exhaustive like the match control flow is
    // only when one pattern is needed to match...
}

fn get_coin_value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny(state) => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 25
    }
}

// things covered in this chapter :

/*
    Create custom types that can be one of a set of enumerated values
    Use std library's Option<T> to use the type system to prevent errors
    If enums values have data inside of them, you can use the match structure...
    Add custom types to your API so that the rust compiler makes certain the functions 
        get only values of the type each function expects
    Next chapter is about rust modules and how to organize them.
*/
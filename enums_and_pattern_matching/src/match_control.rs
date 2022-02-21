#[derive(Debug)] 

// match, enums, options, and some

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

/*
    Match is a control flow based on patterns
    patterns can be anything, variable names, wildcards, values, etc.
    Compiler will confirm that all cases are handled...
    Its like a switch case statement, but cleaner...
    You match workflows and functions to specific patterns...
*/

pub fn second() {
    println!("Second Chapter");
    println!("{}", value_in_cents(Coin::Penny(UsState::Alabama)));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?} {:?}", five, six, none);
    game_impl(8);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny(state) => {
            println!("State Penny from {:?}!", state);
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// takes in an option.. if it is a number then it adds 1, if it isn't then it returns none.
fn plus_one(x: Option<i32>) -> Option<i32> {
    // if you don't handle all the cases of the data type you're matching on, it won't compile
    // matches in rust are exhaustive
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn game_impl(dice_role: i8) {
    match dice_role {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // can use other to handle everything else.. it will bind to the value
        //other => move_player(other)
        // using _ will not use the value at all (memory efficient)
        // empty tuple is like a null response... won't do anything.
        _ => ()
    }
}

fn add_fancy_hat() {
    println!("Add Fancy Hat Here..");
}

fn reroll() {
    println!("Reroll here...")
}

fn remove_fancy_hat() {
    println!("Remove Fancy Hat Here..");
}

fn move_player(num_spaces: i8) {
    println!("Move {} spaces", num_spaces);
}
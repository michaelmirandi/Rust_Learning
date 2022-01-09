// bring IO library into scope, comes from the standard library
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // process a guess
    // takes range expression as the argument: start..end or 1..=100
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret_number);
    println!("Guess the number!");
    
    // use let to declare variables... mut will allow you to make changes to that program
    // variables are immutable by default
    // function that returns a new instance of the string type
    // :: means that it is an associated function of the string type
    // associated function is implemented on a type

    loop {
        let mut guess = String::new();
        println!("Please input your guess.");

        // library::method
        // std::io::stdin() if you didn't want to import the whole thing...
        io::stdin()
            // takes whatever the user types into the standard input and appends it to the string parameter
            // & indicates the argument is a reference, not a copy...
            // no need to copy that data into memory multiple times
            // references are also immutable by default...
            .read_line(&mut guess)
            // rust has a number of types named results in its standard library
            // they are enums: results are enums
            // results encode error handling information
            // io::result has an expect method that you can call
            // if you leave out expect, you will get a compiler warning
            // compiler warnings are very helpful, just like in typescript
            .expect("Failed to read line");

        // shadow the old value of guess with a new one
        // trim off all whitespace, then parse from string to number
        // parse is the equivalent of parseInt in JS, but it is a method off a string in rust, not just a standalone function like JS
        // : annotates the variable type... just like TS
        // u32 is unsigned 32 integer
        // parse method also returns a result type... so you can use the expect method off of it...
        // instead of using expect, you can use a function after to handle the enum routing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // match all errors using the _ param
            Err(_) => continue,
        };

        // string formatting
        println!("You guessed: {}", guess);

        // now we need to create a secret number...
        // use the rand crate from rust team
        // crate is a collection of rust source code files. 
        // cargo handles this for you
        // cargo.lock allows you to have reproducable builds
        // cargo update will ignore the lock file
        // cargo doc --open will build documentation provided by all your dependences locally and open in your browser
        // super powerful
        // the Ordering type is also an enum
        // ordering has three diff categories, less, greater, and equal
        // function where you can handle outputs off of a specific object
        // use match to decide what you're going to do next based on which variant of ordering is returned from the cmp method
        // match expression is made up of arms

        // arms consist of patterns
        // let you express a variety of situations your code might encounter and make sure that you handle them all
        // code wont compile because cmp needs to compare two of the same type...
        // mismatched types on function inputs and outputs will cause a compiler error
        // rust has a very strong static type system
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }


}

/*
by default variables are immutable
- takes advantage of the saftey and easy concurrency that rust offers

difference between variables and constants
- can't use mut with constants
- always have to annotate type for the constants

*/

fn main() {
    // ---------- Variables and Mutability
    // all uppercases and underscores for constants
    // helps keep the maintenance of the program all in one place...
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // add the mut keyword to make it mutable...
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // shadowing
    // first variable is shadowed by the second
    // allows you to overwrite mutable values...
    // shadowing spares you from having to come up with different names if you want to transform the type of a variable...
    let y = 5;

    let y = y + 1;

    // --------- Data Types...
}

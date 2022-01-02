fn main() {
    println!("Hello World");
    println!("{}", plus_one(100_000.1));
}

/*
Functions can have parameters.
Concrete parameters are arguments

Function bodies can contain statements and expressions
Rust is an expression based language
Statements are instructions that perform some action but do not return a value
Expressions evaluate to a resulting value

STATEMENTS DO NOT RETURN VALUES... EXPRESSIONS DO

STATEMENTS AND EXPRESSIONS....
PARAMETERS, RETURN VALUES, and SCOPE...
*/

fn another_function(x: i32, another_param: char) {
    // macro
    // this is a statement
    // cannot assign a let statement to another variable...
    let y = 5;

    // let x = (let y = 5);
    // will not return 
    // expressions can be apart of statements
    // where 10 + 5 is an expression...

    // expressions and statements
    // functions and macros are both expressions
    let ya_digg = 10 + 15;
    // creating scopes...
    // block {} gets evaluated to 100_001
    // expressions do not include ending semicolons
    // adding a semicolon will not return any value with it...
    let first_scope = {
        let y = 100_000;
        y + 1
    };
    println!("Another function! {} {}", x, another_param);
}

// functions with return values 
fn first_return_function_value() -> f64 {
    100_000.001
}

fn plus_one(x: f64) -> f64 {
    x + 1.0
}
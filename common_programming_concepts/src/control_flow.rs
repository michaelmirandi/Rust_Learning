fn main() {
    println!("Hello Control FLow");
    // control the flow using if statements

    let number = 3;

    if number < 3 {
        println!("Number is less than three");
    } else {
        println!("Number is greater than or equal to 3");
    }

    // condition for if statements in the code must be bool
    // does not convert like JS or PY..
    if number == 3 {
        println!("Number is 3");
    } else if number != 3 {
        println!("Number is not three.");
    }

    // using if in a let statement...
    let condition = true;
    // types must be equal in both cases of the condition!
    let eval = if condition { "condition was met" } else { "condition was not met..." };
    println!("The eval is {}", eval);

    // repetition with loops
    // three kinds: loop, while, and for
    // loop tells rust to execute a block over and over until you explicitly tell it to stop...
    // break and continue just like in python...
    // add labels to loops for organization...


    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // returning values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
    
        if counter == 10 {
            // returns 20
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // conditional loops with while
    // while a condition is true, loop through...
    let mut number = 3;
    while number != 0 {
        println!("{}", number);

        number -= 1;
    }

    println!("Lift Offff");

    // looping through a collection with for
    let a = [10, 20, 30, 40, 50];

    // saftey and conciseness
    // most used loop in Rust
    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("Lift Offff");

}

/*
Chapter recap...
Variables, scalar and compound data types, functions, comments, if exressions, and loops

Practice Programs
- convert the temperatures between farenheight and celsius
- generate the nth fibonacci number
- print the lyrics to the Christmas carol "The twelve days of christmas", taking advantage of the repetition in the son


- next chapter... something that isn't commonly out there outside of Rust: ownership...
*/
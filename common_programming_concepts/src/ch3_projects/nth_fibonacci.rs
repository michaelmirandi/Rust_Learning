use std::io;

fn main() {
    // 1, 1, 2, 3, 5, 8, 13, 21, ...
    // sequence shows up in nature all the time
    let user_input = get_user_input();
    let mut counter = 0;
    let mut curr_value = 1;
    let mut last_value = 1;
    let mut temp_value: u32;

    let nth_fib = loop {
        if counter == user_input {
            break curr_value;
        }
        if counter != 0 {
            temp_value = curr_value;
            curr_value = curr_value + last_value;
            last_value = temp_value;
        }

        counter += 1;
    };

    println!("At index {}, the fibonacci sequence value is {}", user_input, nth_fib);
}

fn get_user_input() -> u32 {
    println!("Please which index of the fibonacci sequence you want to return");
    // declare as a string first because it is a string as input... then convert to integer...
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: u32 = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Problem reading in your index..."),
    };

    return index;
}
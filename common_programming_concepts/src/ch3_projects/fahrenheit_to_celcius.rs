use std::io;

fn main() {
    // get user input

    let user_input = get_user_input();
    println!("Fahrenheit is {}.", user_input);
    println!("Celcius is {}.", (user_input - 32.0) * (5.0/9.0));

}

fn get_user_input() -> f64 {
    println!("Please enter the number of degrees in fahrenheit you want to convert to celcius...");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: f64 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Problem reading in your fahrenheit..."),
    };

    return guess;
}
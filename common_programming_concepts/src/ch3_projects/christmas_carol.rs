use std::fmt;

fn main() {
    // loop through 12 times...
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", 
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelveth"];

    let mut gifts = [
        "A partridge in a pear tree",
        "Two turtle-doves",
        "Three French hens",
        "Four calling birds",
        "Five Golden Rings", 
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping", 
        "Twelve drummers drumming"
    ];

    gifts.reverse();

    for (index, day) in days.iter().enumerate() {
        let base = format!("On the {} day of Christmas", day);
        let additional = "My true love sent to me";
        println!("{}", base);
        println!("{}", additional);
        let end_index = gifts.len() - index - 1;
        for gift in gifts[end_index..gifts.len()].iter() {
            if index != 0 && gift == &"A partridge in a pear tree" {
                println!("And {}", gift);
            } else {
                println!("{}", gift);
            } 
        }
        println!("");

    }
}
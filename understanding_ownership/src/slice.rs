fn main() {
    // let mut s = String::from("Hello world");

    // let word = first_word(&s);

    // s.clear();

    // if you try an manupulate s using the word index, you will get an error down here...

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    // string slice is a reference to a part of a string...

    
    let test_str = String::from("Hello");

    let slice_1 = &test_str[..3];
    let slice_2 = &test_str[1..];
    let slice_3 = &test_str[1..3];
    let slice_4 = &test_str[1..test_str.len()];

    let my_string = String::from("hello world");

    let word = first_word_good(&my_string[0..6]);
    let word = first_word_good(&my_string[..]);
    let word = first_word_good(&my_string);

    let my_string_literal = "hello";

    // let word = first_word_good(&my_string_literal[0..6]);
    // let word = first_word_good(&my_string_literal[..]);

    let word = first_word_good(my_string_literal);
    let second_word = second_word(my_string_literal);

    println!("First... {}", word);
    println!("Second... {}", second_word);

    let a = [1, 2, 3, 4, 5];

    other_slices(&a);
}

// slice is a data type that does not have ownershp
// reference to a sequence of elements in a collection (lists, arrays, vectors, etc...)

// returns the index of the first space...
// .iter() returns each element in a collection via loop
// .enumerate() for indexes...
fn first_word(s: String) -> usize {
    let bytes = s.as_bytes();
    // use a pattern to destructure the tuple that we receive from enumerate
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn other_slices(a: &[u128]) -> () {
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    println!("Success!");
}

// remember to pass by reference... you want to return a piece of the original string...
// do not make extra memory to do this...
fn first_word_good(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    // range indicator by itself is the entire thing...
    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }

    "no second word..."
}

// str is more flexible than String
// takes advantage of deref coercions
fn experienced_function(s: &str) -> &str {
    // perform operations on both input and output
    &s
}


// rusts fix for the index storage bug is string slices...

/*

Ownership, borrowing, and slices ensure memory safely in rust programs at compile time
You don't have to write and debug extra code to worry about memory management...
The owner of the data automatically cleans up the data when the owner goes out of scope
Ch 5 is about structs and ownership/borrowing/references are used and talked about through the rest of the book
*/

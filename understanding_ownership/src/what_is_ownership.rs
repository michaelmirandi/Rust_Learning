fn main() {
    let s = String::from("Hello");

    // once the function takes in a value that is stored on the heap... and not the stack
    // the item is lost forever and can never be used again...
    takes_ownership(s);
    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{} {}", s1, s3);

    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1);
    println!("Length is {} for string {}", len, s2);

    {
        // s remains valid until it's out of scope...
        let s = "Hello";
    }

    // string type
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    // memory and allocation
    // stacks, heaps, and garbage collection
    // drop function to return the memory needed...

    // uses a stack, makes a copy of x, and assigns to y
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    // let s2 = s1; this will make s1 invalidated and unusable... no deep copys, just reallocation
    let s2 = s1.clone();


    // ownership and functions
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
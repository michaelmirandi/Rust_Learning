fn main() {
    let mut s1 = String::from("hello");
    // use & to take make reference to the object instead of transferring ownership
    let len = calculate_length(&s1);
    change(&mut s1);
    println!("The length of '{}' is {}.", s1, len);

    // remember.. you can only have one mutable reference to the data at a specific time
    // rust prevents data races at compile time
    // similar to a race condiiton
    // Two or more pointers access the same data at the same time
    // at least one of the pointers is being used to write to the data
    // there's no mechanism being used to synchronize access to the data

    // below only works if you bring have one mutable s1 object at a time using these different scopes
    {
        let r1 = &mut s1;
    }
    let r2 = &mut s1;
    // println!("{}, {}", r1, r2);

    // same occurs for immutable references
    // multiple immutable references to a pointer is ok, but once you add a mutable in.. things go wrong
    // multiple immutable refs are cool because they only view data. not change it
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    // once a reference is used, it's gone and value is returned to original variable owner
    // freeing up the variable to be called as immutable down below
    // compiler used non lexical lifetimes (NLL) to identify these scopes

    let r3 = &mut s;
    println!("{}", r3);

    // dangling references
    // easy to create pointers that reference a location in memory that you're giving to someone else
    // memory management is hard with pointers in languages like c++
    // rust compiler guarantees that references will never be dangling
    let ref_to_nothing = dangle_good();

    // recap
    // references and borrowing
    // preventing dangling
    // Mutable refs
}

// refers to lifetimes which we haven't thought about yet
// fn dangle_bad() -> &String {
//     let s = String::from("hello");

//     &s
// }

// & is by reference...

fn dangle_good() -> String {
    let s = String::from("hello");

    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// function that doesn't return a type makes use of the unit type...
// does not compile if you dont pass it as mut because you try to make a change to a reference...
fn change(s: &mut String) -> () {
    s.push_str(", world");
}

// At any given time, you can have either one mutable reference or any number of immutable references
// References must always be valid

// slices are a different kind of reference

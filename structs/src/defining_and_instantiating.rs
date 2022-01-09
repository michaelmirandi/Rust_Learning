struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

pub fn one_function() {
    /*
        Structs are like tuples, their contents can have different data types
        You name each peice of data
    */
    println!("hello world");

    // either all fields are mutable or not.. no customization here...


    let mut me = User {
        email: String::from("mmirandi@hornets.com"),
        username: String::from("mmirandi"),
        active: true,
        sign_in_count: 1
    };

    me.email = String::from("mmirandi@gatech.edu");

    build_user(String::from("dmichael@gmail.com"), String::from("Darren Michael"));

    // similar to JS, you can unpack a struct using the .. operator
    // in JS its ...
    // unlike JS, unpackiing will not overrite something if you declare it within the struct
    let mut new_user = User {
        email: String::from("anotheremail@yadigg.edu"),
        username: String::from("anotherusername"),
        ..me
    };

    println!("First UN: {}, Second UN: {}", me.username, new_user.username);

    // in this case we can no longer use me... we have to use new_user because all the data from me
    // along with pointers to memory have been given to new_user and it is out of scope
    // ONLY for the types that pull off the heap...
    // so if you gave new_user a new username, then you could still access me

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // unit like structs with no fields
    struct AlwaysEquals;

    let subject = AlwaysEquals;

    /*
        Ownership of structs...
        Earlier you used String rather than &str because you want each struct to have its own place in memory
        Structs can also store references to something else..
        Need to understand lifetimes to do this (CH10)
        Lifetimes ensure that the data referencced by a struct isvalid for as long as the struct is
    */
}

fn build_user(email: String, username: String) -> User {
    // if the names are 1:1 you can just put them in the struct
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
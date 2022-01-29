// can be used as ATTRIBUTES for structs
enum IpAddrKind {
    V4,
    V6
}

// struct IpAddr {
//     address: String,
//     kind: IpAddrKind
// }

// this way is much cleaner

// enum IpAddr {
//     V4(String),
//     V6(String)
// }

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String)
// }

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// you can match off of enums...
// you can also define methods off of enums.... like classes..
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        // method body here..
        println!("Call method off of enum just like struct");
    }
}

pub fn first() {
    // you call enums the same way you do code in different files
    // when you have a set of outcomes that you can enumerate...
    

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    // let home = IpAddr {
    //     address: String::from("127.0.0.1"),
    //     kind: IpAddrKind::V4
    // };

    // let loopback = IpAddr {
    //     address: String::from("::1"),
    //     kind: IpAddrKind::V6
    // };
    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));

    // using IpAddresses is so common that the standard library has something
    // to interact with them...

    // you can put any kind of data inside an enum... even another enum
    let m = Message::Write(String::from("Hello"));
    m.call();

    // option is another type of enum
    // has a lot of advantages over null values
    // defined by the standard library
    // encodes very common scenario in which a value could be something
    //  or it could be nothing
    // prevents common bugs
    // Rust does NOT have the null feature
    // Tony Hoare is the inventor of null
    // if you try and use null values as not null, you get an error
    // Rust has an enum called an OPTION which you can encode null like values
    // you don't need to bring it into scope ... it's already here.
    // type <> parameter just like TS

    // use Some() function to figure out the type and store as Option
    // shorthand
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
    let absent_string: Option<String> = None;

    // the below will NOT work
    // both need to be options
    let x: u8 = 5;
    let y: Option<i8> = Some(5);

    // cannot add option type to raw option...

    // let sum = x + y;

    let z: Option<i8> = Some(10);

    // this won't work either.. compiler will make sure
    //  that you handle the case of None before you call
    // have to convert option to its type before you can perform the 
    //  operations on T
    // every type except option types 
    // use documentation to figure out how to add types...println!
    // compiler will panic if you try to unwrap None...
    // USE MAP TO MAP NONE CASES...
    println!("{}", y.unwrap() + z.unwrap());

    // match expression is a control flow construct that
    // maps outcomes of enums to a function.
}

fn route(ip_kind: IpAddrKind) -> () {
    
}
// functions are declared using fn
// main function is the first code that run sin a rust file...
fn main() {
    // statements end with a semicolon
    println!("Hello World");
}

// compile the program with rustc <program_name>
// run it by calling the executable (binary executable)
// .pdb contains all the debugging information
/*
    Rust is an ahead of time compiled language... meaning you can compile a program and then give the executable to someone else...
    Easy handoff... users don't need to have rust on their machine...
    cargo allows you to write real-world rust applications...
*/

/*
    Cargo...
    Build system and package manager for rust
    Rustaceans... ahaha
    Handles building your code, downloading dependencies, and building those libraries
    Rest of the book assumes that you're using cargo as well...
    cargo new hello_cargo -- to create a new project folder...
*/
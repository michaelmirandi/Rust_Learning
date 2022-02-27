/*
    Crate is a binary or library
    Crate root is the source file that Rust compiler starts from and makes up the root module of your crate
    Package is one or more crates that provide a set of functionality
    Cargo.toml describes how to build the crates
    Package can contain at most one library crate
    - can contain many binary crates, but only one library crate
    if lib.rs is provided, then you will have a library crate
    main.rs is for binary crates
    for multiple binaries, put them in the src/bin folder
*/
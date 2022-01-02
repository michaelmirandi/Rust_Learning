fn main() {
    /*
    Two main subsets: scalar and compound
    Rust is statically typed, meaning that you have to know all of the types of variables at compile time
    Scalar is a single value: integer, floating point, numbers, booleans, and characters
    Unsigned integers... are always positive, signed integers are not

    */

    // default type is u32
    // be ware of integer overflow...
    let x = 1_000;
    println!("Hello World, {}", x);

    // f32 and f64 for floats
    // default is f64

    let new_float = 1.0; // f64
    let another_float: f32 = 1.0; // f32

    // + - / * %

    // true & false lowercase

    let first_bool = true;

    // char type is 4 bytes in size...
    let c = 'M';

    // compound types can group multiple values into one type. 

    // Tuple Type and Arrays
    /*
    Tuples have a fixed length and cannot grow or shrink in size...
    */

    let tup: (i32, f64, f32) = (1_000_000, 1.0, 1.2);
    // tuple unpacking, called destructuring...
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);
    println!("{}", tup.0);
    // tuple with no values is called a unit value ()


    // Array Types...
    // every element of an array must have the same type, unlike a tuple...
    // use arrays when you want data allocated on a stack rather than a heap
    // they DO have fixed length... just like tuples... vectors are more useful
    // storing months of the year in an array because you won't have to change the months ever....

    let first_arr: [u32; 3] = [1, 2, 3];
    // value; length...
    let second_arr = [3; 5];

    // Array is a single chunk of memory of a known, fixed size that can be allocated on the stack
    let first = first_arr[0];

    // Rust panics...
    // this is why rust is unique...
    // in many low-level languages, this type of check is not done at runtime.
    // Rust protects you by immediately exiting instead of allowing the memory access and continuing...
    // Chapter 9 contains most of the error handling logic
}
mod defining_an_enum;
mod match_control;
mod if_let;

fn main() {
    // overview
    /*
        ENUMS allow you to define a type by enumerating it's possible vairants
        OPTION is a useful enum, it expresses that a value can be either something or nothing \
             (number | undefined)
        Look at how pattern matching in the MATCH expression makes it easy to run \
            different code for different values of an enum
        IF LET constructor enabling you to handle enums
        Enums can be user defined
    */
    defining_an_enum::first();
    match_control::second();
    if_let::third();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// everything in impl block will apply strictly to rectangles
// self is alias...
// imlement methods off of structs
impl Rectangle {
    // these functions can plug into any api to be used to do useful things
    // &self allows us to not take ownership... just pure functions and code
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn three_function() {
    let rect1 = Rectangle {
        width: 30, 
        height: 50
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45
    };

    println!(
        "Here is the height using the method syntax, {}",
        rect1.area()
    );


    if rect1.width() {
        println!(
            "Rectangle has nonzero width; its {}",
            rect1.width
        ); 
    }


    println!(
        "Can rect1 hold rect2? {}",
        rect1.can_hold(&rect2)
    );

    println!(
        "Can rect1 hold rect3? {}",
        rect1.can_hold(&rect3)
    );

    // all by reference... make sure you continue to think about ownership


    // rust has automatic referenccing and dereferencing
    // methods have a clear receiver... the type of self
    // makes ownership ergonomic in practice
    // the receiver is of the type self... in this case self calls off of rect1
    rect1.width();
    (&rect1).width();
}
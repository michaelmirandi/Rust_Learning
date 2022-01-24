// opt in to these debug changes...
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

pub fn two_function() {
    // write a program that calculates the area of a rectangle
    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels",
    //     area(width1, height1)
    // )

    // let rect1 = (30, 50);

    // println!(
    //     "The area of the rectangle is {} square pixels",
    //     area(rect1)
    // );
    let scale = 5;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };

    println!(
        "rect is {:?}",
        rect
    );

    dbg!(&rect);
}

fn area(rect: Rectangle) -> u32 {
    rect.height * rect.width
}

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// dbg!() is very helpful to figure out what your code is doing...
// next time learn how to make an area method defined on the rectangle type...
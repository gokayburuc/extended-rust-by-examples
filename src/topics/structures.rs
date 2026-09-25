#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
struct Unit;
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

#[derive(Debug)]
struct Pair(i32, f32);

#[allow(dead_code)]
fn execute_sturcture() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print Debug Struct
    println!("{:?}", peter);

    let first_point: Point = Point { x: 5.2, y: 0.4 };
    let second_point = Point { x: 10.3, y: 0.2 };

    println!("Point Coordinates:  ({}, {})", first_point.x, first_point.y);

    // TODO: add ..second_point syntax explanation
    let bottom_point = Point {
        x: 10.3,
        ..second_point
    };

    println!("Second point: ({}, {})", bottom_point.x, bottom_point.y);

    // Destructure the point using a let binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = first_point;

    let _rectangle: Rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_point,
    };

    let _unit = Unit;
    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

// define a struct

struct Point {
    x: f64,
    y: f64,
}

// Define methods here

impl Point {
    fn origin() -> Point {
        // create a point with 0.0 , 0.0 axis
        Point { x: 0.0, y: 0.0 }
    }

    // associated function, takes two arguments
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

// Rectangle consist of two points
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // area calculation function for Rectangle struct
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // width * height
        ((x1 - x2) * (y1 - y2)).abs()
    }

    // perimeter of an rectangle
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // perimeter  2 (weight + height )
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }
}

pub fn execute_method() {
    let mut rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    rectangle.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}

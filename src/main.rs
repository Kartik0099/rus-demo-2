#[derive(Debug)]
struct Rectangle {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Area for Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }
}

#[derive(Debug)]
struct Square {
    x: usize,
    y: usize,
    size: usize,
}

impl Area for Square {
    fn area(&self) -> usize {
        self.size * self.size
    }
}

trait Area {
    fn area(&self) -> usize;
}

#[derive(Debug)]
enum Shape {
    Rectangle,
    Square,
}
fn main() {}

#[allow(dead_code)]
fn new(x: usize, y: usize, width: usize, height: usize, shape: Shape) -> Box<dyn Area> {
    match shape {
        Shape::Rectangle => Box::new(Rectangle {
            x,
            y,
            width,
            height,
        }),
        Shape::Square => Box::new(Square { x, y, size: width }),
    }
}

// fn add_to(a:)

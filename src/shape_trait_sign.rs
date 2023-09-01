use std::ops;

#[allow(dead_code)]
enum Shape {
    Rectangle,
    Square,
}

#[allow(unused_variables)]
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

impl ops::Add<&Rectangle> for &Rectangle {
    type Output = Rectangle;

    fn add(self, rhs: &Rectangle) -> Self::Output {
        return Rectangle {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        };
    }
}

impl ops::Add<&Square> for &Rectangle {
    type Output = Rectangle;

    fn add(self, rhs: &Square) -> Self::Output {
        return Rectangle {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            width: self.width + rhs.size,
            height: self.height + rhs.size,
        };
    }
}

impl ops::Add<&Square> for &Square {
    type Output = Square;

    fn add(self, rhs: &Square) -> Self::Output {
        return Square {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            size: self.size + rhs.size,
        };
    }
}

impl ops::Add<&Rectangle> for &Square {
    type Output = Rectangle;

    fn add(self, rhs: &Rectangle) -> Self::Output {
        return Rectangle {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            width: self.size + rhs.width,
            height: rhs.height + self.size,
        };
    }
}

fn main() {
    let r1 = new_rect(0, 0, 10, 10);

    let r2 = new_rect(10, 10, 15, 15);

    let r3 = &r1 + &r2;

    println!("r1 : {:?}  + r2 {:?} = r3 {:?} ", r1, r2, r3);
}

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

fn new_rect(x: usize, y: usize, width: usize, height: usize) -> Rectangle {
    return Rectangle {
        x,
        y,
        width,
        height,
    };
}

#[allow(dead_code)]
fn new_square(x: usize, y: usize, size: usize) -> Square {
    return Square { x, y, size };
}

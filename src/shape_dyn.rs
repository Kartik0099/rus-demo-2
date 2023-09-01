use std::ops;
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

fn new(x: usize, y: usize, width: usize, height: usize, is_rect: bool) -> Shape {
    if is_rect {
        return Shape::Rectangle(Rectangle {
            x,
            y,
            width,
            height,
        });
    } else {
        return Shape::Square(Square { x, y, size: width });
    }
}

impl ops::Add for &Shape {
    type Output = Shape;

    fn add(self, rhs: &Shape) -> Self::Output {
        match self {
            Shape::Rectangle(r1) => match rhs {
                Shape::Rectangle(r2) => return Shape::Rectangle(r1 + r2),
                Shape::Square(s1) => return Shape::Rectangle(s1 + r1),
            },
            Shape::Square(s1) => match rhs {
                Shape::Rectangle(r2) => return Shape::Rectangle(s1 + r2),
                Shape::Square(s2) => return Shape::Square(s1 + s2),
            },
        }
    }
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

impl ops::Add<&Rectangle> for &Square {
    type Output = Rectangle;
    fn add(self, rhs: &Rectangle) -> Self::Output {
        return Rectangle {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            height: self.size + rhs.height,
            width: self.size + rhs.width,
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

fn main() {
    let rect = new(0, 0, 10, 10, true);

    let rect2 = new(10, 10, 15, 15, true);

    let s1 = new(0, 0, 10, 0, false);

    let s2 = new(20, 20, 20, 0, false);

    let rect3 = &rect + &rect2;

    println!("Shape : {:?} ", rect3);

    let s3 = &s1 + &s2;

    println!("Shape : {:?} ", s3);

    // let rect = new(0, 0, 10, 10, true);

    let r4 = &rect + &s1;

    println!("Shape : {:?} ", r4);

    let s4 = &s1 + &rect3;

    println!("Shape : {:?} ", s4);
}

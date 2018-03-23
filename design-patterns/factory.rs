// factory
//
// In Factory pattern, objects are created without exposing the creation logic
// to the client and refer to newly created object using a common interface.

/// Equivalent to the interface in java
/// Traits are more like scala's traits than java's interfaces,
/// due to the fact that they can include implementations, and not just
/// abstract interfaces, but here we are not using that facility.
/// Renamed to ShapeLike to not collide with the concrete enum Shape below
pub trait ShapeLike {
    fn draw(&self);
}

/// This is an additional artifact not present in java
/// Because of Rust's low level nature, and the importance of knowing
/// the size of types in advance, it's often far easier to return
/// an enum than an abstract trait.
/// The only downside is that if you add a new type, you need to add a new
/// item to the enum.
#[derive(Debug)]
pub enum Shape {
    Rectangle(Rectangle),
    Square(Square),
    Circle(Circle),
}

/// Our enum that can contain any of the shapes also behaves like a shape
impl ShapeLike for Shape {
    fn draw(&self) {
        match *self {
            Shape::Rectangle(ref rectangle) => rectangle.draw(),
            Shape::Square(ref square) => square.draw(),
            Shape::Circle(ref circle) => circle.draw(),
        }
    }
}

#[derive(Debug)] //Derive debug should be on nearly every struct and enum
pub struct Rectangle {}
// you would store the fields of your respective shapes inside these structs
#[derive(Debug)]
pub struct Square {}
#[derive(Debug)]
pub struct Circle {}



impl ShapeLike for Rectangle {
    fn draw(&self) {
        println!("inside Rectangle::draw()")
    }
}

impl ShapeLike for Square {
    fn draw(&self) {
        println!("inside Square::draw()")
    }
}

impl ShapeLike for Circle {
    fn draw(&self) {
        println!("inside Circle::draw()")
    }
}


/// Factories are not very idiomatic in rust, but leaving this as is
/// Consider using "builder" terminology instead of factory
pub struct ShapeFactory {}

impl ShapeFactory {
    pub fn get(shape: &str) -> Shape {
        match shape.as_ref() {
            "CIRCLE" => Shape::Circle(Circle {}),
            "SQUARE" => Shape::Square(Square {}),
            "RECTANGLE" => Shape::Rectangle(Rectangle {}),
            &_ => unimplemented!(),
        }
    }
}

fn main() {
    let circle = ShapeFactory::get("CIRCLE");
    let square = ShapeFactory::get("SQUARE");
    let rectangle = ShapeFactory::get("RECTANGLE");

    println!("circle: {:?}", circle);
    println!("square: {:?}", square);
    println!("rectangle: {:?}", rectangle);

    circle.draw();
    square.draw();
    rectangle.draw();

}
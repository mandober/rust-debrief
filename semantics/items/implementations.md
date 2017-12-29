Implementations

An implementation is an item that can implement a trait for a specific type.

Implementations are defined with the keyword impl.


struct Circle {
    radius: f64,
    center: Point,
}

impl Copy for Circle {}

impl Clone for Circle {
    fn clone(&self) -> Circle { *self }
}

impl Shape for Circle {
    fn draw(&self, s: Surface) { do_draw_circle(s, *self); }
    fn bounding_box(&self) -> BoundingBox {
        let r = self.radius;
        BoundingBox {
            x: self.center.x - r,
            y: self.center.y - r,
            width: 2.0 * r,
            height: 2.0 * r,
        }
    }
}
It is possible to define an implementation without referring to a trait. The methods in such an implementation can only be used as direct calls on the values of the type that the implementation targets. In such an implementation, the trait type and for after impl are omitted. Such implementations are limited to nominal types (enums, structs, unions, trait objects), and the implementation must appear in the same crate as the Self type:


struct Point {x: i32, y: i32}

impl Point {
    fn log(&self) {
        println!("Point is at ({}, {})", self.x, self.y);
    }
}

let my_point = Point {x: 10, y:11};
my_point.log();
When a trait is specified in an impl, all methods declared as part of the trait must be implemented, with matching types and type parameter counts.

An implementation can take type and lifetime parameters, which can be used in the rest of the implementation. Type parameters declared for an implementation must be used at least once in either the trait or the type of an implementation. Implementation parameters are written after the impl keyword.


impl<T> Seq<T> for Vec<T> {
    /* ... */
}
impl Seq<bool> for u32 {
    /* Treat the integer as a sequence of bits */
}
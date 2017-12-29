// impl methods and afn

// define a struct
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

// impl method for the struct:
impl Circle {
    // takes a ref to an instance (&self) of Circle (Self) type
    // param behaves as if it was: fn area(self: &self)
    fn area(&self) -> f64 {
        // in the fn body, refer to that param as bare `self`
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

trait HasArea {
    fn area(&self) -> f64;
    fn is_larger(&self, &Self) -> bool;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn is_larger(&self, other: &Self) -> bool {
        self.area() > other.area()
    }
}

// ! lifetimes

//#![allow(unused_variables,dead_code)]

struct Car {
    name: String,
}

impl Car {
    fn new(c: &str) -> Self {
        Car { name: c.to_string() }
    }
}

struct Driver<'a> {
    name: String,
    car: Option<&'a Car>,
}

impl<'a> Driver<'a> {
    fn new(n: &str) -> Self {
        Driver {
            name: n.to_string(),
            car: None,
        }
    }

    fn buy(&mut self, c: &'a Car) {
        self.car = Some(c);
    }

    fn swap(&mut self, with: &mut Driver<'a>) {
        if let (Some(sc), Some(wc)) = (self.car, with.car) {
            self.car = Some(wc);
            with.car = Some(sc);
        }
    }

}


fn main() {
    let mustang = Car::new("Mustang");
    let colt = Car::new("Colt");

    let mut pike = Driver::new("Louis Pikett");
    let mut sena = Driver::new("Anton Sena");

    pike.buy(&mustang);
    sena.buy(&colt);

    println!("{} is driving a {}!", pike.name, pike.car.unwrap().name);
    println!("{} is driving a {}!", sena.name, sena.car.unwrap().name);

    pike.swap(&mut sena);

    println!("Now {} is driving a {}!", pike.name, pike.car.unwrap().name);
    println!("Now {} is driving a {}!", sena.name, sena.car.unwrap().name);
}

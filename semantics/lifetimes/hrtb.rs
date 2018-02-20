// Higher-Ranked Trait Bounds
fn main() {
    //  inn: &for<'a> fn(&'a (Animal + 'a))
    let inn = &(jump as for<'a> fn(&'a for<'b> Animal));
    jump(inn);
} // main

trait Animal {
  fn run(&self);
}

impl<'a> Animal for &'a for<'b> Animal
 where for<'b> Animal: Animal {
  fn run(self: &&'a for<'b> Animal) {
    println!("30-aught-six")
  }
}

fn jump(four: &for<'a> Animal) {
    <&for<'a> Animal as Animal>::run(&{
        ((&four).run(), four.run());
        four
    })
}

impl Animal for for<'a> fn(&'a for<'b> Animal) {
    fn run(&self) {
        println!("oooh")
    }
}

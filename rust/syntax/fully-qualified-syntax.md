# Fully Qualified Syntax (FQS)

- Two ways to call a method:
  1. Method call with dot operator: `receiver.method(arg1, ...)`
  2. FQS syntax: `<Type as Trait>::function(receiver, arg1, ...);`
- Former enjoys all benefits of auto adjusting (ref/deref) the receiver to match the expected form in method's signature; the latter does not.
- FQS syntax has several forms:
  - calling the method impl directly on the type:   
      `Type::method(receiver, arg1, ...)`
  - calling the ambiguous method in the trait implementation:   
      `<Trait>::function(receiver, arg1, ...);`
  - calling the ambiguous method in specific trait implementation:   
      `<Type as Trait>::function(receiver, arg1, ...);`
  - For associated functions, there's no receiver (no `self`):    
      `<Type as Trait>::function(arg1, ...);`




- dot operator defaults to calling the method impl directly on the type, if method's name is ambiguous.



Previously known as Universal Function Call Syntax (UFCS)
`<Type as Trait>::function(receiver, arg1, ...);`

For associated functions, there's no receiver (no `self`):
`<Type as Trait>::function(arg1, ...);`

When a type has access to more than one eponymous method (e.g. it has one method impl on the type itself, and others obtained by impl traits), there's ambiguity about which method to call when using method call with dot operator.

The method directly implemented on the type is the default one. To call it, we can use the dot operator: `instance.method()` or FQS: `Type::method(instance)`. 

However, to call the particular trait's method, we must use FQS:   `Trait::method(instance)`. In fact full syntax is:   
`<Type as Trait>::method(instance)`.

A method call using a dot operator, `instance.method()`, will auto de/ref the operand as many times as needed to match the method's signature. So even if a method expects a reference to an instance, `&self`, we can still call it by writing `instance.method()` and let it be auto adjusted into the expected form `(&instance).method()`. With FQS this doesn't happened and we must pass the argument in its expected form.


```rust
struct Person { name: String };

impl Person {
  fn say(&self) {
    println!("I'm a {}", self.name);
  }
}

```



A trait can have a method with the same name as another trait's method, and a type might have a directly implemented method with that same name. When that type impl both of those traits, there will be three methods with the same name.


```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("shahbaz");
    }
}

impl Human {
    // method directly implemented on the type is the default one
    fn fly(&self) {
        println!("waving arms furiously");
    }
}

fn main() {
  let person = Human;
  // calling the ambigously named method defaults to type's own method
  person.fly(); // "waving arms furiously"
  // with dot syntax we don't have to give the precise expected form - the 
  // instance will be auto de/ref to match the signature. Even though the method
  // expects a ref (&self), we can just write `person.fly()` and let it be auto
  // adjusted to expected `(&person).fly()`.
  //
  // We can also use FQS:
  Human::fly(&person);


  // But to call the traits' method, we MUST use FQS
  Pilot::fly(&person);
  Wizard::fly(&person);
}
```

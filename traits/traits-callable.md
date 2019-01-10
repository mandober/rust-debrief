# Function related traits 

`FnOnce`,`FnMut`,`Fn` 
traits are implemented by types that can be invoked like fn.

These correspond to the 3 kinds of methods that can be invoked on an instance: 
- call-by-value: `FnOnce` takes `self`
- call-by-reference: `Fn` takes `&self`
- call-by-mutable-reference: `FnMut` takes `&mut self`
The most common use of these traits is to act as bounds to higher-level fns
that take functions or closures as arguments.

# FnOnce
Trait `std::ops::FnOnce`
version of call operator that takes a by-value receiver, *call-by-value*

```rust
// std::ops::FnOnce
#[lang = "fn_once"]
pub trait FnOnce<Args> {
    type Output;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}

// taking FnOnce as a param:
fn consume<F>(func: F)
    where F: FnOnce() -> String {
    // func consumes its captured var, so it cannot be run more than once
    println!("Consumed: {}", func());
    // attempt to invoke func again will throw "use of moved value" error
}
let x = String::from("x");
let consume_and_return_x = move || x;
consume(consume_and_return_x);
// `consume_and_return_x` can no longer be invoked at this point
```



# Taking a Fn as a parameter:
```rust
fn call_with_one<F>(func: F) -> usize
    where F: Fn(usize) -> usize
{
    func(1)
}

let double = |x| x * 2;
assert_eq!(call_with_one(double), 2);
```

# Taking a FnMut as a parameter:
```rust
fn do_twice<F>(mut func: F)
    where F: FnMut()
{
    func();
    func();
}

let mut x: usize = 1;
{
    let add_two_to_x = || x += 2;
    do_twice(add_two_to_x);
}

assert_eq!(x, 5);
```
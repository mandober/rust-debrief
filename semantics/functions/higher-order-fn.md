## Higher-order functions

Higher-Order function (HOF) take other functions as paramter(s), they can also return a function.


```rust
// Take function f(x), return function f(f(x))
fn twice<A, F>(function: F) -> Box<Fn(A) -> A> 
    where F: 'static + Fn(A) -> A
{
    Box::new(move |a| function(function(a)))
}

// Return x + 3
fn f(x: i32) -> i32 {
    x + 3
}

fn main() {
    let g = twice(f);
    println!("{}", g(7));
}
```

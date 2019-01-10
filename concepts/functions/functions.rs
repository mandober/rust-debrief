// ! Functions

// The final expression in a function is its return value.
// Use `return` for early return
// Must explicitly define params types and return type.
fn foo(x: T, y: U, z: V) -> T {
    ()
}


// ! Function Objects
// Several things can be used as function objects:
//  - Function pointers (a reference to a normal function)
//  - Closures

fn square(n: i32) -> i32 {
    n * n
}

fn apply_twice(f: &Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn main() {
    let x: fn(i32) -> i32 = square;
    let y = apply_twice(&x, 5);
    println!("{}", y); // 625
}

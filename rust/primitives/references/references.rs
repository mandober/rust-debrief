// References

let x = 12;
let ref_x = &x;
println!("{}", *ref_x); // 12



// * REFERENCES AND BORROWING
// We call having references as function parameters borrowing.

// ampersands are references, and they allow you to refer to some value without
// taking ownership of it
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
// function has a reference to an object as a parameter instead of taking ownership of the value:
fn calculate_length(s: &String) -> usize {
    s.len()
}

// modify something borrowed does not work - Just as variables are immutable
// by default, so are references. To modify a value use MUTABLE REFERENCES.
fn main() {
    let mut s = String::from("hello"); // FIRST, the var has to be mutable
    change(&mut s); // SECOND, create a mutable reference
}
// function has a mutable reference to an object as a parameter:
fn change(some_string: &mut String) { // THIRD, declare mutable ref param
    some_string.push_str(", world");  // FOURTH, mutate
}

// THERE CAN BE ONLY 1 MUTABLE REFERENCE TO A PARTICULAR DATA IN A PARTICULAR SCOPE
// CANNOT HAVE A MUTABLE REFERENCE WHILE WE HAVE AN IMMUTABLE ONE

// dangling refs are impossible in rust. compiler will complain:
//   "this function's return type contains a borrowed value,
//   but there is no value for it to be borrowed from"

/**
The Rules of References:
1.At any given time, you can have EITHER
     One mutable reference (write) OR
     Any number of immutable references (read)
2.References must always be valid (no dangling refs)

So, there can be only one writer OR multiple readers.
*/

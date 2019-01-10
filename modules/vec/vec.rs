// ! VECTOR
Vec<T>
/**
Module vec: https://doc.rust-lang.org/stable/std/vec/

Vec<T>
Struct std::vec::Vec
A contiguous growable array type with heap-allocated contents, written Vec<T>.

- Vectors have O(1) indexing, amortized O(1) push (to the end) and O(1) pop (from the end).
- A standard library type: you don't need to import anything.
- A Vec is a heap-allocated growable array (cf. Java's ArrayList, C++'s std::vector, etc.)
- <T> denotes a generic type (The type of a Vec of i32s is Vec<i32>)
- Vectors can only store values of the same type.

Create Vecs with Vec::new() or the vec! macro.
Vec::new() is an example of namespacing.
new is a function defined for the Vec struct.

*/

// ! CREATING
// Vec::new()
// we can create new vec with explicit type annotation:
let mut v1:Vec<u8> = Vec::new();
v1.push(1);
v1.push(2);
// or without it, in which case Rust will infer the type when we add el to vector:
let mut v2 = Vec::new();
v2.push(255);
// it will infer i32 for integers by default.
// To force the specific type:
let mut v3 = Vec::new();
v3.push(255u8); // use suffix
// now the vec is u8 so if we try to push non-u8 int, we get an error
//v3.push(455);

// since vec are often vreated, there is a vec![] macro:
let nums = vec![30, 34, 43, 44, 66, 33];
// This will create a new Vec<i32>
// to create specific type with vec! macro:
let ints :Vec<u8> = vec![30, 34, 43, 44, 66, 33];

// to create vector containg the same value, use macro: vec![val; repeat]
let v3 = vec![0; 4];
let v4 = vec![0, 0, 0, 0];
// v3 and v4 are equal


// ! UPDATING
// To create a vector then add elements to it, we can use the push method:
let mut v5 = Vec::new();
v5.push(5);
v5.push(6);
v5.push(7);
// vec var needs to be mut, of course


// ! ACCESS

let v7 = vec![1, 2, 3];
let x = v7[2]; // 3
// Like arrays, vectors can be indexed with [].
// • You can't index a vector with an i32/i64/etc.
// • You must use a usize because usize is guaranteed to be the same size as a pointer.
// • Other integers can be cast to usize:
     let i: i8 = 2;
     let y = v7[i as usize];

// Read value with indexing syntax or the get method
let vv = vec![1, 2, 3, 4, 5];

// ? INDEXING -> reference
// The Vec type allows to access values by index, because it implements the `Index` trait.
// gives us a reference
// cause a panic when a non-existent element is referenced
let third: &i32 = &vv[2];

// ? GET METHOD -> Option<&T>
// gives us an Option<&T>
// when a non-existent element is referenced, it returns None, without panicking
let third: Option<&i32> = vv.get(2);

// we can’t have mutable and immutable references in the same scope.
// That rule applies here, where we hold an immutable reference to the first element in a vector and try to add an element to the end:
let mut v = vec![1, 2, 3, 4, 5];
let first = &v[0];
v.push(6);
// error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable

// Adding a new element onto the end of the vector might require allocating new memory and copying the old elements over to the new space


// ! Using an Enum to Store Multiple Types
// it can be inconvenient that vectors can only store values that are all the same type, but the variants of an enum are all defined under the same enum type, so when we need to store elements of a different type in a vector, we can define and use an enum.

// We can define an enum whose variants will hold the different value types, and then all of the enum variants will be considered the same type, that of the enum. Then we can create a vector that holds that enum and so, ultimately, holds different types:
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];



// * find max number in vec manually
fn main() {
    let v = vec![3, 6, 10, 8, 7];
    let mut maxy = v[0]; // or: &v[0]
    println!("{}", maxy);

    for n in v { // or: &v
        //println!("{}", n);
        if maxy < n {
            maxy = n;
        }
    }
    println!("The largest number is {}", maxy);
}


// * find max number in vec
let v = [1, 2, 3];
let m = v.iter().max().expect("panic!");
print!("max element is: {}", m);
assert_eq!(v.iter().max(), Some(&3));

let b: Vec<u32> = Vec::new();
assert_eq!(b.iter().max(), None);




// * generate fib. seq. of first 15 values
let mut v :Vec<u16> = vec![1; 15];
for i in 2..15 {
    v[i] = v[i - 2] + v[i - 1];
}
print!("{:?}, ", v);

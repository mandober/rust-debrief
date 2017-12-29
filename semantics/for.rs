// *for
// for is the most different from most C-like languages
// for loops use an iterator expression:
//   n..m creates an iterator from n to m (exclusive).
//   Some data structures can be used as iterators, like arrays and Vecs.

// Loops from 0 to 9.
for x in 0..10 {
    println!("{}", x);
}

let xs = [0, 1, 2, 3, 4];

// Loop through elements in a slice of `xs`
// (my note: seems x here doesn't need to be pre-declared)
for x in &xs {
    println!("{}", x);
}


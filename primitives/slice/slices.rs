// ! slice

// array
let seq: [i32; 3] = [1, 2, 3];
// vector
let seq: Vec<i32> = vec![1, 2, 3];
// A slice into a contiguous sequence (array or vector)
let slice: &[i32] = &seq[..];


fn main() {
    let p: String = String::from("galaxy one");
    let w = first_word(&p);
    println!("word: {}", w);
}

fn first_word(s: &String) -> usize {
    // convert the String to an array of bytes using `as_bytes` method
    let bytes = s.as_bytes();
    println!("bytes: {:?}", bytes); // bytes: [103, 97, 108, ...]
    /**
    create an iterator over the array of bytes. iter is a method that returns
    each element in a collection, and enumerate wraps the result of iter and
    returns each element as part of a tuple instead. The first element of the
    returned tuple is the index, and the second element is a reference to el.
    since enumerate method returns a tuple, destructure it
    Because we get a reference to the element from .iter().enumerate(),
    we use & in the destructure pattern.*/
    for (i, &el) in bytes.iter().enumerate() {
        println!("el: {}, index: {}", el, i); // el: 103, index: 0 [...]
        if el == b' ' { // or: if el == 32
            i // returns the index of space: index 6 (el: 32, index: 6)
        }
    }
    // if only 1 word, return its len
    s.len()
}

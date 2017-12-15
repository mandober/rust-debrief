// ! slice methods


// len
let a = [1, 2, 3];
assert_eq!(a.len(), 3);

// is_empty
let a = [1, 2, 3];
assert!(!a.is_empty());


// first
let v = [10, 40, 30];
assert_eq!(Some(&10), v.first());
let w: &[i32] = &[];
assert_eq!(None, w.first());

// first_mut
let x = &mut [0, 1, 2];
if let Some(first) = x.first_mut() {
    *first = 5;
}
assert_eq!(x, &[5, 1, 2]);


// split_first
let x = &[0, 1, 2];
if let Some((first, elements)) = x.split_first() {
    assert_eq!(first, &0);
    assert_eq!(elements, &[1, 2]);
}

// split_first_mut
let x = &mut [0, 1, 2];
if let Some((first, elements)) = x.split_first_mut() {
    *first = 3;
    elements[0] = 4;
    elements[1] = 5;
}
assert_eq!(x, &[3, 4, 5]);

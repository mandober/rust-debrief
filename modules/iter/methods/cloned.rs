fn cloned<'a, T>(self) -> Cloned<Self>
   where Self: Iterator<Item = &'a T>, T: 'a + Clone;
/**
Creates an iterator which clones all of its elements.

This is useful when you have an iterator over &T,
but you need an iterator over T.

EXAMPLES:
*/
let a = [1, 2, 3];
let v_cloned: Vec<_> = a.iter().cloned().collect();

// cloned is the same as .map(|&x| x), for integers
let v_map: Vec<_> = a.iter().map(|&x| x).collect();

assert_eq!(v_cloned, vec![1, 2, 3]);
assert_eq!(v_map, vec![1, 2, 3]);

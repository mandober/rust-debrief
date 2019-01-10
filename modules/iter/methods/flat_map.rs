fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F> where
    F: FnMut(Self::Item) -> U,
    U: IntoIterator;
/**
Creates an iterator that works like map, but flattens nested structure.

The map adapter is very useful, but only when the closure argument produces values.
If it produces an iterator instead, there's an extra layer of indirection.
flat_map() will remove this extra layer on its own.

Another way of thinking about flat_map:
     `map`: closure returns an item for each element
`flat_map`: closure returns an iterator for each element

*/
let words = ["alpha", "beta", "gamma"];

let merged: String = words
                      .iter()
                      .flat_map(|s| s.chars())
                      .collect();

assert_eq!(merged, "alphabetagamma");

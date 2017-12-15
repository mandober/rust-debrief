# Slice methods


## Slice Methods Index

```rust

len
is_empty
first
first_mut

split_first
split_first_mut
split_last
split_last_mut

get
get_mut
get_unchecked_mut

last
last_mut
as_ptr
as_mut_ptr
swap
reverse
iter
iter_mut
chunks_mut
split_mut
rsplit
rsplit_mut
splitn_mut
rsplitn
rsplitn_mut
contains
starts_with
ends_with
binary_search
binary_search_by
binary_search_by_key
sort_by
sort_by_key
sort_unstable
sort_unstable_by
sort_unstable_by_key
slide
clone_from_slice
copy_from_slice
swap_with_slice
```




## Slice methods

```rust
impl<T> {
    fn len(&self) -> usize;
    // Returns the number of elements in the slice.

    fn is_empty(&self) -> bool;
    // Returns true if the slice has a length of 0.

    fn first(&self) -> Option<&T>;
    // Returns the first element of the slice, or None if empty.

    fn first_mut(&mut self) -> Option<&mut T>;
    // Returns a mut ptr to 1.element of the slice, or None if empty.

    fn split_first(&self) -> Option<(&T, &[T])>; // 1.5.0
    // Returns a tuple of &first AND &rest of elements, or None if empty

    fn split_first_mut(&mut self) -> Option<(&mut T, &mut [T])>; // 1.5.0
    // Returns tuple of &mut first AND &mut rest of elements, or None if empty

}
```






```rust
let x = &mut [0, 1, 2];

if let Some((first, elements)) = x.split_first_mut() {
    *first = 3;
    elements[0] = 4;
    elements[1] = 5;
}
assert_eq!(x, &[3, 4, 5]);
```

```rust
fn split_last(&self) -> Option<(&T, &[T])> // 1.5.0
```
Returns the last and all the rest of the elements of the slice, or None if it is empty.

```rust
let x = &[0, 1, 2];

if let Some((last, elements)) = x.split_last() {
    assert_eq!(last, &2);
    assert_eq!(elements, &[0, 1]);
}
```


```rust
fn split_last_mut(&mut self) -> Option<(&mut T, &mut [T])> // 1.5.0
```
Returns the last and all the rest of the elements of the slice, or None if it is empty.

Examples

let x = &mut [0, 1, 2];

if let Some((last, elements)) = x.split_last_mut() {
    *last = 3;
    elements[0] = 4;
    elements[1] = 5;
}
assert_eq!(x, &[4, 5, 3]);Run
fn last(&self) -> Option<&T>[src][−]

Returns the last element of the slice, or None if it is empty.

Examples

let v = [10, 40, 30];
assert_eq!(Some(&30), v.last());

let w: &[i32] = &[];
assert_eq!(None, w.last());


fn last_mut(&mut self) -> Option<&mut T>

Returns a mutable pointer to the last item in the slice.

Examples

let x = &mut [0, 1, 2];

if let Some(last) = x.last_mut() {
    *last = 10;
}
assert_eq!(x, &[0, 1, 10]);


fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output> 
where
    I: SliceIndex<[T]>, 

Returns a reference to an element or subslice depending on the type of index.

If given a position, returns a reference to the element at that position or None if out of bounds.
If given a range, returns the subslice corresponding to that range, or None if out of bounds.
Examples

let v = [10, 40, 30];
assert_eq!(Some(&40), v.get(1));
assert_eq!(Some(&[10, 40][..]), v.get(0..2));
assert_eq!(None, v.get(3));
assert_eq!(None, v.get(0..4));


fn get_mut<I>(
    &mut self, 
    index: I
) -> Option<&mut <I as SliceIndex<[T]>>::Output> 
where
    I: SliceIndex<[T]>, 


Returns a mutable reference to an element or subslice depending on the type of index (see get) or None if the index is out of bounds.

Examples

let x = &mut [0, 1, 2];

if let Some(elem) = x.get_mut(1) {
    *elem = 42;
}
assert_eq!(x, &[0, 42, 2]);Run
unsafe fn get_unchecked<I>(&self, index: I) -> &<I as SliceIndex<[T]>>::Output 
where
    I: SliceIndex<[T]>, 
[src][−]

Returns a reference to an element or subslice, without doing bounds checking.

This is generally not recommended, use with caution! For a safe alternative see get.

Examples

let x = &[1, 2, 4];

unsafe {
    assert_eq!(x.get_unchecked(1), &2);
}Run
unsafe fn get_unchecked_mut<I>(
    &mut self, 
    index: I
) -> &mut <I as SliceIndex<[T]>>::Output 
where
    I: SliceIndex<[T]>, 
[src][−]

Returns a mutable reference to an element or subslice, without doing bounds checking.

This is generally not recommended, use with caution! For a safe alternative see get_mut.

Examples

let x = &mut [1, 2, 4];

unsafe {
    let elem = x.get_unchecked_mut(1);
    *elem = 13;
}
assert_eq!(x, &[1, 13, 4]);


fn as_ptr(&self) -> *const T


Returns a raw pointer to the slice's buffer.

The caller must ensure that the slice outlives the pointer this function returns, or else it will end up pointing to garbage.

Modifying the container referenced by this slice may cause its buffer to be reallocated, which would also make any pointers to it invalid.

Examples

let x = &[1, 2, 4];
let x_ptr = x.as_ptr();

unsafe {
    for i in 0..x.len() {
        assert_eq!(x.get_unchecked(i), &*x_ptr.offset(i as isize));
    }
}

fn as_mut_ptr(&mut self) -> *mut T[src][−]

Returns an unsafe mutable pointer to the slice's buffer.

The caller must ensure that the slice outlives the pointer this function returns, or else it will end up pointing to garbage.

Modifying the container referenced by this slice may cause its buffer to be reallocated, which would also make any pointers to it invalid.

Examples

let x = &mut [1, 2, 4];
let x_ptr = x.as_mut_ptr();

unsafe {
    for i in 0..x.len() {
        *x_ptr.offset(i as isize) += 2;
    }
}
assert_eq!(x, &[3, 4, 6]);




fn swap(&mut self, a: usize, b: usize)[src][−]

Swaps two elements in the slice.

Arguments

a - The index of the first element
b - The index of the second element
Panics

Panics if a or b are out of bounds.

Examples

let mut v = ["a", "b", "c", "d"];
v.swap(1, 3);
assert!(v == ["a", "d", "c", "b"]);


fn reverse(&mut self)[src][−]

Reverses the order of elements in the slice, in place.

Examples

let mut v = [1, 2, 3];
v.reverse();
assert!(v == [3, 2, 1]);



fn iter(&self) -> Iter<T>[src][−]

Returns an iterator over the slice.

Examples

let x = &[1, 2, 4];
let mut iterator = x.iter();

assert_eq!(iterator.next(), Some(&1));
assert_eq!(iterator.next(), Some(&2));
assert_eq!(iterator.next(), Some(&4));
assert_eq!(iterator.next(), None);




fn iter_mut(&mut self) -> IterMut<T>[src][−]

Returns an iterator that allows modifying each value.

Examples

let x = &mut [1, 2, 4];
for elem in x.iter_mut() {
    *elem += 2;
}
assert_eq!(x, &[3, 4, 6]);Run
fn windows(&self, size: usize) -> Windows<T>[src][−]

Returns an iterator over all contiguous windows of length size. The windows overlap. If the slice is shorter than size, the iterator returns no values.

Panics

Panics if size is 0.

Examples

let slice = ['r', 'u', 's', 't'];
let mut iter = slice.windows(2);
assert_eq!(iter.next().unwrap(), &['r', 'u']);
assert_eq!(iter.next().unwrap(), &['u', 's']);
assert_eq!(iter.next().unwrap(), &['s', 't']);
assert!(iter.next().is_none());Run
If the slice is shorter than size:

let slice = ['f', 'o', 'o'];
let mut iter = slice.windows(4);
assert!(iter.next().is_none());Run
fn chunks(&self, size: usize) -> Chunks<T>[src][−]

Returns an iterator over size elements of the slice at a time. The chunks are slices and do not overlap. If size does not divide the length of the slice, then the last chunk will not have length size.

Panics

Panics if size is 0.

Examples

let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.chunks(2);
assert_eq!(iter.next().unwrap(), &['l', 'o']);
assert_eq!(iter.next().unwrap(), &['r', 'e']);
assert_eq!(iter.next().unwrap(), &['m']);
assert!(iter.next().is_none());


fn chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<T>[src][−]

Returns an iterator over chunk_size elements of the slice at a time. The chunks are mutable slices, and do not overlap. If chunk_size does not divide the length of the slice, then the last chunk will not have length chunk_size.

Panics

Panics if chunk_size is 0.

Examples

let v = &mut [0, 0, 0, 0, 0];
let mut count = 1;

for chunk in v.chunks_mut(2) {
    for elem in chunk.iter_mut() {
        *elem += count;
    }
    count += 1;
}
assert_eq!(v, &[1, 1, 2, 2, 3]);


fn split_at(&self, mid: usize) -> (&[T], &[T])[src][−]

Divides one slice into two at an index.

The first will contain all indices from [0, mid) (excluding the index mid itself) and the second will contain all indices from [mid, len) (excluding the index len itself).

Panics

Panics if mid > len.

Examples

let v = [1, 2, 3, 4, 5, 6];

{
   let (left, right) = v.split_at(0);
   assert!(left == []);
   assert!(right == [1, 2, 3, 4, 5, 6]);
}

{
    let (left, right) = v.split_at(2);
    assert!(left == [1, 2]);
    assert!(right == [3, 4, 5, 6]);
}

{
    let (left, right) = v.split_at(6);
    assert!(left == [1, 2, 3, 4, 5, 6]);
    assert!(right == []);
}


fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T])[src][−]

Divides one &mut into two at an index.

The first will contain all indices from [0, mid) (excluding the index mid itself) and the second will contain all indices from [mid, len) (excluding the index len itself).

Panics

Panics if mid > len.

Examples

let mut v = [1, 0, 3, 0, 5, 6];
// scoped to restrict the lifetime of the borrows
{
    let (left, right) = v.split_at_mut(2);
    assert!(left == [1, 0]);
    assert!(right == [3, 0, 5, 6]);
    left[1] = 2;
    right[1] = 4;
}
assert!(v == [1, 2, 3, 4, 5, 6]);


fn split<F>(&self, pred: F) -> Split<T, F> 
where
    F: FnMut(&T) -> bool, 
[src][−]

Returns an iterator over subslices separated by elements that match pred. The matched element is not contained in the subslices.

Examples

let slice = [10, 40, 33, 20];
let mut iter = slice.split(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10, 40]);
assert_eq!(iter.next().unwrap(), &[20]);
assert!(iter.next().is_none());Run
If the first element is matched, an empty slice will be the first item returned by the iterator. Similarly, if the last element in the slice is matched, an empty slice will be the last item returned by the iterator:

let slice = [10, 40, 33];
let mut iter = slice.split(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10, 40]);
assert_eq!(iter.next().unwrap(), &[]);
assert!(iter.next().is_none());Run
If two matched elements are directly adjacent, an empty slice will be present between them:

let slice = [10, 6, 33, 20];
let mut iter = slice.split(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10]);
assert_eq!(iter.next().unwrap(), &[]);
assert_eq!(iter.next().unwrap(), &[20]);
assert!(iter.next().is_none());Run
fn split_mut<F>(&mut self, pred: F) -> SplitMut<T, F> 
where
    F: FnMut(&T) -> bool, 
[src][−]

Returns an iterator over mutable subslices separated by elements that match pred. The matched element is not contained in the subslices.

Examples

let mut v = [10, 40, 30, 20, 60, 50];

for group in v.split_mut(|num| *num % 3 == 0) {
    group[0] = 1;
}
assert_eq!(v, [1, 40, 30, 1, 60, 1]);Run
fn rsplit<F>(&self, pred: F) -> RSplit<T, F> 
where
    F: FnMut(&T) -> bool, 
[src][−]

🔬 This is a nightly-only experimental API. (slice_rsplit #41020)
Returns an iterator over subslices separated by elements that match pred, starting at the end of the slice and working backwards. The matched element is not contained in the subslices.

Examples

#![feature(slice_rsplit)]

let slice = [11, 22, 33, 0, 44, 55];
let mut iter = slice.rsplit(|num| *num == 0);

assert_eq!(iter.next().unwrap(), &[44, 55]);
assert_eq!(iter.next().unwrap(), &[11, 22, 33]);
assert_eq!(iter.next(), None);Run
As with split(), if the first or last element is matched, an empty slice will be the first (or last) item returned by the iterator.

#![feature(slice_rsplit)]

let v = &[0, 1, 1, 2, 3, 5, 8];
let mut it = v.rsplit(|n| *n % 2 == 0);
assert_eq!(it.next().unwrap(), &[]);
assert_eq!(it.next().unwrap(), &[3, 5]);
assert_eq!(it.next().unwrap(), &[1, 1]);
assert_eq!(it.next().unwrap(), &[]);
assert_eq!(it.next(), None);Run
fn rsplit_mut<F>(&mut self, pred: F) -> RSplitMut<T, F> 
where
    F: FnMut(&T) -> bool, 
[src][−]

🔬 This is a nightly-only experimental API. (slice_rsplit #41020)
Returns an iterator over mutable subslices separated by elements that match pred, starting at the end of the slice and working backwards. The matched element is not contained in the subslices.

Examples

#![feature(slice_rsplit)]

let mut v = [100, 400, 300, 200, 600, 500];

let mut count = 0;
for group in v.rsplit_mut(|num| *num % 3 == 0) {
    count += 1;
    group[0] = count;
}
assert_eq!(v, [3, 400, 300, 2, 600, 1]);Run
fn splitn<F>(&self, n: usize, pred: F) -> SplitN<T, F> 
where
    F: FnMut(&T) -> bool, 
[src][−]

Returns an iterator over subslices separated by elements that match pred, limited to returning at most n items. The matched element is not contained in the subslices.

The last element returned, if any, will contain the remainder of the slice.

Examples

Print the slice split once by numbers divisible by 3 (i.e. [10, 40], [20, 60, 50]):

let v = [10, 40, 30, 20, 60, 50];

for group in v.splitn(2, |num| *num % 3 == 0) {
    println!("{:?}", group);
}Run
fn splitn_mut<F>(&mut self, n: usize, pred: F) -> SplitNMut<T, F> 
where
    F: FnMut(&T) -> bool, 
[src][−]

Returns an iterator over subslices separated by elements that match pred, limited to returning at most n items. The matched element is not contained in the subslices.

The last element returned, if any, will contain the remainder of the slice.

Examples

let mut v = [10, 40, 30, 20, 60, 50];

for group in v.splitn_mut(2, |num| *num % 3 == 0) {
    group[0] = 1;
}
assert_eq!(v, [1, 40, 30, 1, 60, 50]);Run
fn rsplitn<F>(&self, n: usize, pred: F) -> RSplitN<T, F> 
where
    F: FnMut(&T) -> bool, 
[src][−]

Returns an iterator over subslices separated by elements that match pred limited to returning at most n items. This starts at the end of the slice and works backwards. The matched element is not contained in the subslices.

The last element returned, if any, will contain the remainder of the slice.

Examples

Print the slice split once, starting from the end, by numbers divisible by 3 (i.e. [50], [10, 40, 30, 20]):

let v = [10, 40, 30, 20, 60, 50];

for group in v.rsplitn(2, |num| *num % 3 == 0) {
    println!("{:?}", group);
}Run
fn rsplitn_mut<F>(&mut self, n: usize, pred: F) -> RSplitNMut<T, F> 
where
    F: FnMut(&T) -> bool, 
[src][−]

Returns an iterator over subslices separated by elements that match pred limited to returning at most n items. This starts at the end of the slice and works backwards. The matched element is not contained in the subslices.

The last element returned, if any, will contain the remainder of the slice.

Examples

let mut s = [10, 40, 30, 20, 60, 50];

for group in s.rsplitn_mut(2, |num| *num % 3 == 0) {
    group[0] = 1;
}
assert_eq!(s, [1, 40, 30, 20, 60, 1]);Run
fn contains(&self, x: &T) -> bool 
where
    T: PartialEq<T>, 
[src][−]

Returns true if the slice contains an element with the given value.

Examples

let v = [10, 40, 30];
assert!(v.contains(&30));
assert!(!v.contains(&50));Run
fn starts_with(&self, needle: &[T]) -> bool 
where
    T: PartialEq<T>, 
[src][−]

Returns true if needle is a prefix of the slice.

Examples

let v = [10, 40, 30];
assert!(v.starts_with(&[10]));
assert!(v.starts_with(&[10, 40]));
assert!(!v.starts_with(&[50]));
assert!(!v.starts_with(&[10, 50]));Run
Always returns true if needle is an empty slice:

let v = &[10, 40, 30];
assert!(v.starts_with(&[]));
let v: &[u8] = &[];
assert!(v.starts_with(&[]));Run
fn ends_with(&self, needle: &[T]) -> bool 
where
    T: PartialEq<T>, 
[src][−]

Returns true if needle is a suffix of the slice.

Examples

let v = [10, 40, 30];
assert!(v.ends_with(&[30]));
assert!(v.ends_with(&[40, 30]));
assert!(!v.ends_with(&[50]));
assert!(!v.ends_with(&[50, 30]));Run
Always returns true if needle is an empty slice:

let v = &[10, 40, 30];
assert!(v.ends_with(&[]));
let v: &[u8] = &[];
assert!(v.ends_with(&[]));Run
fn binary_search(&self, x: &T) -> Result<usize, usize> 
where
    T: Ord, 
[src][−]

Binary searches this sorted slice for a given element.

If the value is found then Ok is returned, containing the index of the matching element; if the value is not found then Err is returned, containing the index where a matching element could be inserted while maintaining sorted order.

Examples

Looks up a series of four elements. The first is found, with a uniquely determined position; the second and third are not found; the fourth could match any position in [1, 4].

let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

assert_eq!(s.binary_search(&13),  Ok(9));
assert_eq!(s.binary_search(&4),   Err(7));
assert_eq!(s.binary_search(&100), Err(13));
let r = s.binary_search(&1);
assert!(match r { Ok(1...4) => true, _ => false, });Run
fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize> 
where
    F: FnMut(&'a T) -> Ordering, 
[src][−]

Binary searches this sorted slice with a comparator function.

The comparator function should implement an order consistent with the sort order of the underlying slice, returning an order code that indicates whether its argument is Less, Equal or Greater the desired target.

If a matching value is found then returns Ok, containing the index for the matched element; if no match is found then Err is returned, containing the index where a matching element could be inserted while maintaining sorted order.

Examples

Looks up a series of four elements. The first is found, with a uniquely determined position; the second and third are not found; the fourth could match any position in [1, 4].

let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

let seek = 13;
assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Ok(9));
let seek = 4;
assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(7));
let seek = 100;
assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(13));
let seek = 1;
let r = s.binary_search_by(|probe| probe.cmp(&seek));
assert!(match r { Ok(1...4) => true, _ => false, });Run
fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize> 
where
    B: Ord,
    F: FnMut(&'a T) -> B, 
1.10.0	[src][−]

Binary searches this sorted slice with a key extraction function.

Assumes that the slice is sorted by the key, for instance with sort_by_key using the same key extraction function.

If a matching value is found then returns Ok, containing the index for the matched element; if no match is found then Err is returned, containing the index where a matching element could be inserted while maintaining sorted order.

Examples

Looks up a series of four elements in a slice of pairs sorted by their second elements. The first is found, with a uniquely determined position; the second and third are not found; the fourth could match any position in [1, 4].

let s = [(0, 0), (2, 1), (4, 1), (5, 1), (3, 1),
         (1, 2), (2, 3), (4, 5), (5, 8), (3, 13),
         (1, 21), (2, 34), (4, 55)];

assert_eq!(s.binary_search_by_key(&13, |&(a,b)| b),  Ok(9));
assert_eq!(s.binary_search_by_key(&4, |&(a,b)| b),   Err(7));
assert_eq!(s.binary_search_by_key(&100, |&(a,b)| b), Err(13));
let r = s.binary_search_by_key(&1, |&(a,b)| b);
assert!(match r { Ok(1...4) => true, _ => false, });Run
fn sort(&mut self) 
where
    T: Ord, 
[src][−]

Sorts the slice.

This sort is stable (i.e. does not reorder equal elements) and O(n log n) worst-case.

When applicable, unstable sorting is preferred because it is generally faster than stable sorting and it doesn't allocate auxiliary memory. See sort_unstable.

Current implementation

The current algorithm is an adaptive, iterative merge sort inspired by timsort. It is designed to be very fast in cases where the slice is nearly sorted, or consists of two or more sorted sequences concatenated one after another.

Also, it allocates temporary storage half the size of self, but for short slices a non-allocating insertion sort is used instead.

Examples

let mut v = [-5, 4, 1, -3, 2];

v.sort();
assert!(v == [-5, -3, 1, 2, 4]);Run
fn sort_by<F>(&mut self, compare: F) 
where
    F: FnMut(&T, &T) -> Ordering, 
[src][−]

Sorts the slice with a comparator function.

This sort is stable (i.e. does not reorder equal elements) and O(n log n) worst-case.

When applicable, unstable sorting is preferred because it is generally faster than stable sorting and it doesn't allocate auxiliary memory. See sort_unstable_by.

Current implementation

The current algorithm is an adaptive, iterative merge sort inspired by timsort. It is designed to be very fast in cases where the slice is nearly sorted, or consists of two or more sorted sequences concatenated one after another.

Also, it allocates temporary storage half the size of self, but for short slices a non-allocating insertion sort is used instead.

Examples

let mut v = [5, 4, 1, 3, 2];
v.sort_by(|a, b| a.cmp(b));
assert!(v == [1, 2, 3, 4, 5]);

// reverse sorting
v.sort_by(|a, b| b.cmp(a));
assert!(v == [5, 4, 3, 2, 1]);Run
fn sort_by_key<B, F>(&mut self, f: F) 
where
    B: Ord,
    F: FnMut(&T) -> B, 
1.7.0	[src][−]

Sorts the slice with a key extraction function.

This sort is stable (i.e. does not reorder equal elements) and O(n log n) worst-case.

When applicable, unstable sorting is preferred because it is generally faster than stable sorting and it doesn't allocate auxiliary memory. See sort_unstable_by_key.

Current implementation

The current algorithm is an adaptive, iterative merge sort inspired by timsort. It is designed to be very fast in cases where the slice is nearly sorted, or consists of two or more sorted sequences concatenated one after another.

Also, it allocates temporary storage half the size of self, but for short slices a non-allocating insertion sort is used instead.

Examples

let mut v = [-5i32, 4, 1, -3, 2];

v.sort_by_key(|k| k.abs());
assert!(v == [1, 2, -3, 4, -5]);Run
fn sort_unstable(&mut self) 
where
    T: Ord, 
1.20.0	[src][−]

Sorts the slice, but may not preserve the order of equal elements.

This sort is unstable (i.e. may reorder equal elements), in-place (i.e. does not allocate), and O(n log n) worst-case.

Current implementation

The current algorithm is based on pattern-defeating quicksort by Orson Peters, which combines the fast average case of randomized quicksort with the fast worst case of heapsort, while achieving linear time on slices with certain patterns. It uses some randomization to avoid degenerate cases, but with a fixed seed to always provide deterministic behavior.

It is typically faster than stable sorting, except in a few special cases, e.g. when the slice consists of several concatenated sorted sequences.

Examples

let mut v = [-5, 4, 1, -3, 2];

v.sort_unstable();
assert!(v == [-5, -3, 1, 2, 4]);Run
fn sort_unstable_by<F>(&mut self, compare: F) 
where
    F: FnMut(&T, &T) -> Ordering, 
1.20.0	[src][−]

Sorts the slice with a comparator function, but may not preserve the order of equal elements.

This sort is unstable (i.e. may reorder equal elements), in-place (i.e. does not allocate), and O(n log n) worst-case.

Current implementation

The current algorithm is based on pattern-defeating quicksort by Orson Peters, which combines the fast average case of randomized quicksort with the fast worst case of heapsort, while achieving linear time on slices with certain patterns. It uses some randomization to avoid degenerate cases, but with a fixed seed to always provide deterministic behavior.

It is typically faster than stable sorting, except in a few special cases, e.g. when the slice consists of several concatenated sorted sequences.

Examples

let mut v = [5, 4, 1, 3, 2];
v.sort_unstable_by(|a, b| a.cmp(b));
assert!(v == [1, 2, 3, 4, 5]);

// reverse sorting
v.sort_unstable_by(|a, b| b.cmp(a));
assert!(v == [5, 4, 3, 2, 1]);Run
fn sort_unstable_by_key<B, F>(&mut self, f: F) 
where
    B: Ord,
    F: FnMut(&T) -> B, 
1.20.0	[src][−]

Sorts the slice with a key extraction function, but may not preserve the order of equal elements.

This sort is unstable (i.e. may reorder equal elements), in-place (i.e. does not allocate), and O(n log n) worst-case.

Current implementation

The current algorithm is based on pattern-defeating quicksort by Orson Peters, which combines the fast average case of randomized quicksort with the fast worst case of heapsort, while achieving linear time on slices with certain patterns. It uses some randomization to avoid degenerate cases, but with a fixed seed to always provide deterministic behavior.

It is typically faster than stable sorting, except in a few special cases, e.g. when the slice consists of several concatenated sorted sequences.

Examples

let mut v = [-5i32, 4, 1, -3, 2];

v.sort_unstable_by_key(|k| k.abs());
assert!(v == [1, 2, -3, 4, -5]);Run
fn rotate(&mut self, mid: usize)[src][−]

🔬 This is a nightly-only experimental API. (slice_rotate #41891)
Permutes the slice in-place such that self[mid..] moves to the beginning of the slice while self[..mid] moves to the end of the slice. Equivalently, rotates the slice mid places to the left or k = self.len() - mid places to the right.

This is a "k-rotation", a permutation in which item i moves to position i + k, modulo the length of the slice. See Elements of Programming §10.4.

Rotation by mid and rotation by k are inverse operations.

Panics

This function will panic if mid is greater than the length of the slice. (Note that mid == self.len() does not panic; it's a nop rotation with k == 0, the inverse of a rotation with mid == 0.)

Complexity

Takes linear (in self.len()) time.

Examples

#![feature(slice_rotate)]

let mut a = [1, 2, 3, 4, 5, 6, 7];
let mid = 2;
a.rotate(mid);
assert_eq!(&a, &[3, 4, 5, 6, 7, 1, 2]);
let k = a.len() - mid;
a.rotate(k);
assert_eq!(&a, &[1, 2, 3, 4, 5, 6, 7]);

use std::ops::Range;
fn slide<T>(slice: &mut [T], range: Range<usize>, to: usize) {
    if to < range.start {
        slice[to..range.end].rotate(range.start-to);
    } else if to > range.end {
        slice[range.start..to].rotate(range.end-range.start);
    }
}
let mut v: Vec<_> = (0..10).collect();
slide(&mut v, 1..4, 7);
assert_eq!(&v, &[0, 4, 5, 6, 1, 2, 3, 7, 8, 9]);
slide(&mut v, 6..8, 1);
assert_eq!(&v, &[0, 3, 7, 4, 5, 6, 1, 2, 8, 9]);Run
fn clone_from_slice(&mut self, src: &[T]) 
where
    T: Clone, 
1.7.0	[src][−]

Copies the elements from src into self.

The length of src must be the same as self.

If src implements Copy, it can be more performant to use copy_from_slice.

Panics

This function will panic if the two slices have different lengths.

Examples

let mut dst = [0, 0, 0];
let src = [1, 2, 3];

dst.clone_from_slice(&src);
assert!(dst == [1, 2, 3]);Run
fn copy_from_slice(&mut self, src: &[T]) 
where
    T: Copy, 
1.9.0	[src][−]

Copies all elements from src into self, using a memcpy.

The length of src must be the same as self.

If src does not implement Copy, use clone_from_slice.

Panics

This function will panic if the two slices have different lengths.

Examples

let mut dst = [0, 0, 0];
let src = [1, 2, 3];

dst.copy_from_slice(&src);
assert_eq!(src, dst);Run
fn swap_with_slice(&mut self, src: &mut [T])[src][−]

🔬 This is a nightly-only experimental API. (swap_with_slice #44030)
Swaps all elements in self with those in src.

The length of src must be the same as self.

Panics

This function will panic if the two slices have different lengths.

Example

#![feature(swap_with_slice)]

let mut src = [1, 2, 3];
let mut dst = [7, 8, 9];

src.swap_with_slice(&mut dst);
assert_eq!(src, [7, 8, 9]);
assert_eq!(dst, [1, 2, 3]);Run
fn to_vec(&self) -> Vec<T> 
where
    T: Clone, 
[src][−]

Copies self into a new Vec.

Examples

let s = [10, 40, 30];
let x = s.to_vec();
// Here, `s` and `x` can be modified independently.Run
fn into_vec(self: Box<[T]>) -> Vec<T>[src][−]

Converts self into a vector without clones or allocation.

The resulting vector can be converted back into a box via Vec<T>'s into_boxed_slice method.

Examples

let s: Box<[i32]> = Box::new([10, 40, 30]);
let x = s.into_vec();
// `s` cannot be used anymore because it has been converted into `x`.

assert_eq!(x, vec![10, 40, 30]);
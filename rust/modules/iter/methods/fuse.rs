fn fuse(self) -> Fuse<Self>;
/**
Creates an iterator which ends after the first None.

After an iterator returns None, future
calls may or may not yield Some(T) again.

fuse() adapts an iterator, ensuring that after a
None is given, it will always return None forever.
*/


// an iterator which alternates between Some and None
struct Alternate {
    state: i32,
}

impl Iterator for Alternate {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let val = self.state;
        self.state = self.state + 1;

        // if it's even, Some(i32), else None
        if val % 2 == 0 {
            Some(val)
        } else {
            None
        }
    }
}

let mut iter = Alternate { state: 0 };

// we can see our iterator going back and forth
assert_eq!(iter.next(), Some(0));
assert_eq!(iter.next(), None);
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), None);

// however, once we fuse it...
let mut iter = iter.fuse();

assert_eq!(iter.next(), Some(4));
assert_eq!(iter.next(), None);

// it will always return None after the first time.
assert_eq!(iter.next(), None);
assert_eq!(iter.next(), None);
assert_eq!(iter.next(), None);

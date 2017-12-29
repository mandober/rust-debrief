// ! is_some

fn is_some(&self) -> bool;
// Returns true if the option is a Some value.

let x: Option<u32> = Some(2);
assert_eq!(x.is_some(), true);

let x: Option<u32> = None;
assert_eq!(x.is_some(), false);



// ! is_none

fn is_none(&self) -> bool;
// Returns true if the option is a None value.
let x: Option<u32> = Some(2);
assert_eq!(x.is_none(), false);

let x: Option<u32> = None;
assert_eq!(x.is_none(), true);

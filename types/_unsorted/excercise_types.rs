/**
Add redundant type annotations keeping the examples error free.

These examples run fine, there is nothing to correct, but the goal is to add
concrete (without underscore) redundant type annotations to variables, while
refraining to consult compiler's (potential) error messages.
*/
let a = [0,1,2];

let b = &[0,1,2];

let ref c = [0,1,2];

let d = vec![0,1,2];

let e = &d;

let f = &d[..];

let ref g = d[..];

let h = g;

let i = &g;

let j = vec!(a);

let k = vec!(&d[..]);

let l = [b, c];

let m = vec![f, g];





let a: [i32; 3] = [0,1,2];

let b: &[i32; 3] = &[0,1,2];

let ref c: [i32; 3] = [0,1,2];

let d: Vec<i32> = vec![0,1,2];

let e: &[i32] = &d;

let f: &[i32] = &d[..];

let ref g: [i32] = d[..];

let h: &[i32] = g;

let i: &&[i32] = &g;

let j: Vec<[i32; 3]> = vec!(a);

let k: Vec<&[i32]> = vec!(&d[..]);

let l: [&[i32; 3]; 2] = [b, c];

let m: Vec<&[i32]> = vec![f, g];

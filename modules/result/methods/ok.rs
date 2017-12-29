// ! ok

fn ok(self) -> Option<T>;

// Converts from Result<T, E> to Option<T>.

// Converts self into an Option<T>, consuming self, and discarding the error, if any.

let x: Result<u32, &str> = Ok(2);
assert_eq!(x.ok(), Some(2));

let x: Result<u32, &str> = Err("Nothing here");
assert_eq!(x.ok(), None);


let mut stdin1 = String::new();
io::stdin().read_line(&mut stdin1).expect("read error");

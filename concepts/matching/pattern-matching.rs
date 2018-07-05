// match expression

/** EXAMPLE:
Dealing with IO. Types redundantly annotated.

a) File::open tries openning a file and returns Result<T, E>, where Ok(T) is
   file handle and Err(E) is io::Error.

b) To get either Ok or Err variant Result has to be matched. Since match is an
   expression, its value can be assigned to var (here to file_handle).
   the type of file_handle var has to be the same for all arms of match statement,
   but here, since the Err arm will abort with panic, it doesn't have to match
   the error type, it just matches Ok type, which is of type "File"

c) Ok arm: if this variant matches (and it will if file is opened successfully),
   "extract" the value from Ok variant and put it in fh variable. fh will be
   the return value of the overall match expression and hence its value will be
   assigned to file_handle.

d) Err arm with MATCH GUARD. Unlike the last match arm, this one has additional
   constraint: not only does the result of File::open has to be Error, but it has
   to be "NotFound" error type (unlike the last arm that only cares about error,
   not about its kind).
   `Err(ref err)` means to extaract a REFERENCE to Err variant. If it were
   `Err(&err)` that would mean to match on reference i.e. Err variant would then
   have to contain a reference to a value, not a value itself.

e) Inner match expression will possibly return only fc value since the other
   match arm is panic. The returned fc value will than be returned value of the
   outter match exp as well.

f) the last match arm of outter match. It will match on any  other errors
   (apart from NotFound error). It doesnt return, it panics.

g) semi-colon because the match exp's returned val is assigned to let. since let
   is a statement and as such must always be terminated with semicolon.

*/

let result: Result<File, Error> = std::fs::File::open("hello.txt"); // a

let file_handle: File = match result { // b

    Ok(fh) => fh, // c

    Err(ref err) if err.kind() == std::io::ErrorKind::NotFound => { // d
        match std::fs::File::create("hello.txt") { // e
            Ok(fc) => fc,
            Err(e) => panic!("problem creating a new file: {:?}", e),
        }
    },

    Err(error) => panic!("problem opening the file: {:?}", error), // f

}; // g




// assign to f the matching value
let f = match result {
    Ok(v) => v,
    Err(e) => panic!("{:?}", e),
};

// if let variant:
if let Ok(v) = result {
    let f = v; // but then f falls out of scope. fix: declare it early
}









// An enum and a match expression that has the variants of the enum as its patterns
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// If you want to run multiple lines of code in a match arm, you can use curly braces
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// * Patterns that Bind to Values
// match arms can bind to parts of the values that match the pattern.
// This is how we can extract values out of enum variants.

#[derive(Debug)] // So we can inspect the state
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
// In the match expression for this code, we add a variable called state to the pattern that matches values of the variant Coin::Quarter. When a Coin::Quarter matches, the state variable will bind to the value of that quarter’s state. Then we can use state in the code for that arm,
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
// If we were to call
value_in_cents(Coin::Quarter(UsState::Alaska))
// coin would be Coin::Quarter(UsState::Alaska)
// so the last arm matches. At that point, the binding for state will be the value UsState::Alaska.
// We can then use that binding in the println! expression, thus getting the inner state value out of the Coin enum variant for Quarter.


// * Matching with Option<T>

// Let’s say we want to write a function that takes an Option<i32>, and if there’s a value inside, adds one to that value. If there isn’t a value inside, the function should return the None value and not attempt to perform any operations.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
// When we call `plus_one(five)`, the variable x in the body of plus_one will have the value Some(5).
// We then compare that against each match arm.
// In second arm Some(5) matches Some(i)
// the `i` binds to the value contained in Some, so `i` takes the value 5
// The code in the match arm is then executed, so we add 1 to the value of i
// AND CREATE A NEW `Some` VALUE with 6 inside. It returns: Some<i32>(6)

// But when x is None, the first arm will match: None => None
// so the program stops and returns the `None` value on the right side of =>.
// Because the first arm matched, no other arms are compared.

//? Matches Are Exhaustive
// we mustcover every possible case

//? The _ Placeholder
// pattern _ is a catch all
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
// The _ pattern will match any value
// The () is just the unit value, so nothing will happen in the _ case.

// However, the match expression can be a bit wordy in a situation in which we only care about one of the cases. For this situation, Rust provides `if let`.


// * if let
// The if let syntax lets you combine if and let into a less verbose way to
// handle values that match one pattern and ignore the rest.

// A match that only cares about executing code when the value is Some(3)
let val = Some(0u8);
match val {
    Some(3) => println!("three"),
    _ => (),
}
// Here, we want to do something with the Some(3) match but do nothing with any
// other `Some<u8>` value or the `None` value. Instead of the above, we can use
// `if let` (shortcut to above):
if let Some(3) = val {
    println!("three");
}
// `if let` takes a pattern and an expression separated by an `=`
// It works the same way as a match, where the expression is
// given to the match and the pattern is its first arm.


// We can include an `else` with an `if let`:
// The block of code that goes with the `else` is the same as the block of
// code that would go with the `_` case in the match expression that is equivalent
// to the `if let else`.

// with a match expression
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}

// with `if let else` expression (shortcut to above)
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}






// ! if let
// * example 1:
// let opt = Some(3);
match opt {
    Some(v) => {
        // on match do (return) this:
        v + 5
    },
    None => (),
}
// if let shortcut to above
if let Some(v) = opt {
    // on match do (return) this:
    v + 5
}



// * example 2:
// assign to f the matching value
let f = match result {
    Ok(v) => v,
    Err(e) => panic!("{:?}", e),
};
// if let:
if let Ok(v) = result {
    let f = v;
}



// * example 3:

// take
fn take(&mut self) -> Option<T>
// eg
let mut state: Option<Box<State>> = Some(Box::new(Draft {}));
state.take()
assert_eq!(state, None);


impl Post {
    pub fn request_review(&mut self) {

        // if let
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }

        // or:
        self.state = match self.state.take() {
            Some(s) => Some(s.request_review()),
             _ => (),
        }

        // match
        match self.state.take() {
            Some(s) => {
                self.state = Some(s.request_review())
            },
            _ => (),
        }

    }
}

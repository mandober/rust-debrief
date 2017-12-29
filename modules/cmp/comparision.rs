// comparing

// bringing a type called std::cmp::Ordering into scope from the standard library.
use std::cmp::Ordering;
// Ordering is another enum, like Result, but the variants for Ordering are 
// Less, Greater, and Equal. 

fn main() {
    let n1 = 50u8;
    let n2 = 33u8;

    // The cmp method compares two values and can be called on anything that 
    // can be compared. It takes a REFERENCE to whatever you want to compare with
    match n1.cmp(&n2) {
        Ordering::Less    => println!("{} is less than {}", n1, n2),
        Ordering::Greater => println!("{} is greater than {}", n1, n2),
        Ordering::Equal   => println!("{} and {} are equal", n1, n2),
    }
}

// use a match expression to decide what to do next based on which variant of 
// Ordering was returned from the call to cmp


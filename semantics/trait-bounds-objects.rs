// ! Trait bounds
/**

To compare 2 values of type T using the greater-than operator, which is defined
as a default method on trait std::cmp::PartialOrd, we put `PartialOrd` in the
trait bounds for T, so it works on slices of any type that can be compared.
PartialOrd is in the prelude.
*/
fn largest<T: PartialOrd>(list: &[T]) -> T {
    let mut maxy = list[0];
    for &n in list.iter() {
        if maxy < n { maxy = n; }
    }
    maxy
}

/**
but different errors occur:
  error[E0508]: cannot move out of type `[T]`, a non-copy array
  error[E0507]: cannot move out of borrowed content

The key to this error is cannot move out of type [T], a non-copy array.
With our non-generic versions of the largest function, we were only trying to
find the largest i32 or char.
Types like i32 and char that have a known size can be stored on the stack, so
they implement the Copy trait.
When we changed the largest function to be generic, it’s now possible that the
list parameter could have types in it that don’t implement the Copy trait, which
means we wouldn’t be able to move the value out of list[0] and into the largest
variable.

If we only want to be able to call this code with types that are Copy, we can
add Copy to the trait bounds of T.
*/
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

/**
If we don’t want to restrict our largest function to only types that implement the Copy trait, we could specify that T has the trait bound Clone instead of Copy and clone each value in the slice when we want the largest function to have ownership.
Using the clone function means we’re potentially making more heap allocations, though, and heap allocations can be slow if we’re working with large amounts of data.

Another way we could implement largest is for the function to return a reference to a T value in the slice. If we change the return type to be &T instead of T and change the body of the function to return a reference, we wouldn’t need either the Clone or Copy trait bounds and we wouldn’t be doing any heap allocations.
*/



// ! generic fns
/**
Rust can usually deduce the type of parameters of generic functions by looking at the arguments. If the generic function you’re calling doesn’t have any arguments that provide useful clues, you may have to spell it out: */

// calling a generic method collect<C>() that takes no arguments
let v1 = (0..1000).collect(); // error: can't infer type
let v2 = (0..1000).collect::<Vec<i32>>(); // ok

// Sometimes we need multiple abilities from a type parameter. For example, if we want to print out the top ten most common values in a vector, we’ll need for those values to be printable:
use std::fmt::Debug;
fn top_ten<T: Debug>(values: &Vec<T>) { /* */ }
// but how can we determine which values are the most common? The usual way is use the values as keys in a hash table. That means the values need to support the Hash and Eq operations. The bounds on T must include these as well as Debug. The syntax for this uses the + sign:
fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) { /* */ }


// Generic functions can have multiple type parameters:
// Run a query on a large, partitioned data set.
// See <http://research.google.com/archive/mapreduce.html>.
fn run_query<M: Mapper + Serialize, R: Reducer + Serialize>(
    data: &DataSet, map: M, reduce: R) -> Results
{ }

// Rust provides an alternative syntax using the keyword where:
fn run_query<M, R>(data: &DataSet, map: M, reduce: R) -> Results
    where M: Mapper + Serialize,
          R: Reducer + Serialize
{ }



// Type aliases can be generic, too:
type PancakeResult<T> = Result<T, PancakeError>;


// * trait objects or generics
/**
The choice of whether to use trait objects or generic code is subtle. Since both features are based on traits, they have a lot in common.

Trait objects are the right choice whenever you need a collection of values of mixed types, all together.

Another possible reason to use trait objects is to reduce the total amount of compiled code. Rust may have to compile a generic function many times, once for each type it’s used with.

However, generics have two important advantages over trait objects, with the result that in Rust, generics are the more common choice.

The first advantage is speed. Each time the Rust compiler generates machine code for a generic function, it knows which types it’s working with, so it knows at that time which write method to call. There’s no need for dynamic dispatch.
Some generic function named `min` is just as fast as if we had written separate functions min_u8, min_i64, min_string, and so on.

Compare that to the behavior with trait objects. Rust never knows what type of value a trait object points to until run time.

The second advantage of generics is that not every trait can support trait objects. Traits support several features, such as static methods, that work only with generics: they rule out trait objects entirely.

*/

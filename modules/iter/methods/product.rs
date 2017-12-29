fn product<P>(self) -> P
   where P: Product<Self::Item>;
/**
1.11.0

Iterates over the entire iterator, multiplying all the elements

An empty iterator returns the one value of the type.

Panics:
When calling product() and a primitive integer type is being returned,
method will panic if the computation overflows and debug assertions are enabled.
*/

fn factorial(n: u32) -> u32 {
    (1..).take_while(|&i| i <= n).product()
}

assert_eq!(factorial(0), 1);
assert_eq!(factorial(1), 1);
assert_eq!(factorial(5), 120);

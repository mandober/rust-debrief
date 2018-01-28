// ! ARRAYS
/**
An example is an array holding more than 32 elements of a type that is `Clone`;
the standard library only implements `Clone` up until arrays of size 32.
In this case, implementation of `Clone` cannot be derived, but can be impl as:
*/
let arr1 = [1, 2, 3]; // (array of 3 elements)
let arr2 = [2; 32];   // (array of 32 elements of 2s)


// find max el in array
fn main() {
    let arr = [2, 5, 8, 4, 6];
    let mut maxy = &arr[0];
    println!("{}", maxy);

    for n in arr.iter() {
        if maxy < n {
            maxy = n;
        }
    }
    println!("The largest number is {}", maxy);
}


// ! Multidimensional arrays
// You can easily write array having several dimensions:
let mut x = [[[[23; 4]; 6]; 8]; 15];
x[14][7][5][3] = 56;
print!("{}, {}", x[0][0][0][0], x[14][7][5][3]); // 23, 56

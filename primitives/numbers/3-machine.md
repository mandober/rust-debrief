# Machine-dependent integer types

Both types have the same number of bits as the platform's pointer type.
On x64 architectures it has 64bits (8Bytes), on x86 it has 32bits (8Bytes).

`usize`
unsigned integer type with the same number of bits as the platform's pointer type.
It can *represent every memory address* in the process.

As an always positive value it can represent every memory address; 
it is commonly used as a type for annotating length values, like array's length;


`isize`
signed integer type with the same number of bits as the platform's pointer type. 

Theoretical *upper bound* on *object and array size* is the maximum `isize` value.
This ensures that isize can be used to calculate differences between pointers 
into an object or array and can address every byte within an object along with 
one byte past the end.

isize complements usize like other integer types do, but its practical use is 
to represent a difference between sizes, since this can yield a negative number;
like result of subtraction between 2 memory addresses, array lengths, etc.


types: `isize`, `usize`
ref: `&usize`
mut ref: `&mut usize`


kind: primitive, concrete, fixed, numeric  
sized: yes  
size: 64bits@x64 (32b@x86)  
storage: stack  
std: primitive types, module  
sample: `let len: usize = 42`  


traits: `Copy`, `Debug`, `Display` 
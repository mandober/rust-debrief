# Data Structures

- array
- linked-list



## implicit data structure
an implicit data structure or space-efficient data structure is a data structure that stores very little information other than the main or required data: a data structure that requires low overhead. They are called "implicit" because the position of the elements carries meaning and relationship between elements; this is contrasted with the use of pointers to give an explicit relationship between elements. Definitions of "low overhead" vary, but generally means constant overhead; in big O notation, O(1) overhead. A less restrictive definition is a succinct data structure, which allows greater overhead.

A fundamental distinction is between static data structures (read-only) and dynamic data structures (which can be modified). Simple implicit data structures, such as representing a sorted list as an array, may be very efficient as a static data structure, but inefficient as a dynamic data structure, due to modification operations (such as insertion in the case of a sorted list) being inefficient.

A trivial example of an implicit data structure is an array data structure, which is an implicit data structure for a list, and requires only the constant overhead of the length; unlike a linked list, which has a pointer associated with each data element, which explicitly gives the relationship from one element to the next.

## succinct data structure
is a data structure which uses an amount of space that is "close" to the information-theoretic lower bound, but (unlike other compressed representations) still allows for efficient query operations. The concept was originally introduced by Jacobson to encode bit vectors, (unlabeled) trees, and planar graphs. Unlike general lossless data compression algorithms, succinct data structures retain the ability to use them in-place, without decompressing them first. A related notion is that of a compressed data structure, in which the size of the data structure depends upon the particular data being represented.

## compressed data structure
refers to a data structure whose operations are roughly as fast as those of a conventional data structure for the problem, but whose size can be substantially smaller. The size of the compressed data structure is typically highly dependent upon the entropy of the data being represented.
- compressed suffix array
- the FM-index


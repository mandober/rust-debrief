# Lists

A linked list is a linear collection of elements (called node), whose linearity is not inherited from its physical placement in memory (as it is with arrays), but each node has a pointer to the next node, thus forming a linear sequence.

In the simplest form, each node is composed of data and a reference (acting as a link) to the next node in the sequence. This form allows for efficient insertion or removal of elements from any position in the sequence.

More complex variants of linked lists have additional, backwards, links, which  facilitate a two way movement (back as well as forth) during list manipulations.

A drawback of linked lists is that access time is linear, O(n). Faster access, such as random access, is not feasible. Also since they are not a compact data structure (like arrays) cache locality is poor resulting in lots of cache misses.  


# List variants
- linkage: 
  - singly-linked
  - doubly-linked
    - XOR list
- circular list
- unrolled list
- skip lists



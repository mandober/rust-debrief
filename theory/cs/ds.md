# Data structures

http://opendatastructures.org/

An interface describes what a data structure does, while an implementation describes how the data structure does it.

An interface, sometimes also called an abstract data type, defines the set of operations supported by a data structure and the semantics, or meaning, of those operations.

An interface tells us nothing about how the data structure implements these operations; it only provides a list of supported operations along with specifications about what types of arguments each operation accepts and the value returned by each operation.

A data structure implementation, on the other hand, includes the internal representation of the data structure as well as the definitions of the algorithms that implement the operations supported by the data structure. 

## The Queue, Stack, and Deque Interfaces
The Queue interface represents a collection of elements to which we can add elements and remove the next element.
- `add(v)` add the value to the Queue
- `remove()` remove the next (previously added) value from the Queue and return it. The Queue's queueing discipline decides which element should be removed.

There are many possible queueing disciplines, the most common of which include FIFO, priority, and LIFO.

### FIFO
A FIFO (first-in-first-out) Queue removes items in the same order they were added, much in the same way a queue (or line-up) works when checking out at a cash register in a grocery store. This is the most common kind of Queue so the qualifier FIFO is often omitted.
- `add(v)` is often called `enqueue(v)`
- `remove` is `dequeue()`




















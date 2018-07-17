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


---




# Data structures

https://www.wikiwand.com/en/Data_structure

A data structure is a particular way of organizing and storing data, so that it can be accessed and modified efficiently. More precisely, it is a collection of data values together with the relationships among them and the operations that can be applied to the data.

Data structures can implement one or more particular abstract data types (ADT), which specify the operations that can be performed on a data structure and the computational complexity of those operations.

> A data structure is a concretion of a particular ADT.

Different kinds of data structures are suited to different kinds of applications, and some are highly specialized to specific tasks. For example, relational databases commonly use B-tree indexes for data retrieval, while compiler implementations usually use hash tables to look up identifiers.

Data structures provide a means to manage large amounts of data efficiently for uses such as large databases and internet indexing services. Usually, efficient data structures are key to designing efficient algorithms.

Some formal design methods and programming languages emphasize data structures, rather than algorithms, as the key organizing factor in software design. Data structures can be used to organize the storage and retrieval of information stored in main memory and in secondary storage.


## Implementation
Data structures are generally based on the ability of a computer to fetch and store data at any place in its memory, specified by a pointer.

A pointer is a bit string representing a memory address, that can be itself stored in memory and manipulated by the program.

Thus, the array and record data structures are based on computing the addresses of data items with arithmetic operations; while the linked data structures are based on storing addresses of data items within the structure itself. Many data structures use both principles, sometimes combined in non-trivial ways.

The implementation of a data structure usually requires writing a set of procedures that create and manipulate instances of that structure. The efficiency of a data structure cannot be analyzed separately from those operations. This observation motivates the theoretical concept of an abstract data type, a data structure that is defined indirectly by the operations that may be performed on it, and the mathematical properties of those operations (including their space and time cost).

---
https://xlinux.nist.gov/dads/HTML/dataStructure.html

## Data structure
An organization of information, usually in memory, for better algorithm efficiency, such as queue, stack, linked list, heap, dictionary, and tree, or conceptual unity, such as the name and address of a person. It may include redundant information, such as length of the list or number of nodes in a subtree.

Specialization:
- external memory data structure
- passive data structure
- active data structure
- persistent data structure
- recursive data structure

Most data structures have associated algorithms to perform operations, such as search, insert, or balance, that maintain the properties of the data structure.


## Recursive data structure
A data structure that is partially composed of smaller or simpler instances of the same data structure. For instance, a tree is composed of smaller trees (subtrees) and leaf nodes, and a list may have other lists as elements.

## Persistent data structure
A data structure that preserves its old versions, that is, previous versions may be queried in addition to the latest version.

partially persistent data structure, fully persistent data structure, confluently persistent data structure, etc.

## Active data structure
A data structure with an associated thread or process that performs internal operations to give the external behavior of another, usually more general, data structure. Also known as functional data structure.

For example, a queue is usually considered to be unbounded. However, actual queues provided by the hardware or operating system may be significantly limited. Changing the writing and reading processes to use a bounded queue makes those applications more complicated. However, an active queue can accept input from the writer through a system queue, and save items in memory or on disk if the system queue for the reader is full. When the reader's queue has space, items can be retrieved and put back in the queue. Although there are now three components, rather than just the writer and reader, the high level abstraction is very simple and clear.

## Passive data structure
A data structure that is only changed by external threads or processes, in contrast to an active data structure.

## External memory data structure
A data structure that is efficient even when accessing most of the data is very slow, such as, on a disk.


## Abstract data type (ADT)
A set of data values and associated operations that are precisely specified independent of any particular implementation.

Specializations: dictionary, stack, queue, priority queue, set, bag.

Since the data values and operations are defined with mathematical precision, rather than as an implementation in a computer language, we may reason about effects of the operations, relations to other abstract data types, whether a program implements the data type, etc.

One of the simplest abstract data types is the stack. The operations new(), push(v, S), top(S), and popOff(S) may be defined with axiomatic semantics as following.

new() returns a stack
popOff(push(v, S)) = S
top(push(v, S)) = v
where S is a stack and v is a value. (The usual pop operation is a combination of top, to return the top value, and popOff, to remove the top value.) Contrast this with the axiomatic semantics definition of a set, a dictionary, or a queue.
From these axioms, one may define equality between stacks, define a pop function which returns the top value in a non-empty stack, etc. For instance, the predicate isEmpty(S) may be added and defined with the following additional axioms.

isEmpty(new()) = true
isEmpty(push(v, S)) = false


---

List of terms relating to algorithms and data structures:
https://www.wikiwand.com/en/List_of_terms_relating_to_algorithms_and_data_structures

Dictionary of Algorithms and Data Structures:
https://xlinux.nist.gov/dads/

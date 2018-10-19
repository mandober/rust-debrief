## The heap

The stack and the heap are parts of memory that is available to your code to use at runtime, but they are structured in different ways.

The stack and the heap are abstractions that help you determine when to allocate and deallocate memory.

The stack is very fast, and is where memory is allocated in Rust by default.
But the allocation is local to a function call, and is limited in size.

The heap, on the other hand, is slower, and is explicitly allocated by your program. But it’s effectively unlimited in size, and is globally accessible.

This meaning of heap, which allocates arbitrary-sized blocks of memory in arbitrary order, is quite different from the heap data structure.

Rust stack allocates by default, which means that basic values go on the stack.

When a function gets called, some memory gets allocated for all of its local variables and some other information. This is called a *stack frame*. When the function exits, its stack frame gets deallocated. Allocation and deallocation happens automatically.

Stack allocation is very fast. Since we know all the local variables we have ahead of time, we can grab the memory all at once. And since we'll throw them all away at the same time as well, we can get rid of it very fast too. The downside is that we can't keep values around if we need them for longer than a single function. 

~ ~ ~

The stack is fast because of the way it accesses the data: it never has to search for a place to put new data or a place to get data from because that place is always the top. Another property that makes the stack fast is that all data on the stack must take up a known, fixed size.

For data with a size unknown to us at compile time or a size that might change, we can store data on the heap instead. 

The heap is less organized: when we put data on the heap, we ask for some amount of space. The operating system finds an empty spot somewhere in the heap that is big enough, marks it as being in use, and returns to us a pointer to that location. This process is called allocating on the heap, and sometimes we abbreviate the phrase as just allocating.

Pushing values onto the stack is not considered allocating. Because the pointer is a known, fixed size, we can store the pointer on the stack, but when we want the actual data, we have to follow the pointer.

Contemporary processors are faster if they jump around less in memory. 
A processor can do its job better if it works on data that’s close to other data rather than farther away. Allocating a large amount of space on the heap can also take time.

When our code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so we don’t run out of space are all problems that ownership addresses.


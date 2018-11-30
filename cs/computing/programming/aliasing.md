# Aliasing

Aliasing describes a situation in which a memory location can be accessed through different symbolic names.

Multiple concurent read accesses of a memory location does not have any negative consequences.

Multiple concurent write accesses that modify the value, trigger numerous problems - simultanous write access must be tightly controlled to avoid a barrage of issues.

Modifying the value through one name implicitly modifies the values associated with all aliased names. 

Most importantly, aliasing makes it very hard to understand, analyze and optimize programs. Aliasing issues make it difficult for compilers to generate code that runs as fast as possible.

Despite being the prevalent cause that results in non-determinism of code execution, many languages today, to a different extent, allow simultanous writes to the same memory location.

Fortunatelly, there are modern languages, like Rust, that take on this problem more seriously, and put such offending behaviour under a tight control. For example, in Rust, at any time, it is only allowed to either have multiple read references (multiple simultanous aliases that read the same memory location are allowed) XOR a single reference with write permission. That way, many issues are avoided or reported at compile-time, e.g. a situation where a writer is changing the value underneath a reader. Rust may be the only language that is completely type and memory safe, but without using a garbage collection.

Another approach comes from functional programming where modification is completely forbidden - once set, a value cannot ever be changed. But this approach cannot be implemented resonably without a garbage collection, which however efficient still introduces some non-determinism to the program.




---

https://www.wikiwand.com/en/Aliasing_(computing)
https://www.wikiwand.com/en/Pointer_aliasing


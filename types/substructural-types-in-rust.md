# Substructural types in Rust

A substructural type system restricts one or more structural rules in order to control the number of times a value may be used. Substructural type systems are useful for constraining access to system resources by keeping track of changes and preventing invalid states.

Structural rules that are controlled are weakening and contraction.

The combinations of these properties gives us 4 interesting types:
* can be used any number of times (default)
* can't be used more than once (**affine**)
* must be used at least once (**relevant**)
* must be used exactly once (**linear**)

For added confusion, sometimes linear or affine is used as a synonym for the whole substructural system.


### Default types
_can be used any number of times_

This is how almost every programming language works. Rust provides this semantic in two forms: Copy and Clone.

`Copy` is unrestricted use of the value. In Rust it has a very specific language semantic as well - copying a value can be done with a bitwise copy, and discarding the value can be done by just forgetting it (it can't implement Drop). This is in contrast to Swift and C++, which may implicitly execute code when a value is copied or destroyed (e.g. incrementing and decrementing reference counts).

`Clone` is like Copy, but where we might need to do some work when we do the copy or destroy - like the aforementionned reference count manipulation. Copying a Clone value requires an explicit call to `clone()`, while destroying a Clone value implicitly calls `drop()` (technically it calls `drop_in_place()`, which calls `drop()` and then recursively calls `drop_in_place()` on all fields).

Plain Old Data types like i32, f64, [i32; 4], etc. are Copy.
Almost every type in Rust is Clone (or it could be).


### Affine types
_can't be used more than once_

This is Rust's move semantics or move-only types.

Here Rust defines a "use" to be pass-by-value.
Pass-by-reference isn't considered a use, because we want a way to actually make use of these value more than once.
The borrow checker makes sure we get rid of all these references before we perform a "real" use.

The purpose of this type is that it gives us a simple way to prove that we have unique access to a value. If you have a move-only type by-value, you know that no one else is looking at it, and it's safe to do something that invalidates it, like freeing a pointer it owns. This also gives fairly strong aliasing guarantees for mutable references, which helps avoid tons of logic bugs.

More broadly, this system can be used as a proof of work. For instance, if I need `step1()` to always precede `step2()`, I can use a move-only value to enforce this.

Something that move-only types can't prevent is forgetting to do something. We can make sure you perform step1 before step2, but we can't actually make you do step2. Which brings us to the next kind of type.


### Relevant types
_must be used at least once_

This is the form Rust has the weak support for, but it does have some. 
Aspects of At Least Once can be seen in two places:
- the unused_variables, unused_assignments, and unused_must_use lints
- Drop

unused_variables is the simplest form. Lots of compilers support it, since it's pretty easy to implement and strongly indicative of an implementation bug.

must_use is an annotation that can be applied to a type or a function, to indicate that the function, or any function that returns a must_use type, shouldn't have its return value implicitly ignored.

Drop provides stronger support for must-use values by introducing a final step that will automatically be called on a value as it's destroyed. Assigning to a variable, printing it, stuffing it in collection, or even calling panic!() is no escape: drop() will be called.

Actually there's one escape hatch: mem::forget(). mem::forget will take any value and prevent it from being Dropped. It can be thought of as The All User of values, that any must-use value can be used by. Mostly it's just used by unsafe code to take ownership of the data owned by a Drop type (see: Vec::into_raw_parts).

Also there's some several ways for destructors to not run (often called "destructor leaking"):

building a reference counted cycle which will leak into infinity
overwriting the value with ptr::write/copy
aborting the program
never leaving the scope the value is defined for
Drop also has several annoying limitations:

It can't produce a value
It can't take extra values in
There Is Only One Drop

This means that Drop can't be used to ensure step2 gets called, nor can it be used if step3(takes, other, values). It also means it can't be used to ensure one of step3a or step3b is called.

One extreme option that I've seen is to implement drop() as abort("this value must be used"). All "proper" consumers then mem::forget the value, preventing this "destructor bomb" from going off. This provides a dynamic version of strict must-use values. Although it's still vulnerable to the few ways destructors can leak, this isn't a significant concern in practice. Mostly it just stinks because it's dynamic and Rust users Want Static Verification.

Ultimately, Rust lacks "proper" support for this kind of type.

(Edit: a commenter pointed out to me that you can hack some stronger must-use guarantees by making step3 return a value, and having the current function return a Step3Token. This makes it so that it's impossible to return from the function without a proof of completing step3. But there's lots of holes in this design, not the least of which is that the user of your API needs to be "in on it" and set that up).

(Edit2: It's possible for a library to statically require its client to call functions up to and including step3 by providing with_step1<R, F: FnOnce(Step1Token) -> (R, Step3Token)>(f: F) -> R instead of step1() -> Step1Token, which fixes the first edit's approach's issue of require the client to have prior knowledge of step3. This is a similar pattern to what scoped_threadpool uses to work around the JoinGuard unsoundness)


### Linear types
_must be used exactly once_

There's nothing actually interesting about this case. 
It's just a move-only must-use type.
People who say they want linear types In Rust actually just want proper support for relevant types.



## Adding proper must-use types to Rust

Proper support for must-use types requires 3 things:
* checker that ensures a must-use value is always locally moved (mem::forget being the ultimate destination of all must-use types)
* proper trait bound for code to support and derive must-useness with
* stdlib support


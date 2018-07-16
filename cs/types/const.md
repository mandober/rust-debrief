https://github.com/rust-lang/rfcs/pull/1909

The issue is that const ("constant"; "a value that doesn't change") has two potential meanings in a programming language:

"Not mutable", or (if applied to a value) "runtime constant". This is what const means in C and C++.

"Compile-time constant", that is, a value already known at compile-time. This is what const means in Rust. (And, if my memory's not playing tricks, Pascal?)

I've always disfavored our use of the const keyword for this purpose, for this reason - it means something different than it does in C, which is where most people are familiar with it from.

(...someone at this point is itching to bring up *const. Yes, our usage of const is also inconsistent. Given that *const is most often used for C FFI, I think this syntax is mostly justifiable by thinking of that const there as being "in the C sense", so that our *const means the same thing as their const*.)

Anyway: even if const were to mean the other thing, mut still means "mutable", which is the opposite of "immutable", and not of "compile-time". It very much does not mean "runtime value". This is kind of like suggesting we use "dark weight" to describe something that is difficult to lift because "dark" is the opposite of "light".
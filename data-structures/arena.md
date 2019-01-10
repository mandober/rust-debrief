## Arena

https://github.com/SimonSapin/rust-typed-arena/blob/master/src/lib.rs


An arena is a fast but limited type of allocator. Arenas are a type of allocator that destroy the objects within, all at once, once the arena itself is destroyed. They do not support deallocation of individual objects while the arena itself is still alive. The benefit of an arena is very fast allocation; just a vector push.


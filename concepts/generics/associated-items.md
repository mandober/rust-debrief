# Associated items

- associated items are items declared in traits and defined in impls
- assoc. items can have default values, overridable in impl




Associated items are declared within and along with a trait. Declared items may contain optional (default) definition. These declarations together make up the contract that implementors must respect.


The implementing type must define items that were only declared, it may override items containing default definition. Overriding default definition with an identical one will triger a linting error about futility of such maneuvre.

Associated items:
- Associated functions ("static" functions)
- Associated types
- Associated constants
- Associated lifetimes (?)


An associated item can be:
- _declaration_ that declares the signature a definition must respect
- _definition_ that contains the actual implementation
- definitions are optional and overridable









---

Rust's [RFC 195](https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md) defines support for associated items (functions, statics, types, lifetimes) in trait declarations. All associated items can have default values.




- https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md
- https://github.com/nox/rust-rfcs/blob/master/text/0195-associated-items.md
- https://github.com/rust-lang/rfcs/blob/master/text/1598-generic_associated_types.md
- https://internals.rust-lang.org/t/rfc-0195-associated-items-and-const-fn/4518
- https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md
- http://smallcultfollowing.com/babysteps/blog/2016/11/02/associated-type-constructors-part-1-basic-concepts-and-introduction/
- https://www.reddit.com/r/rust/comments/2deaqh/rfc_associated_items_and_multidispatch_traits/
- https://news.ycombinator.com/item?id=14915539

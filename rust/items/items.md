# Items

Each source file contains a sequence of zero or more item definitions, and may optionally begin with any number of attributes that apply to the containing module, most of which influence the behavior of the compiler. The anonymous crate module can have additional attributes that apply to the crate as a whole.

An item is a component of a crate. Items are organized within a crate by a nested set of modules. Every crate has a single "outermost" anonymous module; all further items within the crate have paths within the module tree of the crate.

Items are entirely determined at compile-time, generally remain fixed during execution, and may reside in ROM.


## List of Items
- modules
- extern crate declarations
- extern blocks
- use declarations
- function definitions
- type definitions
- trait definitions
- implementations
- struct definitions
- enumeration definitions
- union definitions
- constant items
- static items


## Sub-items
Some items form an implicit scope for the declaration of sub-items. In other words, within a function or module, declarations of items can (in many cases) be mixed with the statements, control blocks, and similar artifacts that otherwise compose the item body. The meaning of these scoped items is the same as if the item was declared outside the scope - it is still a static item - except that the item's path name within the module namespace is qualified by the name of the enclosing item, or is private to the enclosing item (in the case of functions). The grammar specifies the exact locations in which sub-item declarations may appear.

```
item :
vis ?
mod_item |
fn_item |
type_item |
struct_item | enum_item |
const_item | static_item |
trait_item |
impl_item |
extern_block_item ;
```

# Language Items

Lang items are a way for the std (and core) to define types, traits, functions, and other items which the compiler needs to know about.
Lang items are denoted with attribute `#[lang = "item"]`

Primitive types are not defined in any crate, but are implemented by the compiler. The standard library defines methods on primitives, even though coherence rules only allow for implementing methods on types defined in own crate, with the help of lang attributes whose value is the name of the primitive, e.g. `#[lang = "f64"]`. There can be only one such attribute.


Lang items [source](https://github.com/rust-lang/rust/blob/1ca100d0428985f916eea153886762bed3909771/src/librustc/middle/lang_items.rs#L252-L363).

```rust
// Variant name             Name                     Method name

// Primitives
CharImplItem,               "char",                  char_impl;
StrImplItem,                "str",                   str_impl;
SliceImplItem,              "slice",                 slice_impl;
ConstPtrImplItem,           "const_ptr",             const_ptr_impl;
MutPtrImplItem,             "mut_ptr",               mut_ptr_impl;
I8ImplItem,                 "i8",                    i8_impl;
I16ImplItem,                "i16",                   i16_impl;
I32ImplItem,                "i32",                   i32_impl;
I64ImplItem,                "i64",                   i64_impl;
I128ImplItem,               "i128",                  i128_impl;
IsizeImplItem,              "isize",                 isize_impl;
U8ImplItem,                 "u8",                    u8_impl;
U16ImplItem,                "u16",                   u16_impl;
U32ImplItem,                "u32",                   u32_impl;
U64ImplItem,                "u64",                   u64_impl;
U128ImplItem,               "u128",                  u128_impl;
UsizeImplItem,              "usize",                 usize_impl;
F32ImplItem,                "f32",                   f32_impl;
F64ImplItem,                "f64",                   f64_impl;

// Marker traits
CopyTraitLangItem,          "copy",                  copy_trait;
SendTraitLangItem,          "send",                  send_trait;
SyncTraitLangItem,          "sync",                  sync_trait;
SizedTraitLangItem,         "sized",                 sized_trait;
UnsizeTraitLangItem,        "unsize",                unsize_trait;

DropTraitLangItem,          "drop",                  drop_trait;
CoerceUnsizedTraitLangItem, "coerce_unsized",        coerce_unsized_trait;

AddTraitLangItem,           "add",                   add_trait;
SubTraitLangItem,           "sub",                   sub_trait;
MulTraitLangItem,           "mul",                   mul_trait;
DivTraitLangItem,           "div",                   div_trait;
RemTraitLangItem,           "rem",                   rem_trait;
NegTraitLangItem,           "neg",                   neg_trait;
NotTraitLangItem,           "not",                   not_trait;
BitXorTraitLangItem,        "bitxor",                bitxor_trait;
BitAndTraitLangItem,        "bitand",                bitand_trait;
BitOrTraitLangItem,         "bitor",                 bitor_trait;
ShlTraitLangItem,           "shl",                   shl_trait;
ShrTraitLangItem,           "shr",                   shr_trait;

AddAssignTraitLangItem,     "add_assign",            add_assign_trait;
SubAssignTraitLangItem,     "sub_assign",            sub_assign_trait;
MulAssignTraitLangItem,     "mul_assign",            mul_assign_trait;
DivAssignTraitLangItem,     "div_assign",            div_assign_trait;
RemAssignTraitLangItem,     "rem_assign",            rem_assign_trait;
BitXorAssignTraitLangItem,  "bitxor_assign",         bitxor_assign_trait;
BitAndAssignTraitLangItem,  "bitand_assign",         bitand_assign_trait;
BitOrAssignTraitLangItem,   "bitor_assign",          bitor_assign_trait;
ShlAssignTraitLangItem,     "shl_assign",            shl_assign_trait;
ShrAssignTraitLangItem,     "shr_assign",            shr_assign_trait;

IndexTraitLangItem,         "index",                 index_trait;
IndexMutTraitLangItem,      "index_mut",             index_mut_trait;

UnsafeCellTypeLangItem,     "unsafe_cell",           unsafe_cell_type;

DerefTraitLangItem,         "deref",                 deref_trait;
DerefMutTraitLangItem,      "deref_mut",             deref_mut_trait;

FnTraitLangItem,            "fn",                    fn_trait;
FnMutTraitLangItem,         "fn_mut",                fn_mut_trait;
FnOnceTraitLangItem,        "fn_once",               fn_once_trait;

EqTraitLangItem,            "eq",                    eq_trait;
OrdTraitLangItem,           "ord",                   ord_trait;

StrEqFnLangItem,            "str_eq",                str_eq_fn;

PanicFnLangItem,            "panic",                 panic_fn;
PanicBoundsCheckFnLangItem, "panic_bounds_check",    panic_bounds_check_fn;
PanicFmtLangItem,           "panic_fmt",             panic_fmt;

ExchangeMallocFnLangItem,   "exchange_malloc",       exchange_malloc_fn;
BoxFreeFnLangItem,          "box_free",              box_free_fn;
StrDupUniqFnLangItem,       "strdup_uniq",           strdup_uniq_fn;
StartFnLangItem,            "start",                 start_fn;
EhPersonalityLangItem,      "eh_personality",        eh_personality;
EhUnwindResumeLangItem,     "eh_unwind_resume",      eh_unwind_resume;
MSVCTryFilterLangItem,      "msvc_try_filter",       msvc_try_filter;
OwnedBoxLangItem,           "owned_box",             owned_box;
PhantomDataItem,            "phantom_data",          phantom_data;

// Deprecated:
CovariantTypeItem,         "covariant_type",         covariant_type;
ContravariantTypeItem,     "contravariant_type",     contravariant_type;
InvariantTypeItem,         "invariant_type",         invariant_type;
CovariantLifetimeItem,     "covariant_lifetime",     covariant_lifetime;
ContravariantLifetimeItem, "contravariant_lifetime", contravariant_lifetime;
InvariantLifetimeItem,     "invariant_lifetime",     invariant_lifetime;
NoCopyItem,                "no_copy_bound",          no_copy_bound;
NonZeroItem,               "non_zero",               non_zero;
DebugTraitLangItem,        "debug_trait",            debug_trait;
```

The `panic` item corresponds to divide-by-zero and various panic cases with `match`. The `panic_bounds_check` item is for indexing arrays. 

The `begin_unwind` lang item has a predefined symbol name and is sort of a "weak lang item" in the sense that a crate is not required to have it defined to use it, but a final product is required to define it somewhere. Additionally, there are restrictions on crates that use a weak lang item, but do not have it defined.


**from "what is a lang item"**
https://manishearth.github.io/blog/2017/01/11/rust-tidbits-what-is-a-lang-item

*ImplItem*  
"ImplItem" suffix marks implementations on primitive types. For example, `char` has some methods, and someone has to say `impl char` to define them.
But coherence only allows us to impl methods on types defined in our own crate, and char isn’t defined … in any crate, so how do we add methods to it? #[lang = "char"] provides an escape hatch; applying that to impl char will allow you to break the coherence rules and add methods, as is done in the standard library. Since lang items can only be defined once, only a single crate gets the honor of adding methods to char, so we don’t have any of the issues that arise from sidestepping coherence.


*Marker traits*  
`Send` is a lang item because you are allowed to use it in a + bound in a trait object (Box<SomeTrait+Send+Sync>), and the compiler caches it aggressively.

`Sync` is a lang item for the same reasons as Send, but also because the compiler needs to enforce its implementation on types used in statics.

`Copy` is fundamental to classifying values and reasoning about moves/etc, so it needs to be a lang item.

`Sized` is also fundamental to reasoning about which values may exist on the stack. It is also magically included as a bound on generic parameters unless excluded with `?Sized`.

`Unsize` is implemented automatically on types using a specific set of rules (listed in the nomicon). Unlike Send and Sync, this mechanism for autoimplementation is tailored for the use case of Unsize and can’t be reused on user-defined marker traits.

`Drop` is a lang item (core) because the compiler needs to know which types have destructors, and how to call these destructors.

`CoerceUnsized` is a lang item (core) because the compiler is allowed to perform DST coercions (nomicon) when it is implemented.

All of the builtin operators (also Deref and PartialEq/PartialOrd) are lang items because the compiler needs to know what trait to require and call when it comes across such an operation.

`UnsafeCell` is a lang item (core) because it has very special semantics; it prevents certain optimizations. Specifically, Rust is allowed to reorder reads/writes to &mut foo with the assumption that the local variable holding the reference is the only alias allowed to read from or write to the data, and it is allowed to reorder reads from &foo assuming that no other alias writes to it. We tell LLVM that these types are noalias. UnsafeCell<T> turns this optimization off, allowing writes to &UnsafeCell<T> references. This is used in the implementation of interior mutability types like Cell<T>, RefCell<T>, and Mutex<T>.

The `Fn` traits (core) are used in dispatching function calls, and can be specified with special syntax sugar, so they need to be lang items. They also get autoimplemented on closures.

The `str_eq` lang item is outdated. It used to specify how to check the equality of a string value against a literal string pattern in a match (match uses structural equality, not PartialEq::eq), however I believe this behavior is now hardcoded in the compiler.

The panic-related lang items (core) exist because rustc itself inserts panics in a few places. The first one, "panic", is used for integer overflow panics in debug mode, and "panic_bounds_check" is used for out of bounds indexing panics on slices. The last one, "panic_fmt" hooks into a function defined later in libstd.

The "exchange_malloc" and "box_free" (alloc) are for telling the compiler which functions to call in case it needs to do a malloc() or free(). These are used when constructing Box<T> via placement box syntax and when moving out of a deref of a box.

"strdup_uniq" seemed to be used in the past for moving string literals to the heap, but is no longer used.

We’ve already seen the start lang item (std) being used in our minimal example program. This function is basically where you find Rust’s “runtime”: it gets called with a pointer to main and the command line arguments, it sets up the “runtime”, calls main, and tears down anything it needs to. Rust has a C-like minimal runtime, so the actual libstd definition doesn’t do much. But you theoretically could stick a very heavy runtime initialization routine here.

The exception handling lang items (panic_unwind, in multiple platform-specific modules) specify various bits of the exception handling behavior. These hooks are called during various steps of unwinding: eh_personality is called when determining whether or not to stop at a stack frame or unwind up to the next one. eh_unwind_resume is the routine called when the unwinding code wishes to resume unwinding after calling destructors in a landing pad. msvc_try_filter defines some parameter that MSVC needs in its unwinding code. I don’t understand it, and apparently, neither does the person who wrote it.


The "owned_box" (`alloc`) lang item tells the compiler which type is the Box type. Box is special; this lang item is how the compiler finds impls on Box and knows what the type is. Unlike the other primitives, Box doesn’t actually have a type name (like `bool`) that can be used if you’re writing libcore or libstd. This lang item gives Box a type name that can be used to refer to it. (It also defines some, but not all, of the semantics of `Box<T>`)


The "phantom_data" (core) type itself is allowed to have an unused type parameter, and it can be used to help fix the variance and drop behavior of a generic type. More on this in the nomicon.

The "non_zero" lang item (core) marks the NonZero<T> type, a type which is guaranteed to never contain a bit pattern of only zeroes. This is used inside things like Rc<T> and Box<T> – we know that the pointers in these can/should never be null, so they contain a NonZero<*const T>. When used inside an enum like Option<Rc<T>>, the discriminant (the “tag” value that distinguishes between Some and None) is no longer necessary, since we can mark the None case as the case where the bits occupied by NonZero in the Some case are zero. Beware, this optimization also applies to C-like enums that don’t have a variant corresponding to a discriminant value of zero (unless they are #[repr(C)])

There are also a bunch of deprecated lang items there. For example, NoCopy used to be a struct that could be dropped within a type to make it not implement Copy; in the past Copy implementations were automatic like Send and Sync are today. NoCopy was the way to opt out. There also used to be NoSend and NoSync. CovariantType/CovariantLifetime/etc were the predecessors of PhantomData; they could be used to specify variance relations of a type with its type or lifetime parameters, but you can now do this with providing the right PhantomData, e.g. InvariantType<T> is now PhantomData<Cell<T>>. The nomicon has more on variance. I don’t know why these lang items haven’t been removed (they don’t work anymore anyway); the only consumer of them is libcore so “deprecating” them seems unnecessary. It’s probably an oversight.

Interestingly, Iterator and IntoIterator are not lang items, even though they are used in for loops. Instead, the compiler inserts hardcoded calls to ::std::iter::IntoIterator::into_iter and ::std::iter::Iterator::next, and a hardcoded reference to ::std::option::Option (The paths use core in no_std mode). This is probably because the compiler desugars for loops before type resolution is done, so withut this, libcore would not be able to use for loops since the compiler wouldn’t know what calls to insert in place of the loops while compiling.


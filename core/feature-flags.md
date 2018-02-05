# Feature Flags

Unstable Rust features organized by feature gate.
To use an unstable feature, you must use a feature flag, e.g. `#![feature(box_syntax)]`.


## Compiler flags

- `linker_flavor` every rustc target defaults to some linker.
- `profile` allows the generation of code coverage reports.
- `remap_path_prefix` allows replacing prefixes of compiler emitted file paths.


## Language features

- `advanced_slice_patterns` lets you use `..` pattern in slice matching.
- `asm` inline assembly via `asm!` macro.
- `attr_literals` allows other types of literals to be used in attributes. 
- `box_patterns`lets you match on `Box`es.
- `box_syntax` provides unstable `box` keyword.
- `catch_expr` adds support for a `catch` expression.
- `concat_idents`adds a macro for concatenating multiple identifiers into one.
- `conservative_impl_trait` allows a conservative form of abstract return types.
- `const_fn` allows marking free functions and inherent methods as `const`
- `const_indexing` allows the constant evaluation of index operations on constant arrays and repeat expressions.
- `crate_visibility_modifier` allows the `crate` keyword to be used as a visibility modifier synonymous to `pub(crate)`
- `doc_cfg` allows an API be documented as only available in some specific platforms.
- `doc_masked` allows a crate to exclude types from a given crate from appearing in lists of trait implementations.
- `fn_must_use` allows functions and methods to be annotated with `#[must_use]`.
- `generators` define generator or coroutine literals. `yield` keyword.
- `global_allocator` change the allocator
- `global_asm` macro `global_asm!` allows writing arbitrary assembly outside the scope of a function body.
- `i128_type` adds support for 128 bit signed and unsigned integer types.
- `inclusive_range_syntax` adds syntax for inclusive ranges: `0 ..= 10`
- `lang_items` pluggable operations
- `link_args` customize linking
- `macro_vis_matcher` visibility qualifier for fragments in macro definitions.
- `match_beginning_vert` add a '|' to the beginning of a match arm
- `match_default_bindings` improves pattern-matching on references in `match`.
- `non_ascii_idents`  adds support for non-ASCII identifiers.
- `non_exhaustive` use `#[non_exhaustive]` attribute on structs and enums.
- `on_unimplemented` provides `#[rustc_on_unimplemented]` attribute.
- `optin_builtin_traits` allows defining of auto traits.
- `plugin`compiler plugins
- `plugin_registrar` compiler plugins
- `proc_macro` procedural macros features
- `slice_patterns` matching against a slice or array, with `&` feature.
- `trace_macros` trace the expansion of macros in the code.
- `unboxed_closures` allows writing functions using the "rust-call" ABI.
- `unsized_tuple_coercion` RFC for unsized tuple coercion.
- `used` prevent optimization of `static` variables.

```
abi_msp430_interrupt
abi_ptx
abi_thiscall
abi_sysv64
abi_unadjusted
abi_vectorcall
abi_x86_interrupt
allocator_internals
allow_fail
allow_internal_unsafe
allow_internal_unstable
associated_type_defaults
cfg_target_feature
cfg_target_has_atomic
cfg_target_thread_local
cfg_target_vendor
clone_closures
compiler_builtins
copy_closures
custom_attribute
custom_derive
decl_macro
default_type_parameter_fallback
dotdoteq_in_patterns
dropck_eyepatch
dropck_parametricity
exclusive_range_pattern
fundamental
generic_param_attrs
intrinsics
link_cfg
link_llvm_intrinsics
linkage
log_syntax
macro_reexport
main
naked_functions
needs_allocator
needs_panic_runtime
never_type
no_core
no_debug
omit_gdb_pretty_printer_section
overlapping_marker_traits
panic_runtime
placement_in_syntax
platform_intrinsics
prelude_import
quote
profiler_runtime
repr128
repr_align
repr_simd
rustc_attrs
rustc_const_unstable
rustc_diagnostic_macros
sanitizer_runtime
simd
simd_ffi
specialization
staged_api
start
static_nobundle
stmt_expr_attributes
structural_match
target_feature
thread_local
type_ascription
underscore_lifetimes
untagged_unions
unwind_attributes
use_extern_macros
```


## Library features

- `fn_traits` allows impl of Fn traits for creating custom closure-like types.
- `slice_rsplit` enables rsplit and rsplit_mut methods on slices.
- `splice` allows replacing a range of values in `String` with another range.
- `string_retain` allows to retains only the characters specified by predicate.
- `try_trait` new trait `Try` for extending `?` operator to other types.

```
align_offset
alloc
alloc_jemalloc
alloc_system
allocator_api
ascii_ctype
box_heap
c_void_variant
char_error_internals
coerce_unsized
collection_placement
collections
collections_range
compiler_builtins_lib
concat_idents_macro
core_char_ext
core_float
core_intrinsics
core_panic
core_private_bignum
core_private_diy_float
core_slice_ext
core_str_ext
dec2flt
decode_utf8
derive_clone_copy
derive_eq
drain_filter
duration_from_micros
entry_and_modify
entry_or_default
error_type_id
exact_size_is_empty
fd
fd_read
fixed_size_array
flt2dec
fmt_flags_align
fmt_internals
fnbox
from_utf8_error_as_bytes
fused
future_atomic_orderings
generator_trait
get_type_id
heap_api
hint_core_should_pause
i128
inclusive_range
int_error_internals
integer_atomics
io
io_error_internals
ip
ip_constructors
iter_rfind
iter_rfold
iterator_step_by
libstd_io_internals
libstd_sys_internals
libstd_thread_internals
linked_list_extras
lookup_host
map_entry_replace
mpsc_select
n16
never_type_impls
nonzero
offset_to
once_poison
option_ref_mut_cloned
panic_abort
panic_col
panic_unwind
pattern
placement_in
placement_new_protocol
pointer_methods
print_internals
proc_macro
proc_macro_internals
profiler_runtime_lib
rand
range_contains
raw
rc_downcast
read_initializer
refcell_replace_swap
rt
rustc_private
sanitizer_runtime_lib
set_stdio
shared
sip_hash_13
slice_concat_ext
slice_get_slice
slice_rotate
sort_internals
step_trait
str_escape
str_internals
swap_nonoverlapping
swap_with_slice
take_set_limit
test
thread_local_internals
thread_local_state
toowned_clone_into
trusted_len
try_from
unicode
unique
unreachable
unsize
update_panic_count
vec_remove_item
vec_resize_default
windows_c
windows_handle
windows_net
windows_stdio
```
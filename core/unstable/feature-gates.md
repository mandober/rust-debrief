# Feature Flags

Unstable Rust features organized by feature gate.
To use an unstable feature, you must use a feature flag,
e.g. `#![feature(box_syntax)]`


## Compiler flags
- `linker_flavor`
- `profile`
- `remap_path_prefix`


## Language features
- `abi_msp430_interrupt`
- `abi_ptx`
- `abi_sysv64`
- `abi_thiscall`
- `abi_unadjusted`
- `abi_vectorcall`
- `abi_x86_interrupt`
- `advanced_slice_patterns`
- `allocator_internals`
- `allow_fail` test attribute
- `allow_internal_unsafe`
- `allow_internal_unstable`
- `asm`
- `associated_type_defaults`
- `attr_literals`
- `box_patterns`
- `box_syntax`
- `catch_expr`
- `cfg_target_feature`
- `cfg_target_has_atomic`
- `cfg_target_thread_local`
- `cfg_target_vendor`
- `clone_closures`
- `compiler_builtins`
- `concat_idents`
- `conservative_impl_trait`
- `const_fn`
- `const_indexing`
- `copy_closures`
- `custom_attribute`
- `custom_derive`
- `decl_macro`
- `default_type_parameter_fallback`
- `doc_cfg`
- `doc_masked`
- `dotdoteq_in_patterns`
- `dropck_eyepatch`
- `dropck_parametricity`
- `exclusive_range_pattern`
- `fn_must_use`
- `fundamental`
- `generators`
- `generic_param_attrs`
- `global_allocator`
- `global_asm`
- `i128_type`
- `inclusive_range_syntax`
- `intrinsics`
- `lang_items`
- `link_args`
- `link_cfg`
- `link_llvm_intrinsics`
- `linkage`
- `log_syntax`
- `macro_reexport`
- `macro_vis_matcher`
- `main`
- `match_beginning_vert`
- `match_default_bindings`
- `naked_functions`
- `needs_allocator`
- `needs_panic_runtime`
- `never_type`
- `no_core`
- `no_debug`
- `non_ascii_idents`
- `omit_gdb_pretty_printer_section`
- `on_unimplemented`
- `optin_builtin_traits`
- `overlapping_marker_traits`
- `panic_runtime`
- `placement_in_syntax`
- `platform_intrinsics`
- `plugin`
- `plugin_registrar`
- `prelude_import`
- `proc_macro`
- `profiler_runtime`
- `quote`
- `repr128`
- `repr_align`
- `repr_simd`
- `rustc_attrs`
- `rustc_const_unstable`
- `rustc_diagnostic_macros`
- `sanitizer_runtime`
- `simd`
- `simd_ffi`
- `slice_patterns`
- `specialization`
- `staged_api`
- `start`
- `static_nobundle`
- `stmt_expr_attributes`
- `structural_match`
- `target_feature`
- `thread_local`
- `trace_macros`
- `type_ascription`
- `unboxed_closures`
- `underscore_lifetimes`
- `unsized_tuple_coercion`
- `untagged_unions`
- `unwind_attributes`
- `use_extern_macros`
- `used`

## Library Features
- `align_offset`
- `alloc`
- `alloc_jemalloc`
- `alloc_system`
- `allocator_api`
- `ascii_ctype`
- `box_heap`
- `c_void_variant`
- `char_error_internals`
- `coerce_unsized`
- `collection_placement`
- `collections`
- `collections_range`
- `compiler_builtins_lib`
- `concat_idents_macro`
- `core_char_ext`
- `core_float`
- `core_intrinsics`
- `core_panic`
- `core_private_bignum`
- `core_private_diy_float`
- `core_slice_ext`
- `core_str_ext`
- `dec2flt`
- `decode_utf8`
- `derive_clone_copy`
- `derive_eq`
- `drain_filter`
- `duration_from_micros`
- `entry_and_modify`
- `entry_or_default`
- `error_type_id`
- `exact_size_is_empty`
- `fd`
- `fd_read`
- `fixed_size_array`
- `flt2dec`
- `fmt_flags_align`
- `fmt_internals`
- `fn_traits`
- `fnbox`
- `from_utf8_error_as_bytes`
- `fused`
- `future_atomic_orderings`
- `generator_trait`
- `get_type_id`
- `heap_api`
- `hint_core_should_pause`
- `i128`
- `inclusive_range`
- `int_error_internals`
- `integer_atomics`
- `io`
- `io_error_internals`
- `ip`
- `ip_constructors`
- `iter_rfind`
- `iter_rfold`
- `iterator_step_by`
- `libstd_io_internals`
- `libstd_sys_internals`
- `libstd_thread_internals`
- `linked_list_extras`
- `lookup_host`
- `map_entry_replace`
- `mpsc_select`
- `n16`
- `never_type_impls`
- `nonzero`
- `offset_to`
- `once_poison`
- `option_ref_mut_cloned`
- `panic_abort`
- `panic_col`
- `panic_unwind`
- `pattern`
- `placement_in`
- `placement_new_protocol`
- `pointer_methods`
- `print_internals`
- `proc_macro`
- `proc_macro_internals`
- `profiler_runtime_lib`
- `rand`
- `range_contains`
- `raw`
- `rc_downcast`
- `read_initializer`
- `refcell_replace_swap`
- `rt`
- `rustc_private`
- `sanitizer_runtime_lib`
- `set_stdio`
- `shared`
- `sip_hash_13`
- `slice_concat_ext`
- `slice_get_slice`
- `slice_rotate`
- `slice_rsplit`
- `sort_internals`
- `splice`
- `step_trait`
- `str_escape`
- `str_internals`
- `string_retain`
- `swap_nonoverlapping`
- `swap_with_slice`
- `take_set_limit`
- `test`
- `thread_local_internals`
- `thread_local_state`
- `toowned_clone_into`
- `trusted_len`
- `try_from`
- `try_trait`
- `unicode`
- `unique`
- `unreachable`
- `unsize`
- `update_panic_count`
- `vec_remove_item`
- `vec_resize_default`
- `windows_c`
- `windows_handle`
- `windows_net`
- `windows_stdio`
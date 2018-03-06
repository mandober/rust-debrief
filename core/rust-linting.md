rustc -W help

Available lint options:
-W <foo>           Warn about <foo>
-A <foo>           Allow <foo>
-D <foo>           Deny <foo>
-F <foo>           Forbid <foo> (deny <foo> and all attempts to override)

Lint checks provided by rustc:

```

                                         name  default  meaning
                                         ----  -------  -------
                         anonymous-parameters  allow    detects anonymous parameters
                                 box-pointers  allow    use of owned (Box type) heap memory
                           fat-ptr-transmutes  allow    detects transmutes of fat pointers
                 missing-copy-implementations  allow    detects potentially-forgotten implementations of `Copy`
                missing-debug-implementations  allow    detects missing implementations of fmt::Debug
                                 missing-docs  allow    detects missing documentation for public members
                                trivial-casts  allow    detects trivial casts which could be removed
                        trivial-numeric-casts  allow    detects trivial casts of numeric types which could be removed
                                  unsafe-code  allow    usage of `unsafe` code
                            unstable-features  allow    enabling unstable features (deprecated. do not use)
                         unused-extern-crates  allow    extern crates that are never used
                         unused-import-braces  allow    unnecessary braces around an imported item
                        unused-qualifications  allow    detects unnecessarily qualified names
                               unused-results  allow    unused result of an expression in a statement
                     variant-size-differences  allow    detects enums with widely varying variant sizes
                                    const-err  warn     constant evaluation detected erroneous expression
                                    dead-code  warn     detect unusedunexported items
                                   deprecated  warn     detects use of deprecated items
                              deprecated-attr  warn     detects use of deprecated attributes
       illegal-floating-point-literal-pattern  warn     floating-point literals cannot be used in patterns
                              improper-ctypes  warn     proper use of libc types in foreign modules
                late-bound-lifetime-arguments  warn     detects generic lifetime arguments in path segments with late bound lifetime parameters
                         non-camel-case-types  warn     typesvariantstraits and type parameters should have camel case names
                 non-shorthand-field-patterns  warn     using `Struct { x: x }` instead of `Struct { x }`
                               non-snake-case  warn     variablesmethodsfunctionslifetime parameters and modules should have snake case names
                       non-upper-case-globals  warn     static constants should have uppercase identifiers
                      no-mangle-generic-items  warn     generic items must be mangled
                         overflowing-literals  warn     literal out of range for its type
                              path-statements  warn     path statements with no effect
                 patterns-in-fns-without-body  warn     patterns in functions without body were erroneously allowed
                            plugin-as-library  warn     compiler plugin used as ordinary library in non-plugin crate
                            private-in-public  warn     detect private items in public interfaces not caught by the old implementation
                        private-no-mangle-fns  warn     functions marked #[no_mangle] should be exported
                    private-no-mangle-statics  warn     statics marked #[no_mangle] should be exported
                    renamed-and-removed-lints  warn     lints that have been renamed or removed
                              stable-features  warn     stable features found in #[feature] directive
                      unconditional-recursion  warn     functions that cannot return without calling themselves
                      unions-with-drop-fields  warn     use of unions that contain fields with possibly non-trivial drop code
                                unknown-lints  warn     unrecognized lint attribute
                             unreachable-code  warn     detects unreachable code paths
                         unreachable-patterns  warn     detects unreachable patterns
                            unused-allocation  warn     detects unnecessary allocations that can be eliminated
                           unused-assignments  warn     detect assignments that will never be read
                            unused-attributes  warn     detects attributes that were not used by the compiler
                           unused-comparisons  warn     comparisons made useless by limits of the types involved
                           unused-doc-comment  warn     detects doc comments that aren't used by rustdoc
                              unused-features  warn     unused or unknown features found in crate-level #[feature] directives
                               unused-imports  warn     imports that are never used
                                unused-macros  warn     detects macros that were not used
                              unused-must-use  warn     unused result of a type flagged as #[must_use]
                                   unused-mut  warn     detect mut variables which don't need to be mutable
                                unused-parens  warn     if, match, while, return do not need parentheses
                                unused-unsafe  warn     unnecessary use of an `unsafe` block
                             unused-variables  warn     detect variables which are not used in any way
                                     warnings  warn     mass-change the level for lints which produce warnings
                                   while-true  warn     suggest using `loop { }` instead of `while true { }`
                          exceeding-bitshifts  deny     shift exceeds the type's number of bits
                    extra-requirement-in-impl  deny     detects extra requirements in impls that were erroneously allowed
                   invalid-type-param-default  deny     type parameter default erroneously allowed in invalid location
                legacy-constructor-visibility  deny     detects use of struct constructors that would be invisible with new visibility rules
                   legacy-directory-ownership  deny     non-inlinenon-`#[path]` modules (e.g. `mod foo;`) were erroneously allowed in some files not named `mod.rs`
                               legacy-imports  deny     detects names that resolve to ambiguous glob imports with RFC 1560
                   missing-fragment-specifier  deny     detects missing fragment specifiers in unused `macro_rules!` patterns
                           mutable-transmutes  deny     mutating transmuted &mut T from &T may cause undefined behavior
                        no-mangle-const-items  deny     const items will not have their symbols exported
    parenthesized-params-in-types-and-modules  deny     detects parenthesized generic parameters in type and module names
              pub-use-of-private-extern-crate  deny     detect public reexports of private extern crates
              resolve-trait-on-defaulted-unit  deny     attempt to resolve a trait on an expression whose type cannot be inferred but which currently defaults to ()
                          safe-extern-statics  deny     safe access to extern statics was erroneously allowed
                          unknown-crate-types  deny     unknown crate type found in #[crate_type] directive

```


Lint groups provided by rustc:

warnings
- all
- built-in
- lints

bad-style
- non-camel-case-types
- non-snake-case
- non-upper-case-globals

future-incompatible
- private-in-public
- pub-use-of-private-extern-crate
- patterns-in-fns-without-body
- safe-extern-statics
- invalid-type-param-default
- extra-requirement-in-impl
- legacy-directory-ownership
- legacy-imports
- legacy-constructor-visibility
- resolve-trait-on-defaulted-unit
- missing-fragment-specifier
- illegal-floating-point-literal-pattern
- anonymous-parameters
- parenthesized-params-in-types-and-modules
- late-bound-lifetime-arguments

unused
- unused-imports
- unused-variables
- unused-assignments
- dead-code
- unused-mut
- unreachable-code
- unreachable-patterns
- unused-must-use
- unused-unsafe
- path-statements
- unused-attributes
- unused-macros


Compiler plugins can provide additional lints and lint groups. 
To see a listing of thesere-run `rustc -W help` with a crate filename.


anonymous-parameters
box-pointers
fat-ptr-transmutes
missing-copy-implementations
missing-debug-implementations
missing-docs
trivial-casts
trivial-numeric-casts
unsafe-code
unstable-features
unused-extern-crates
unused-import-braces
unused-qualifications
unused-results
variant-size-differences
const-err
dead-code
deprecated
deprecated-attr
illegal-floating-point-literal-pattern
improper-ctypes
late-bound-lifetime-arguments
non-camel-case-types
non-shorthand-field-patterns
non-snake-case
non-upper-case-globals
no-mangle-generic-items
overflowing-literals
path-statements
patterns-in-fns-without-body
plugin-as-library
private-in-public
private-no-mangle-fns
private-no-mangle-statics
renamed-and-removed-lints
stable-features
unconditional-recursion
unions-with-drop-fields
unknown-lints
unreachable-code
unreachable-patterns
unused-allocation
unused-assignments
unused-attributes
unused-comparisons
unused-doc-comment
unused-features
unused-imports
unused-macros
unused-must-use
unused-mut
unused-parens
unused-unsafe
unused-variables
warnings
while-true
exceeding-bitshifts
extra-requirement-in-impl
invalid-type-param-default
legacy-constructor-visibility
legacy-directory-ownership
legacy-imports
missing-fragment-specifier
mutable-transmutes
no-mangle-const-items
parenthesized-params-in-types-and-modules
pub-use-of-private-extern-crate
resolve-trait-on-defaulted-unit
safe-extern-statics
unknown-crate-types


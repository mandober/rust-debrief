# Lints

List linting attributes and default settings: `rustc -W help`

Linting actions:
- Set lint allowed: `-A, --allow OPT`
- Set lint warnings: `-W, --warn OPT`
- Set lint denied:`-D, --deny OPT`
- Set lint forbidden:`-F, --forbid OPT` (deny and disallow overrides)

Compiler plugins can provide additional lints. Print the list of lints that a crate contributes: `rustc -W help CRATE_NAME`.

```shell
# List lints and default settings
rustc -W help

# List lints provided by a CRATE
rustc -W help CRATE

# allow non-snake-case
rustc --allow non-snake-case ...

# Set the most restrictive lint level.
# More restrictive lints are capped at this level
rustc --cap-lints LEVEL
```

## List of lints provided by rustc

- `anonymous-parameters`
- `box-pointers`
- `coerce-never`
- `const-err`
- `dead-code`
- `deprecated`
- `exceeding-bitshifts`
- `illegal-floating-point-literal-pattern`
- `improper-ctypes`
- `incoherent-fundamental-impls`
- `invalid-type-param-default`
- `late-bound-lifetime-arguments`
- `legacy-constructor-visibility`
- `legacy-directory-ownership`
- `legacy-imports`
- `missing-copy-implementations`
- `missing-debug-implementations`
- `missing-docs`
- `missing-fragment-specifier`
- `mutable-transmutes`
- `no-mangle-const-items`
- `no-mangle-generic-items`
- `non-camel-case-types`
- `non-shorthand-field-patterns`
- `non-snake-case`
- `non-upper-case-globals`
- `overflowing-literals`
- `parenthesized-params-in-types-and-modules`
- `path-statements`
- `patterns-in-fns-without-body`
- `plugin-as-library`
- `private-in-public`
- `private-no-mangle-fns`
- `private-no-mangle-statics`
- `pub-use-of-private-extern-crate`
- `renamed-and-removed-lints`
- `resolve-trait-on-defaulted-unit`
- `safe-extern-statics`
- `safe-packed-borrows`
- `single-use-lifetime`
- `stable-features`
- `trivial-casts`
- `trivial-numeric-casts`
- `tyvar-behind-raw-pointer`
- `unconditional-recursion`
- `unions-with-drop-fields`
- `unknown-crate-types`
- `unknown-lints`
- `unreachable-code`
- `unreachable-patterns`
- `unreachable-pub`
- `unsafe-code`
- `unstable-features`
- `unused-allocation`
- `unused-assignments`
- `unused-attributes`
- `unused-comparisons`
- `unused-doc-comment`
- `unused-extern-crates`
- `unused-features`
- `unused-import-braces`
- `unused-imports`
- `unused-macros`
- `unused-must-use`
- `unused-mut`
- `unused-parens`
- `unused-qualifications`
- `unused-results`
- `unused-unsafe`
- `unused-variables`
- `variant-size-differences`
- `while-true`



## Lint groups provided by rustc

- **warnings**: all lints that are set to issue warnings
- **bad-style**:
  - `non-camel-case-types`
  - `non-snake-case`
  - `non-upper-case-globals`
- **future-incompatible**:
  - `private-in-public`
  - `pub-use-of-private-extern-crate`
  - `patterns-in-fns-without-body`
  - `safe-extern-statics`
  - `invalid-type-param-default`
  - `legacy-directory-ownership`
  - `legacy-imports`
  - `legacy-constructor-visibility`
  - `resolve-trait-on-defaulted-unit`
  - `missing-fragment-specifier`
  - `illegal-floating-point-literal-pattern`
  - `anonymous-parameters`
  - `parenthesized-params-in-types-and-modules`
  - `late-bound-lifetime-arguments`
  - `safe-packed-borrows`
  - `incoherent-fundamental-impls`
  - `coerce-never`
  - `tyvar-behind-raw-pointer`
- **unused**:
  - `unused-imports`
  - `unused-variables`
  - `unused-assignments`
  - `unused-mut`
  - `unused-must-use`
  - `unused-unsafe`
  - `unused-attributes`
  - `unused-macros`
  - `unused-allocation`
  - `unused-doc-comment`
  - `unused-extern-crates`
  - `unused-features`
  - `unused-parens`
  - `unreachable-code`
  - `unreachable-patterns`
  - `dead-code`
  - `path-statements`


## Lint description and defaults
- `anonymous-parameters` detects anonymous parameters `A`
- `box-pointers` use of owned (Box type) heap memory `A`
- `fat-ptr-transmutes` detects transmutes of fat pointers `A`
- `missing-copy-implementations` detects potentially-forgotten implementations of `Copy` `A`
- `missing-debug-implementations` detects missing implementations of `fmt::Debug` `A`
- missing-docs: detects missing documentation for public members `A`
- trivial-casts: detects trivial casts which could be removed `A`
- trivial-numeric-casts: detects trivial casts of numeric types which could be removed `A`
- unsafe-code: usage of `unsafe` code `A`
- unused-extern-crates: extern crates that are never used `A`
- unused-import-braces: unnecessary braces around an imported item `A`
- unused-qualifications: detects unnecessarily qualified names `A`
- unused-results: unused result of an expression in a statement `A`
- variant-size-differences: detects enums with widely varying variant sizes `A`
* unstable-features: enabling unstable features (deprecated. do not use) `A`


```
                         anonymous-parameters  allow    detects anonymous parameters                                                                                     
                                 box-pointers  allow    use of owned (Box type) heap memory                                                                              
                 missing-copy-implementations  allow    detects potentially-forgotten implementations of `Copy`                                                          
                missing-debug-implementations  allow    detects missing implementations of fmt::Debug                                                                    
                                 missing-docs  allow    detects missing documentation for public members                                                                 
                          single-use-lifetime  allow    detects single use lifetimes                                                                                     
                                trivial-casts  allow    detects trivial casts which could be removed                                                                     
                        trivial-numeric-casts  allow    detects trivial casts of numeric types which could be removed                                                    
                              unreachable-pub  allow    `pub` items not reachable from crate root                                                                        
                                  unsafe-code  allow    usage of `unsafe` code                                                                                           
                            unstable-features  allow    enabling unstable features (deprecated. do not use)                                                              
                         unused-extern-crates  allow    extern crates that are never used                                                                                
                         unused-import-braces  allow    unnecessary braces around an imported item                                                                       
                        unused-qualifications  allow    detects unnecessarily qualified names                                                                            
                               unused-results  allow    unused result of an expression in a statement                                                                    
                     variant-size-differences  allow    detects enums with widely varying variant sizes

                                    const-err  warn     constant evaluation detected erroneous expression                                                                
                                    dead-code  warn     detect unused, unexported items                                                                                  
                                   deprecated  warn     detects use of deprecated items                                                                                  
       illegal-floating-point-literal-pattern  warn     floating-point literals cannot be used in patterns                                                               
                              improper-ctypes  warn     proper use of libc types in foreign modules                                                                      
                 incoherent-fundamental-impls  warn     potentially-conflicting impls were erroneously allowed                                                           
                late-bound-lifetime-arguments  warn     detects generic lifetime arguments in path segments with late bound lifetime parameters                          
                         non-camel-case-types  warn     types, variants, traits and type parameters should have camel case names                                         
                 non-shorthand-field-patterns  warn     using `Struct { x: x }` instead of `Struct { x }` in a pattern                                                   
                               non-snake-case  warn     variables, methods, functions, lifetime parameters and modules should have snake case names                      
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
                          safe-packed-borrows  warn     safe borrows of fields of packed structs were was erroneously allowed                                            
                              stable-features  warn     stable features found in #[feature] directive                                                                    
                     tyvar-behind-raw-pointer  warn     raw pointer to an inference variable                                                                             
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
                                unused-parens  warn     `if`, `match`, `while` and `return` do not need parentheses                                                      
                                unused-unsafe  warn     unnecessary use of an `unsafe` block                                                                             
                             unused-variables  warn     detect variables which are not used in any way                                                                   
                                     warnings  warn     mass-change the level for lints which produce warnings                                                           
                                   while-true  warn     suggest using `loop { }` instead of `while true { }`                                                             

                                 coerce-never  deny     detect coercion to !                                                                                             
                          exceeding-bitshifts  deny     shift exceeds the type's number of bits                                                                          
                   invalid-type-param-default  deny     type parameter default erroneously allowed in invalid location                                                   
                legacy-constructor-visibility  deny     detects use of struct constructors that would be invisible with new visibility rules                             
                   legacy-directory-ownership  deny     non-inline, non-`#[path]` modules (e.g. `mod foo;`) were erroneously allowed in some files not named `mod.rs`    
                               legacy-imports  deny     detects names that resolve to ambiguous glob imports with RFC 1560                                               
                   missing-fragment-specifier  deny     detects missing fragment specifiers in unused `macro_rules!` patterns                                            
                           mutable-transmutes  deny     mutating transmuted &mut T from &T may cause undefined behavior                                                  
                        no-mangle-const-items  deny     const items will not have their symbols exported                                                                 
    parenthesized-params-in-types-and-modules  deny     detects parenthesized generic parameters in type and module names                                                
              pub-use-of-private-extern-crate  deny     detect public re-exports of private extern crates                                                                
              resolve-trait-on-defaulted-unit  deny     attempt to resolve a trait on an expression whose type cannot be inferred but which currently defaults to ()     
                          safe-extern-statics  deny     safe access to extern statics was erroneously allowed                                                            
                          unknown-crate-types  deny     unknown crate type found in #[crate_type] directive                      
```

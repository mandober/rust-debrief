# The Compiler

- Stages: Parsing, Resolution, HIR, Type-checking, MIR, LLVM, Linking
- The compiler is written in Rust
- Crate is the smallest compiling unit
- each compilation processes a single crate with source code and produces a single crate in binary form (executable or library).
- Rust has a phase distinction between compile-time and run-time; Statically interpreted semantic rules govern the success of compilation. Dynamically interpreted semantic rules govern the program's run-time behavior.

Source is parsed into AST, 
then simplified into HIR (High-level Intermediate Representation),
then lowered into MIR (Mid-level Intermediate Representation), 
then translated into LLVM IR
which generates the Machine Code.


## General stages of compilation:

1. **Parse** the source code and produces the AST.
2. **Resolution**  
   Processing the AST recursively, resolving paths, expanding macros, processing configuration directives.
3. **HIR**  
   Convert the AST into the HIR (high-level IR). The HIR is a lightly de-sugared variant of the AST. It is more processed than the AST and more suitable for the analyses that follow. It is not required to match the syntax of the Rust language.
3. **Type-checking**  
   Performing type checking: assigning types to every HIR expression, resolving type-dependent paths and associated type references. Type checking creates side-tables that include the types of expressions, the way to resolve methods, etc. Followed by other analyses, such as privacy checking.
4. **MIR**  
   Lowering the HIR into MIR (middle IR), a very de-sugared version of Rust, well suited to the `borrowck`, but also certain high-level optimizations. 
5. **LLVM**  
   Translation to LLVM and LLVM optimizations: producing LLVM IR from MIR, LLVM run optimizations that produce object files.
6. **Linking**  
   Linking the object files together.


## Links

* [An informal guide to reading and working on the rustc compiler](https://github.com/rust-lang/rust/tree/master/src/librustc)
* [Internals Forum](https://internals.rust-lang.org/)
* [Rust Forge](https://forge.rust-lang.org/)
* [Rust Issues](https://github.com/rust-lang/rust/issues)
* [LLVM](https://llvm.org/)
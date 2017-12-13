# The Compiler

- Stages: Parsing > Resolution > HIR > Type-checking > MIR > LLVM > Linking
- The compiler is written in Rust
- Crate is the smallest compiling unit


## General stages of compilation:

1. **Parsing**  
   Processing the `.rs` files and produces the AST.
2. **Resolution**  
   Processing the AST recursively, resolving paths, expanding macros, processing configuration directives.
3. **HIR**  
   Convert the AST into the HIR (high-level IR). The HIR is a lightly desugared variant of the AST. It is more processed than the AST and more suitable for the analyses that follow. It is not required to match the syntax of the Rust language.
3. **Type-checking**  
   Performing type checking: assigning types to every HIR expression, resolving type-dependen paths and associated type references. Type checking creates side-tables that include the types of expressions, the way to resolve methods, etc. Followed by other analyses, such as privacy checking.
4. **MIR**  
   Lowering the HIR into MIR (middle IR), a very desugared version of Rust, well suited to the borrowck, but also certain high-level optimizations. 
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
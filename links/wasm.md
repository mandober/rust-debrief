# WebAssembly

- [WebAssembly.org][wado]
  WebAssembly or wasm is a portable, size and load-time efficient low-level 
  programming language for in-browser client-side scripting, designed to be 
  faster than JS.
- [asm.js][asmj]
  Optimizable, low-level, subset of JS as intermediate programming language 
  designed to allow software written in languages such as Rust to be run as web 
  applications while maintaining performance characteristics considerably better 
  than standard JS.
- [Emscripten][emsc]
  Emscripten is a source-to-source compiler that runs as a backend to the LLVM 
  compiler and produces a subset of JavaScript as asm.js or WebAssembly.   
  [git repo](https://github.com/kripken/emscripten.git)
- [Clang][clng]
  The goal of the Clang project is to create a new C based language front-end: 
  C, C++, Objective C/C++, OpenCL C and others for the LLVM compiler.
- [LLVM][llvm]
  Collection of modular and reusable compiler and toolchain technologies.
- [binaryen](https://github.com/WebAssembly/binaryen)
  Binaryen is a compiler and toolchain infrastructure library for WebAssembly, written in C++. It aims to make compiling to WebAssembly easy, fast, and effective.
- [awesome-wasm](https://github.com/mbasso/awesome-wasm)
- [wasm weekly](http://wasmweekly.news/)



## Rust and wasm
- [Rust and WebAssembly][rawg]
  Coordinating early work on using Rust and WebAssembly together.
- [Rust and WebAssembly: Guide][rawb]
  Small guidebook describing how to use Rust and WebAssembly.
- [Hello, Rust!][hrwa]
  Rust for the Web: WebAssembly setup and examples.
- [The Rusty Web][trws]
  Demonstration of integrating Rust into a web app. Benchmark of wasm vs JS. 
  https://github.com/davidMcneil/the-rusty-web
- [The Rusty Web: Guide][trwb]
  Guidebook for using Rust to target the web.
- [mir2wasm](https://github.com/brson/mir2wasm/)
  An experimental compiler from Rust to WebAssembly, based on rustc and MIR.
- [wasm-bindgen](https://github.com/alexcrichton/wasm-bindgen)
- [wasm-bindgen: design](https://github.com/alexcrichton/wasm-bindgen/blob/master/DESIGN.md)




[wado]: http://webassembly.org/
[asmj]: http://asmjs.org/
[emsc]: http://kripken.github.io/emscripten-site/
[llvm]: http://llvm.org/
[clng]: http://clang.llvm.org/
[rawg]: https://github.com/rust-lang-nursery/rust-wasm
[rawb]: https://rust-lang-nursery.github.io/rust-wasm/
[hrwa]: https://www.hellorust.com/
[trwb]: https://davidmcneil.gitbooks.io/the-rusty-web/content/
[trws]: https://davidmcneil.github.io/the-rusty-web/



## crates
- https://github.com/koute/cargo-web
- stdweb, a "standard library for the client-side web".
  https://github.com/koute/stdweb/
- Yew, a framework for client-side web apps.
  https://github.com/DenisKolodin/yew

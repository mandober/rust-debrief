# WebAssembly

WebAssembly (wasm, WA) is a web standard that defines a binary format and a corresponding assembly-like text format for executable code in Web pages.

It is meant to enable executing code nearly as fast as running native machine code.

It is executed in a sandbox in the web browser after a formal verification step.

Programs can be compiled from high-level languages into wasm modules and loaded as libraries from within JavaScript applets.

WebAssembly is a portable stack machine which is designed to be faster to parse than JavaScript, as well as faster to execute, and to enable very compact code representation.

For backward compatibility, wasm can be compiled into asm.js and executed on incompatible browsers this way. 

Emscripten can compile to wasm using LLVM in the backend. It supports compilation from Rust, C and C++ into wasm.

After the minimum viable product release, there are plans to support garbage collection which would make WebAssembly a compilation target for garbage collected programming languages like Java and C#.

In March 2017, a WebAssembly binary format was defined, along with a human-readable linear assembly bytecode format that resembles traditional assembly languages.


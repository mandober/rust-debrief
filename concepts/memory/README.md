# Rust and memory


* Variable
  - Free variable
  - Bound variables
  * Variable attributes
    - Symbolic name
    - Identifier
    - Data type
    - Scope
    - Visibility
    - Lifetime
* Variable types
  - Constant
  - Static
  - Local variable
  - Temporary variable
* Value
  - Object
  - Literal
  - Value data type
  - Move vs Copy values
* Pointer
  - Pointer data type
* Aliasing


* Declaration
  - Forward declarations
  - Definition
  - Explicit declaration
  - Implicit declaration
* Initialization
  - Re-initialization
  - Deinitialization
* Binding
  - Assignment
  - Mutability
  - Rebinding
  - Static binding
  - Dynamic binding
  - Early binding
  - Late binding
  - Symbol table
* Ownership
  - RAII
  - Move Semantics
  - Copy Semantics
  - Drop
* Borrowing
  - Mutability
  - Shared borrowing
  - Exclusive borrowing
  - Consuming ownership
  - Moving


Declaration
- declaration
- definition
- forward declaration
- explicit
- implicit
- entity
- identifier
  - identifier kind
  - identifier properties
    - descriptive
    - prescriptive

Binding
- initialization
- re-initialization
- deinitialization
- binding
  - early binding
  - late binding




`main` is the fn: when it returns, the program is also over; it only remains to return the error (status) code to the caller (shell). So the extent of main fn is the extent of the whole program; the extent of main fn is the extent of the program, practically the `'static` lifetime. All construction done strictly within main fn itself, which is using the stack will have a stable address. main fn is in the main thread, which, like any thread, has its own stack, but unlike user-created threads, the main's stack size cannot be specified (it's already running). Just before the main's stack base address is the address of the caller (shell) that ran the program. 


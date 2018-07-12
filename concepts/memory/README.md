# Rust and Memory


`main` is the fn: when it returns, the program is also over; it only remains to return the error (status) code to the caller (shell). So the extent of main fn is the extent of the whole program; the extent of main fn is the extent of the program, practically the `'static` lifetime. All construction done strictly within main fn itself, which is using the stack will have a stable address. main fn is in the main thread, which, like any thread, has its own stack, but unlike user-created threads, the main's stack size cannot be specified (it's already running). Just before the main's stack base address is the address of the caller (shell) that ran the program. 







- [Value](value.md)
- [Pointers](pointers.md)


- Variables
- Declaration
- Definition
- Forward declarations
- Identifier
- Explicit declaration
- Implicit declaration
- Binding
- Initialization
- Re-initialization
- Deinitialization
- Late binding
- Early binding
- Static binding

1. Entities
  - variable
  - constant
  - static
  - fn
  - trait
  - module
2. Declaration
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
3. Binding
  - initialization
  - re-initialization
  - deinitialization
  - binding
    - early binding
    - late binding

value
variable
allocation
memory address/location
stack/heap
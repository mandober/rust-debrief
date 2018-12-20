# calling convention

- [Calling convention](#calling-convention)
- [Call site processing](#call-site-processing)
- [Subroutine entry processing](#subroutine-entry-processing)
- [Return processing](#return-processing)
- [Unwinding](#unwinding)


*calling convention* is an implementation-level (low-level) scheme for how functions receive parameters from their caller and how they return a result. Differences in various implementations include where parameters, return values, return addresses and scope links are placed (register, stack, mix of both, etc.), and how the tasks of preparing for a function call and restoring the environment afterward are divided between the caller and the callee.

## Call site processing

Usually the call stack manipulation needed at the site of a call to a subroutine is minimal (which is good since there can be many call sites for each subroutine to be called). The values for the actual arguments are evaluated at the call site, since they are specific to the particular call, and either pushed onto the stack or placed into registers, as determined by the used calling convention. The actual call instruction, such as *branch and link*, is then typically executed to transfer control to the code of the target subroutine.

## Subroutine entry processing
In the called subroutine, the first code executed is usually termed the subroutine prologue, since it does the necessary housekeeping before the code for the statements of the routine is begun.

The prologue will commonly save the return address left in a register by the call instruction by pushing the value onto the call stack. Similarly, the current stack pointer and/or frame pointer values may be pushed. Alternatively, some instruction set architectures automatically provide comparable functionality as part of the action of the call instruction itself, and in such an environment the prologue does not need to do this.

If frame pointers are being used, the prologue will typically set the new value of the frame pointer register from the stack pointer. Space on the stack for local variables can then be allocated by incrementally changing the stack pointer.

## Return processing
When a subroutine is ready to return, it executes an epilogue that undoes the steps of the prologue. This will typically restore saved register values (such as the frame pointer value) from the stack frame, pop the entire stack frame off the stack by changing the stack pointer value, and finally branch to the instruction at the return address. Under many calling conventions the items popped off the stack by the epilogue include the original argument values, in which case there usually are no further stack manipulations that need to be done by the caller. With some calling conventions, however, it is the caller's responsibility to remove the arguments from the stack after the return.


## Unwinding

Returning from the called function will pop the top frame off of the stack, perhaps leaving a return value. The more general act of popping one or more frames off the stack to resume execution elsewhere in the program is called stack unwinding and must be performed when non-local control structures are used, such as those used for exception handling. In this case, the stack frame of a function contains one or more entries specifying exception handlers. When an exception is thrown, the stack is unwound until a handler is found that is prepared to handle (catch) the type of the thrown exception.

Some languages have other control structures that require general unwinding. Pascal allows a global `goto` statement to transfer control out of a nested function and into a previously invoked outer function. This operation requires the stack to be unwound, removing as many stack frames as necessary to restore the proper context to transfer control to the target statement within the enclosing outer function. Similarly, C has the `setjmp` and `longjmp` functions that act as non-local `goto`s. Common Lisp allows control of what happens when the stack is unwound by using the unwind-protect special operator.

When applying a continuation, the stack is (logically) unwound and then rewound with the stack of the continuation. This is not the only way to implement continuations; for example, using multiple, explicit stacks, application of a continuation can simply activate its stack and wind a value to be passed. The Scheme programming language allows arbitrary thunks to be executed in specified points on "unwinding" or "rewinding" of the control stack when a continuation is invoked.

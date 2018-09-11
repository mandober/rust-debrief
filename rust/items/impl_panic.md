# impl panic

Implementation of panics backed by libgcc/libunwind (in some form)

For background on exception handling and stack unwinding please see "Exception Handling in LLVM" (llvm.org/docs/ExceptionHandling.html) and documents linked from it. These are also good reads:
- http://mentorembedded.github.io/cxx-abi/abi-eh.html
- http://monoinfinito.wordpress.com/series/exception-handling-in-c/
- http://www.airs.com/blog/index.php?s=exception+frames


## A brief summary

Exception handling happens in two phases: 
1. search phase
2. cleanup phase

In both phases the unwinder walks stack frames from top to bottom using information from the stack frame unwind sections of the current process's modules ("module" here refers to an OS module i.e. an executable or a dynamic library).

For each stack frame, it invokes the associated "personality routine", whose address is also stored in the unwind info section.

In the search phase, the job of a personality routine is to examine exception object being thrown, and to decide whether it should be caught at that stack frame. Once the handler frame has been identified, cleanup phase begins.

In the cleanup phase, the unwinder invokes each personality routine again. This time it decides which (if any) cleanup code needs to be run for the current stack frame. If so, the control is transferred to a special branch in the function body, the "landing pad", which invokes destructors, frees memory, etc. At the end of the landing pad, control is transferred back to the unwinder and unwinding resumes.

Once stack has been unwound down to the handler frame level, unwinding stops and the last personality routine transfers control to the catch block.


## Personalities
`eh_personality` and `eh_unwind_resume` language items are used by the compiler when generating unwind info. The first one is the personality routine described above. The second one allows compilation target to customize the process of resuming unwind at the end of the landing pads. `eh_unwind_resume` is used only if
`custom_unwind_resume` flag in the target options is set.

...

Register ids were lifted from LLVM's
`TargetLowering::getExceptionPointerRegister()` and
`TargetLowering::getExceptionSelectorRegister()`
for each architecture, then mapped to DWARF register numbers via register definition tables (typically `<arch>RegisterInfo.td`, search for "DwarfRegNum").
See: http://llvm.org/docs/WritingAnLLVMBackend.html#defining-a-register.

..

The following code is based on GCC's C and C++ personality routines. For reference, see: 
https://github.com/gcc-mirror/gcc/blob/master/libstdc++-v3/libsupc++/eh_personality.cc
https://github.com/gcc-mirror/gcc/blob/trunk/libgcc/unwind-c.c

The personality routine for most of our targets, except ARM, which has a slightly different ABI (however, iOS goes here as it uses SjLj unwinding). Also, the 64-bit Windows implementation lives in `seh64_gnu.rs`

...

ARM EHABI personality routine.
http://infocenter.arm.com/help/topic/com.arm.doc.ihi0038b/IHI0038B_ehabi.pdf

...




## Frame unwind info registration
Each module's image contains a frame unwind info section (usually `.eh_frame`).  When a module is loaded/unloaded into the process, the unwinder must be informed about the location of this section in memory. The methods of achieving that vary by the platform. On some (e.g. Linux), the unwinder can discover unwind info sections on its own (by dynamically enumerating currently loaded modules via the `dl_iterate_phdr()` API and finding their `.eh_frame` sections). Others, like Windows, require modules to actively register their unwind info sections via unwinder API.

This module defines two symbols which are referenced and called from `rsbegin.rs` to register our information with the GCC runtime. The implementation of stack unwinding is (for now) deferred to `libgcc_eh`, however Rust crates use these Rust-specific entry points to avoid potential clashes with any GCC runtime.


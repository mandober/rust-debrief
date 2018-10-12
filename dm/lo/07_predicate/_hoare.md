# Hoare logic

https://en.wikipedia.org/wiki/Hoare_logic

Hoare logic (also known as Floyd–Hoare logic or Hoare rules) is a formal system with a set of logical rules for reasoning rigorously about the correctness of computer programs. It was proposed in 1969 by the British computer scientist and logician Tony Hoare, and subsequently refined by Hoare and other researchers.[1] The original ideas were seeded by the work of Robert W. Floyd, who had published a similar system for flowcharts.


## Hoare triple
The central feature of Hoare logic is the Hoare triple. A triple describes how the execution of a piece of code changes the state of the computation. A Hoare triple is of the form

{P} C {Q}
where P and Q are assertions and C is a command. P is named the precondition and Q the postcondition: when the precondition is met, executing the command establishes the postcondition. Assertions are formulae in predicate logic.

Hoare logic provides axioms and inference rules for all the constructs of a simple imperative programming language. In addition to the rules for the simple language in Hoare's original paper, rules for other language constructs have been developed since then by Hoare and many other researchers. There are rules for concurrency, procedures, jumps, and pointers.

## Partial and total correctness
Using standard Hoare logic, only partial correctness can be proven, while termination needs to be proved separately. Thus the intuitive reading of a Hoare triple is: Whenever P holds of the state before the execution of C, then Q will hold afterwards, or C does not terminate. In the latter case, there is no "after", so Q can be any statement at all. Indeed, one can choose Q to be false to express that C does not terminate.

Total correctness can also be proven with an extended version of the While rule.

In his 1969 paper, Hoare used a narrower notion of termination which also entailed absence of any run-time errors: "_Failure to terminate may be due to an infinite loop; or it may be due to violation of an implementation-defined limit, for example, the range of numeric operands, the size of storage, or an operating system time limit_".

## Rules
- **Empty statement axiom schema**
  The empty statement rule asserts that the skip statement does not change the state of the program, thus whatever holds true before skip also holds true afterwards
- **Assignment axiom schema**
  The assignment axiom states that, after the assignment, any predicate that was previously true for the right-hand side of the assignment now holds for the variable. Formally, let P be an assertion in which the variable x is free.
- **Rule of composition**
  Hoare's rule of composition applies to sequentially executed programs S and T, where S executes prior to T and is written S;T (Q is called the midcondition)
- **Conditional rule**
- **Consequence rule**
- **While rule**
- **While rule for total correctness**


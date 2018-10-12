<!-- TOC -->

- [Classical logic](#classical-logic)
- [Infinitary logic](#infinitary-logic)
- [Paraconsistent logic](#paraconsistent-logic)
- [Separation logic](#separation-logic)
- [Many-sorted logic](#many-sorted-logic)
- [n-valued logics](#n-valued-logics)
- [Second-order logic](#second-order-logic)

<!-- /TOC -->


## Classical logic
Law of excluded middle
Double negative elimination 
Law of noncontradiction 
Principle of explosion 
Monotonicity of entailment 
Idempotency of entailment 
Commutativity of conjunction 
De Morgan's laws 
Principle of bivalence
Propositional logic
Predicate logic


## Infinitary logic
https://en.wikipedia.org/wiki/Infinitary_logic
An infinitary logic allows infinitely long statements or proofs.

Infinitary logics may have different properties from FOL, such as the lack of compactness and completeness.

Notions of compactness and completeness that are equivalent in finitary logic, sometimes are not equivalent in infinitary logics.

Non-Monotonic Logics
Logics (devised for AI) in which adding premises may diminish the set of derivable conclusions.
Logical Frameworks
These are logical systems which are intended to provide a framework within which work can be undertaken in a wide range of different logics. The framework may provide a formal metalanguage in which the various logics are defined, or a core logic, of which the various logics are extensions.
Programming Logics
These are logical systems customised to reasoning about programs in some particular programming language, or general schemes and methods for reasoning about programs (e.g. Hoare Logic).
Specification Logics
These are the inference systems associated with formal specification languages (such as may be used in the specification of highly secure or safety critical systems).


## Paraconsistent logic
A **paraconsistent logic** is a logical system that attempts to deal with contradictions in a discriminating manner.

Paraconsistent logic is the subfield of logic that is concerned with studying and developing paraconsistent or inconsistency tolerant logic systems.

The defining feature of a paraconsistent logic is that it rejects the **principle of explosion**.

As a result, paraconsistent logics, unlike classical and other logics, can be used to formalize inconsistent but non-trivial theories.
https://en.wikipedia.org/wiki/Paraconsistent_logic


## Separation logic
Separation logic is an extension of Hoare logic, a way of reasoning about programs.
It was developed by John C. Reynolds, Peter O'Hearn, Samin Ishtiaq and Hongseok Yang, drawing upon early work by Rod Burstall.
The assertion language of separation logic is a special case of the __logic of bunched implications (BI)__.
Separation logic facilitates reasoning about:
- programs that manipulate pointer data structures—including information hiding in the presence of pointers;
- **transfer of ownership** (avoidance of semantic frame axioms); and
- **virtual separation** (modular reasoning) between concurrent modules.
Separation logic supports the developing field of research described by Peter O'Hearn and others as local reasoning, whereby specifications and proofs of a program component mention only the portion of memory used by the component, and not the entire global state of the system. Applications include automated program verification (where an algorithm checks the validity of another algorithm) and automated parallelization of software.

https://en.wikipedia.org/wiki/Separation_logic


## Many-sorted logic
https://en.wikipedia.org/wiki/Many-sorted_logic
Many-sorted logic can reflect formally our intention not to handle the universe as a homogeneous collection of objects, but to partition it in a way that is similar to types in typeful programming. Both functional and assertive "parts of speech" in the language of the logic reflect this typeful partitioning of the universe, even on the syntax level: substitution and argument passing can be done only accordingly, respecting the "sorts".
There are more ways to formalize the intention mentioned above; a many-sorted logic is any package of information which fulfills it. In most cases, the following are given:
a set of sorts, S
an appropriate generalization of the notion of signature to be able to handle the additional information that comes with the sorts.
The domain of discourse of any structure of that signature is then fragmented into disjoint subsets, one for every sort.


## n-valued logics

Values
2-valued logic
3-valued logic
n-valued logic
∞-valued logic

Logics
- __Two-valued logic__ is classical logic with `true` and `false` values, but it may be extended to __n-valued logic__ (n > 2).
- __Many-valued__ logic (multi- or multiple-) is a _propositional calculus_ in which there are more than two truth values.
- __Three-valued__ e.g. Łukasiewicz's and Kleene's, accepting `true`, `false`, and `unknown`
- **Finite-valued** (Finitely-many valued) with more than 3 values
- **Infinite-valued** (Infinitely-many valued)
  - **Fuzzy logic**
  - **Probability logic**

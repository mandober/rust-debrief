# Deduction systems



One way to classify different styles of deduction systems is to look at the form of judgments in the system and see which things may appear as the conclusion of a (sub)proof.


The simplest judgment form is used in **Hilbert-style deduction systems**, where a judgment has the form $$B$$, where $$B$$ is any formula (of first-order or other logic).

**Theorems** are formulas that appear as the concluding judgment in a valid proof.

A Hilbert-style system needs no distinction between formulas and judgments.

The price paid for the simple syntax of a Hilbert-style system is that complete formal proofs tend to get extremely long.

Concrete arguments about proofs in such a system almost always appeal to the deduction theorem. This leads to the idea of including the deduction theorem as a formal rule in the system, which happens in natural deduction.


## Natural deduction systems
In **natural deduction**, judgments have the shape:
$$A_1, A_2, \dots A_n \vdash B$$
where the A's and B are formulae. Permutations of the A's are immaterial.

A **judgment** consists of a (possibly empty) list of formulae on the left-hand side of a turnstile symbol, with a single formula on the right-hand side.

**Theorems** are those formulae $$B$$ such that
$$\vdash B$$
(empty left-hand side) is the conclusion of a valid proof.

The standard semantics of a judgment in natural deduction is that it asserts that whenever antecedents are all true, the consequent will also be true.

The judgments below are equivalent in the strong sense that a proof of either one may be extended to a proof of the other.

$$
A_1, A_2, \dots A_n \vdash B
\iff
\vdash A_1, A_2, \dots A_n \to B
$$


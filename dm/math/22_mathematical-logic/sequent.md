# Sequent

A sequent is a very general kind of conditional assertion, where antecedent (hypotheses) is on the left side and consequent (succedent) is on the right side of the turnstile sign.

\[
\phi
\vdash
\psi
\]

This sequent can be read as "$$\phi$$ implies $$\psi$$" or "$$\psi$$ can be proven using $$\phi$$". The terms $$\phi$$ and $$\psi$$ are atomic clauses and they represent zero or more logical statements:

\[
A_1, A_2, \dots, A_n
\vdash
B_1, B_2, \dots, B_n
\]

A sequent is understood to mean that if all of the antecedent conditions are true, then at least one of the consequent formulas is true. Explicitly written as:

\[
A_1 \land A_2 \land \dots \land A_n
\vdash
B_1 \lor B_2 \lor \dots \lor B_n
\]

Explicit version is uncommon as it's understood that commas represent conjunctions when separating antecedents, and disjunctions when separating consequents. This style of conditional assertion is almost always associated with the conceptual framework of sequent calculus.

Sequents are best understood in the context of logical judgments such as:
* **Unconditional assertion**: no antecedent formulas.
* **Conditional assertion**: any number of antecedent formulas.
  - **Simple conditional assertion**: a single consequent formula.
  - **Sequent**: multiple consequent formulas.

Examples:
- Unconditional assertion: $$\vdash B$$, means that B is true.
- Simple conditional assertion: $$A_1, A_2, A_3 \vdash B$$ 
  meaning __if__ A1 `AND` A2 `AND` A3 are true, __then__ B is true.
- Sequent: $$A_1, A_2, A_3 \vdash B_1, B_2, B_3, B_4$$ 
  meaning __if__ A1 `AND` A2 `AND` A3 are true, __then__ B1 `OR` B2 `OR` B3 `OR` B4 is true.

Thus sequents are a generalization of simple conditional assertions, which are a generalization of unconditional assertions.

The word `OR` here is the inclusive `OR`. The motivation for disjunctive semantics on the right side of a sequent comes from 3 main benefits, as identified in the founding paper on sequent by Gentzen:
1. The symmetry of the classical inference rules for sequents with such semantics.
2. The ease and simplicity of converting such classical rules to intuitionistic rules.
3. The ability to prove completeness for predicate calculus when it is expressed in this way.






---

https://blog.tchatzigiannakis.com/structural-rules-and-substructural-logics/


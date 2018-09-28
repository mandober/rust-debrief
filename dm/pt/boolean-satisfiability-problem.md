# Boolean satisfiability problem

https://en.wikipedia.org/wiki/Boolean_satisfiability_problem

In CS, the Boolean satisfiability problem (sometimes called _propositional satisfiability problem_ and abbreviated as _SATISFIABILITY_ or _SAT_) is the problem of determining if there exists an interpretation that satisfies a given Boolean formula.

In other words, it asks whether the variables of a given Boolean formula can be consistently replaced by the values TRUE or FALSE in such a way that the formula evaluates to TRUE.

If this is the case, the formula is called _satisfiable_. On the other hand, if no such assignment exists, the function expressed by the formula is FALSE for all possible variable assignments and the formula is _unsatisfiable_.

For example, the formula `a AND NOT b` is satisfiable because one can find the values `a = TRUE` and `b = FALSE`, which make `(a AND NOT b) = TRUE`. In contrast, `a AND NOT a` is unsatisfiable.


## Basic definitions and terminology

A propositional logic formula (Boolean expression) is built from:
- variables
- operator AND (conjunction, ∧)
- operator  OR (disjunction, ∨)
- operator NOT (negation, ¬)
- parentheses

A formula is said to be satisfiable if it can be made TRUE by assigning appropriate logical values (i.e. TRUE, FALSE) to its variables.

> The Boolean satisfiability problem (SAT) is, given a formula, to check whether it is satisfiable.

There are several special cases of the Boolean satisfiability problem in which the formulas are required to have a particular structure.

Literals:
- _positive literal_: variable
- _negative literal_: negation of a variable

- __Clause__ is a _disjunction_ of literal(s).
- __Horn clause__ if a clause contains at most one positive literal.
- __Conjunctive Normal Form (CNF)__: formula that is a *conjunction* of clause(s).

__Conjunctive normal form__ (CNF): in Boolean logic, a formula is in  Conjunctive Normal Form (or Clausal Normal Form) if it is a conjunction of one or more clauses, where a clause is a disjunction of literals; otherwise put, it is an __AND of ORs__.

For example:
`x1`       is a positive literal,
`¬x2`      is a negative literal,
`x1 ∨ ¬x2` is a clause, and 
`(x1 ∨ ¬x2) ∧ (¬x1 ∨ x2 ∨ x3) ∧ ¬x1` is a formula in CNF.

The first and third clauses are Horn clauses, but its second clause is not.
The formula is satisfiable by choosing `x1 = F`, `x2 = F`, and `x3` arbitrarily, since:
`(F ∨ ¬F) ∧ (¬F ∨ F ∨ x3) ∧ ¬F` evaluates to 
`(F ∨ T) ∧ (T ∨ F ∨ x3) ∧ T`, and in turn to 
`T ∧ T ∧ T` (i.e. to `TRUE`).

In contrast, the CNF formula `a ∧ ¬a`, consisting of two clauses of one literal, is unsatisfiable, since for `a=T` or `a=F` it evaluates to 
`T ∧ ¬T` (i.e., `F`) or `F ∧ ¬F` (i.e. again `F`), respectively.













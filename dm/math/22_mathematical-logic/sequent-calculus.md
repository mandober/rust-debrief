# Sequent calculus

https://en.wikipedia.org/wiki/Sequent_calculus

Sequent calculus is a style of formal logical argumentation where every line of a proof, instead of being an unconditional tautology, is rather a _conditional tautology_, called a _sequent_ by Gerhard Gentzen, the inventor.

Each conditional tautology is inferred from other conditional tautologies on earlier lines in a formal argument, according to _rules of inference_.


## Proof calculi

Sequent calculus is one of several extant styles of _proof calculus_ for expressing line-by-line logical arguments.

Proof calculi:
* __Hilbert style__: every line is an unconditional tautology (or theorem).
* __Gentzen style__: every line is a conditional tautology (or theorem) with zero or more conditions on the left.
* __Natural deduction__: every conditional line has exactly one asserted proposition on the right.
* __Sequent calculus__: every conditional line has zero or more asserted propositions on the right.

_Natural deduction_ and _sequent calculus_ systems are particular distinct kinds of Gentzen-style systems. _Gentzen-style_ systems typically have very few axioms, if any, relying more on the inference rules. _Hilbert-style_ systems typically have a very small number of inference rules, relying more on the sets of axioms.

_Gentzen-style_ systems have significant practical and theoretical advantages compared to Hilbert-style systems.

For example, both natural deduction and sequent calculus systems facilitate the _elimination_ and _introduction_ of _universal_ and _existential quantifiers_ so that unquantified logical expressions can be manipulated according to the much simpler rules of propositional calculus.

In a typical argument, quantifiers are eliminated, then propositional calculus is applied to unquantified expressions (which typically contain free variables), and then the quantifiers are reintroduced, much like the way way in which mathematical proofs are carried out in practice by mathematicians.

Predicate calculus proofs are generally much easier to discover with this approach, and are often shorter. Natural deduction systems are more suited to practical theorem-proving. Sequent calculus systems are more suited to theoretical analysis.


## Hilbert-style deduction systems

One way to classify different styles of deduction systems is to look at the form of judgments in the system, i.e. which things may appear as the conclusion of a (sub)proof.

The simplest judgment form is used in Hilbert-style deduction systems, where a judgment has the form $$B$$, where $$B$$ is any formula of first-order logic (or whatever logic the deduction system applies to).

The theorems are those formulas that appear as the concluding judgment in a valid proof. A Hilbert-style system needs no distinction between formulas and judgments. The price paid for the simple syntax of a Hilbert-style system is that complete formal proofs tend to get extremely long. Concrete arguments about proofs in such a system almost always appeal to the deduction theorem. This leads to the idea of including the deduction theorem as a formal rule in the system, which happens in natural deduction.


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



## Sequent calculus systems

Sequent calculus generalizes the form of a natural deduction judgment to a syntactic object called a sequent:

$$A_{1}, \ldots, A_{n} \vdash B_{1}, \ldots, B_{k}$$

- formulas on left-hand side of the turnstile are called the __antecedent__
- formulas on right-hand side are called the __succedent__ or __consequent__
- together they are called __cedents__ or __sequents__
- The LHS or RHS (or neither or both) may be empty
- As in natural deduction, theorems are those $$B$$ where $$\vdash B$$ is the conclusion of a valid proof.
- The standard semantics of a sequent is an assertion that whenever every antecedent is true, at least one consequent will also be true.
- Thus the empty sequent, having both cedents empty, is false.
- A comma to the left of turnstile represents an `AND`
- A comma to the right of turnstile represents an `OR` (inclusive)

The sequents,

$$A_1, \dots, A_n \vdash B_1, \dots, B_k$$,

and

$$\vdash (A_1 \land\dots\land A_n)\to(B_1 \lor\dots\lor B_k)$$

are equivalent in the sense that a proof of either one may be extended to a proof of the other.

In a classical context the semantics of the sequent can also (by propositional tautology) be expressed as:

$$\vdash (\neg A_1 \lor \dots \lor \neg A_n ) \lor
(B_1 \lor \dots \lor B_k)$$

which means that it cannot be the case that all of the As (antecedents) are true and all of the Bs (succedents) are false.

```
⊢  CNF → DNF
⊢ ¬CNF ∨ DNF
 ¬¬CNF ⊢ DNF
   CNF ⊢ DNF

⊢  (A1 ∧ ⋯ ∧ An)  → (B1 ∨ ⋯ ∨ Bk)  ⟺
⊢ ¬(A1 ∧ ⋯ ∧ An)  ∨ (B1 ∨ ⋯ ∨ Bk)  ⟺
⊢ (¬A1 ∨ ⋯ ∨ ¬An) ∨ (B1 ∨ ⋯ ∨ Bk)  ⟺
    A1 ∧ ⋯ ∧ An   ⊢  B1 ∨ ⋯ ∨ Bk   ⟺
    A1 , ⋯ , An   ⊢  B1 , ⋯ , Bk
```

This means that a symmetry such as De Morgan's laws, which manifests itself as logical negation on the semantic level, translates directly into a left-right symmetry of sequents — and indeed, the inference rules in sequent calculus for dealing with conjunction are mirror images of those dealing with disjunction.

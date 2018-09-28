# Sequent

A sequent is a general kind of conditional assertion, where hypotheses are on the left side and consequents are on the right side of the _turnstile_ sign: $$\phi \vdash \psi$$.

> Sequents are a generalization of conditional assertions, which are a generalization of unconditional assertions.

The turnstile sign can be read as "_therefore_", "_entails_", "_implies_", or "_proves_" as in "$$\psi$$ _can be proven using_ $$\phi$$". Here, the terms $$\phi$$ and $$\psi$$ are clauses that represent logical statements and they can be instantiated to any number of formulae.

A sequent may have any number of _condition formulas_ (on the left side), called **antecedents**, and any number of _asserted formulas_ (on the right side), called **succedents** or **consequents**. Together they are called __cedents__ or __sequents__.


$$
A_1, A_2, \dots, A_m
\vdash
B_1, B_2, \dots, B_n
$$

The standard semantics of a sequent is an assertion that whenever every antecedent is true, at least one consequent will also be true, explicitly written as:

$$
A_1 \land A_2 \land \dots \land A_m
\vdash
B_1 \lor B_2 \lor \dots \lor B_n
$$

Explicit version is uncommon as it's understood that commas represent **conjunctions** when separating antecedents, and **disjunctions** when separating consequents.


## The form and semantics of sequents

Sequent forms:
* **Unconditional assertion** has no antecedent formulas.     
  $$\quad\vdash B\quad$$ means that $$B$$ is true
* **Conditional assertion**has any number of antecedent formulas.
  - **Simple conditional assertion** has a single consequent formula.    
    $$A_1, A_2, A_3 \vdash B$$ meaning 
    if $$A_1$$ and $$A_2$$ and $$A_3$$ are true, then $$B$$ is true.
  - **Sequent**: multiple consequent formulas.    
    $$A_1, A_2, A_3 \vdash B_1, B_2, B_3, B_4$$ meaning 
    if $$A_1$$ and $$A_2$$ and $$A_3$$ are true, then $$B_1$$ or $$B_2$$ or $$B_3$$ or $$B_4$$ is true.
* **Empty sequent**, having both cedents empty, is "_not satisfiable_" (or _false_ if insisted upon). The meaning of the sequent is effectively $$\top \vdash \bot$$, which is equivalent to $$\vdash \bot$$, which cannot be valid.

The "_or_" is consequents is "_inclusive or_".

## Syntax details
Generally, in a sequent of the form $$\Gamma \vdash \Sigma$$ 
both $$Γ$$ and $$Σ$$ are sequences of logical formulas.

The same formula may appear twice in the same sequence i.e. the formula can be duplicated (contraction).

The full set of sequent calculus inference rules contains rules:
- to swap adjacent formulas on the left and on the right of the assertion symbol, and thereby arbitrarily permute the left and right sequences, 
- and also to insert arbitrary formulas 
- and remove duplicate copies within the left and the right sequences.

Common structural rules:
- **Weakening** is an addition of formulae in a cedent.
- **Exchange** is swapping of two cedent formulae.
- **Contraction** is a duplication of a formula in a cedent, or when two unifiable members on the same side of a sequent are replaced by a single member or a common instance.


## Properties
Since every formula in the antecedent must be true to conclude the truth of at least one formula in the succedent, adding formulas to either side results in a __weaker sequent__, while removing them from either side gives a __stronger sequent__. This is one of the symmetry advantages which follows from the use of disjunctive semantics on the RHS of the assertion symbol, whereas conjunctive semantics is adhered to on the LHS.


**Empty antecedent**     
In the extreme case where the list of antecedent formulas of a sequent is empty, the consequent is unconditional. This differs from the simple unconditional assertion because the number of consequents is arbitrary, not necessarily a single consequent.

So, for example, $$\vdash B_1, B_2$$ means that either $$B_1$$ or $$B_2$$, or both are true. An empty antecedent formula list is equivalent to the "**_always true_**" proposition, called the "**_verum_**", denoted by $$\top$$.

**Empty consequent**     
In the extreme case where the list of consequent formulas of a sequent is empty, the rule is still that at least one term on the right be true, which is clearly impossible.

This is signified by the "**_always false_**" proposition, called the "**_falsum_**", denoted $$\bot$$. Since the consequence is false, at least one of the antecedents must be false.

So, for example, $$A_1, A_2 \vdash$$ means that at least one of the antecedents $$A_1$$ and $$A_2$$ must be false.

This again shows the symmetry due to disjunctive semantics of the RHS:
- If the LHS is empty, then one or more RHS propositions must be true.
- If the RHS is empty, then one or more LHS propositions must be false.

**Empty cedents**     
The extreme case of empty sequent is "_not satisfiable_".
In this case, the meaning of the sequent is effectively 
$$\top \vdash \bot$$, which is equivalent to
$$\vdash \bot$$, which clearly cannot be valid.


## Examples
A sequent of the form $$\vdash\alpha, \beta$$ for logical formulas $$\alpha$$ and $$\beta$$, means that either $$\alpha$$ is true or $$\beta$$ is true, but it doesn't mean that either of them is a tautology.

To clarify this, consider: $$\vdash B\lor A,C\lor \lnot A$$

This is a valid sequent because either $$B \lor A$$ is true or $$C \lor \lnot A$$ is true. But neither of these expressions is a tautology in isolation. It is the disjunction of these two expressions which is a tautology.

Similarly, a sequent of the form $$\alpha, \beta \vdash$$ 
for logical formulas $$\alpha$$ and $$\beta$$, means that either $$\alpha$$ is false or $$\beta$$ is false, but it doesn't mean that either $$\alpha$$ is a contradiction or $$\beta$$ is a contradiction.

To clarify this, consider: $$B\land A,C\land \lnot A \vdash$$

This is a valid sequent because either $$B\land A$$ is false or $$C\land \lnot A$$ is false. But neither of these expressions is a contradiction in isolation. It is the conjunction of these two expressions which is a contradiction.

## Rules
Most proof systems provide ways to deduce one sequent from another. These inference rules are written with a list of sequents above and below a line. This rule indicates that if everything above the line is true, so is everything under the line.

A typical rule is:

$$
\underline{\Gamma ,\alpha \vdash \Sigma \qquad \Gamma \vdash \alpha}\\
\quad \quad \quad {\Gamma \vdash \Sigma}
$$

This indicates that if we can deduce that $$\Gamma , \alpha$$ yields $$\Sigma$$, and that $$\Gamma$$ yields $$\alpha$$, then we can also deduce that $$\Gamma$$ yields $$\Sigma$$.


## Interpretation
The assertion symbol in sequents originally meant exactly the same as the implication operator. But over time, its meaning has changed to signify provability within a theory rather than semantic truth in all models.


The intuitive meaning of the sequent $$\Gamma \vdash \Sigma$$ is that under the assumption of Γ the conclusion of Σ is provable.

Classically, the formulae on the left of the turnstile can be interpreted conjunctively while the formulae on the right can be considered as a disjunction.

This means that, when all formulae in Γ hold, then at least one formula in Σ also has to be true.

If the succedent is empty, this is interpreted as falsity, i.e. $$\Gamma \vdash$$ means that Γ proves falsity and is thus inconsistent.

On the other hand an empty antecedent is assumed to be true, i.e., $$\vdash \Sigma$$ means that Σ follows without any assumptions, i.e., it is always true (as a disjunction). A sequent of this form, with Γ empty, is known as a __logical assertion__.

Since formal proofs in proof theory are purely syntactic, the meaning of (the derivation of) a sequent is only given by the properties of the calculus that provides the actual rules of inference.

In their introductory logical form, Γ represents a set of assumptions that we begin our logical process with, for example "Socrates is a man" and "All men are mortal". The Σ represents a logical conclusion that follows under these premises. For example "Socrates is mortal" follows from a reasonable formalization of the above points and we could expect to see it on the Σ side of the turnstile. In this sense, $$\vdash$$ means the process of reasoning, or "therefore" in English.


## Variations
The general notion of sequent introduced here can be specialized in various ways.

**LJ Sequent Calculus**    
The restriction of the general sequent calculus to single-succedent-formula sequents, with the same inference rules as for general sequents, constitutes an intuitionistic sequent calculus, denoted as __LJ__.

Similarly, one can obtain calculi for __dual-intuitionistic logic__ (a type of paraconsistent logic) by requiring that sequents be singular in the antecedent.

In many cases, sequents are assumed to consist of multisets or sets instead of sequences, disregarding the order and number of occurrences of a formula.

For classical propositional logic this does not yield a problem, since the conclusions that one can draw from a collection of premises do not depend on these data. In substructural logic, however, this may become quite important.

__Natural deduction systems__ use single-consequence conditional assertions, but they typically do not use the same sets of inference rules as Gentzen introduced in 1934. Tabular natural deduction systems are particularly used, being very convenient for practical theorem-proving in propositional calculus and predicate calculus.






---

- https://en.wikipedia.org/wiki/Sequent
- https://en.wikipedia.org/wiki/Sequent_calculus
- https://blog.tchatzigiannakis.com/structural-rules-and-substructural-logics/


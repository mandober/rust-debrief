# Logical equivalence

<!-- TOC -->

- [Logical Equivalence](#logical-equivalence)
- [Logical equivalences](#logical-equivalences)
- [Logical equivalences with conditionals](#logical-equivalences-with-conditionals)
- [Relation to material equivalence](#relation-to-material-equivalence)
- [Logical equality](#logical-equality)

<!-- /TOC -->


## Logical Equivalence

In logic, statements $$p$$ and $$q$$ are **logically equivalent** if they have the same logical content. That is, if they have the same truth value in every model.

The logical equivalence of $$p$$ and $$q$$ is denoted by $$p \equiv q,\ Epq,\ p\iff q$$. Since these symbols are also used for material equivalence, the proper interpretation depends on the context.

Logical equivalence is different from material equivalence, although the two concepts are closely related.



## Logical equivalences

- **Identity laws**
  - $$\quad p \land \top \equiv p$$ 
  - $$\quad p \lor \bot\equiv p$$
- **Domination laws**
  - $$\quad p\land \bot \equiv \bot$$ 
  - $$\quad p \lor  \top \equiv \top$$
- **Idempotent laws**
  - $$\quad p \land p \equiv p$$
  - $$\quad p \lor  p \equiv p$$
- **Negation laws**
  - $$\quad p\land\lnot p\equiv\bot$$
  - $$\quad p\lor\lnot p\equiv\top$$
- **Double negation law**
  - $$\quad \lnot (\lnot p) \equiv p$$
- **De Morgan's laws**
  - $$\lnot (p\land q) \equiv \lnot p \lor \lnot q$$
  - $$\lnot (p \lor q) \equiv \lnot p \land \lnot q$$
- **Commutative laws**
  - $$\quad p \land q \equiv q \land p$$
  - $$\quad p \lor q \equiv q \lor p$$
- **Associative laws**
  - $$(p \land q) \land r \equiv p \land (q \land r)$$
  - $$(p \lor q) \lor r \equiv p \lor (q \lor r)$$
- **Distributive laws**
  - $$p \land (q \lor r) \equiv (p \land q) \lor (p \land r)$$
  - $$p \lor (q \land r) \equiv (p \lor q) \land (p \lor r)$$
- **Absorption laws**
  - $$p \land (p \lor q) \equiv p$$
  - $$p \lor (p \land q) \equiv p$$


## Logical equivalences with conditionals

$$
p\rightarrow q \quad \ \ \equiv \neg p\vee q\\
p\rightarrow q \quad \ \ \equiv \neg q\rightarrow \neg p\\
p\vee q \quad \quad      \equiv \neg p\rightarrow q\\
p\wedge q \quad \quad    \equiv \neg (p\rightarrow \neg q)\\
\neg (p\rightarrow q)\   \equiv p\wedge \neg q
\\
(p\rightarrow q)\wedge (p\rightarrow r) \equiv p\rightarrow (q\wedge r)\\
(p\rightarrow q)\vee (p\rightarrow r)   \equiv p\rightarrow (q\vee r)\\
(p\rightarrow r)\wedge (q\rightarrow r) \equiv (p\vee q)\rightarrow r\\
(p\rightarrow r)\vee (q\rightarrow r)  \equiv (p\wedge q)\rightarrow r
$$


The following statements are logically equivalent:
- If Lisa is in France, then she is in Europe.
  $$f\rightarrow e$$
- If Lisa is not in Europe, then she is not in France.
  $$\neg e\rightarrow \neg f$$

Syntactically, these two and are derivable from each other via the rules of **contraposition** and **double negation**.

Semantically, they are true in exactly the same models (interpretations, valuations), namely, those in which either "Lisa is in France" is false or "Lisa is in Europe" is true (this example wroks in classical logic - some non-classical logics do not deem these two sentences logically equivalent).


## Relation to material equivalence

Logical equivalence is different from material equivalence.

Formulas $$p$$ and $$q$$ are logically equivalent iff the statement of their material equivalence, $$p\iff q$$, is a tautology.

The material equivalence of $$p$$ and $$q$$, often written $$p\iff q$$, is itself another statement in the same object language as $$p$$ and $$q$$.

This statement expresses the idea "$$p$$ if and only if $$q$$".

In particular, the truth value of $$p\iff q$$ can change from one model to another.

The claim that two formulas are logically equivalent is a statement in the metalanguage, expressing a relationship between two statements $$p$$ and $$q$$.

The statements are logically equivalent if, in every model, they have the same truth value.


## Logical equality

**Logical equality** is a logical operator that corresponds to equality in Boolean algebra and to the logical **biconditional** in propositional calculus. It gives the functional value _true_ if both functional arguments have the same logical value,otherwise _false_.


---
https://www.wikiwand.com/en/Logical_equivalence

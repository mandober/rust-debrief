## Logical Consequence and Equivalence

## Logical Consequence
definition: If an argument with premises φ1,...,φn 
and conclusion ψ is valid, then ψ is a logical consequence of φ1,...,φn.

Alternatively, we can of course define logical consequence directly in terms of truth-value assignments:

Logical Consequence
definition: ψ is a logical consequence of φ1,...,φn if and only if every truth-value assignment that makes φ1,...,φn true also makes ψ true.


## Logical equivalence
In logic, statements $$p$$ and $$q$$ are logically equivalent if they have the same logical content (semantic concept). Two statements are equivalent if they have the same truth value in every model, denoted as $$p\equiv q$$, or, sometimes, $$p\iff q$$. The former symbol also denotes _material equivalence_ - the proper interpretation depends on the context. Logical equivalence is different from material equivalence, although the two concepts are closely related (and they have the same truth table).


## Logical equivalences

- Identity laws:    
  $$\quad p \land \top \equiv p$$, $$\quad p \lor \bot\equiv p$$
- Domination laws:     
  $$\quad p\land \bot \equiv \bot$$, $$\quad p \lor  \top \equiv \top$$
- Idempotent laws:     
  $$\quad p \land p \equiv p$$, $$\quad p \lor  p \equiv p$$    
- Negation laws:     
  $$\quad p\land\lnot p\equiv\bot$$, $$\quad p\lor\lnot p\equiv\top$$
- Double negation law:    
  $$\quad \lnot (\lnot p) \equiv p$$
- De Morgan's laws:      
  $$\quad 
\lnot (p \land q)
\equiv
\lnot p \lor \lnot q
$$, $$\quad
\lnot (p \lor q)
\equiv
\lnot p \land \lnot q
$$
- Commutative laws:     
  $$\quad p \land q \equiv q \land p$$, $$\quad p \lor q \equiv q \lor p$$
- Associative laws:    
  $$\quad 
(p \land q) \land r
\equiv 
p \land (q \land r)
$$, $$\quad
(p \lor q) \lor r
\equiv 
p \lor (q \lor r)
$$
- Distributive laws:    
  $$\quad 
p \land (q \lor r)
\equiv
(p \land q) \lor (p \land r)
$$, $$\quad
p \lor (q \land r)
\equiv
(p \lor q) \land (p \lor r)
$$
- Absorption laws:     
  $$\quad 
p \land (p \lor q)
\equiv p
$$, $$\quad
p \lor (p \land q)
\equiv p
$$


## Logical equivalences with conditionals

$$
p\implies q\equiv \neg p\vee q\\
p\implies q\equiv \neg q\implies \neg p\\
p\vee q\equiv \neg p\implies q\\
p\wedge q\equiv \neg (p\implies \neg q)\\
\neg (p\implies q)\equiv p\wedge \neg q\\
(p\implies q)\wedge (p\implies r)\equiv p\implies (q\wedge r)\\
(p\implies q)\vee (p\implies r)\equiv p\implies (q\vee r)\\
(p\implies r)\wedge (q\implies r)\equiv (p\vee q)\implies r\\
(p\implies r)\vee (q\implies r)\equiv (p\wedge q)\implies r
$$


The following statements are logically equivalent:

If Lisa is in France, then she is in Europe. (In symbols,  f\implies e}.)
If Lisa is not in Europe, then she is not in France. (In symbols,  \neg e\implies \neg f}.)
Syntactically, (1) and (2) are derivable from each other via the rules of contraposition and double negation. Semantically, (1) and (2) are true in exactly the same models (interpretations, valuations); namely, those in which either Lisa is in France is false or Lisa is in Europe is true.

(Note that in this example classical logic is assumed. Some non-classical logics do not deem (1) and (2) logically equivalent.)

Relation to material equivalence
Logical equivalence is different from material equivalence. Formulas p and q are logically equivalent if and only if the statement of their material equivalence ( p\iff q}) is a tautology (Copi et at. 2014:348).

The material equivalence of p and q (often written  p\iff q}) is itself another statement in the same object language as p and q. This statement expresses the idea "'p if and only if q'". In particular, the truth value of  p\iff q} can change from one model to another.

The claim that two formulas are logically equivalent is a statement in the metalanguage, expressing a relationship between two statements p and q. The statements are logically equivalent if, in every model, they have the same truth value.


## Material equivalence

Material equivalence: If and only if


## Logical equality
Logical equality is a logical operator that corresponds to equality in Boolean algebra and to the logical biconditional in propositional calculus. It gives the functional value true if both functional arguments have the same logical value, and false if they are different.



# Logical connectives

Logical connectives are symbols used to connect sentences in a grammatically valid way, such that the value of the compound sentence produced depends only on that of the original sentences and on the meaning of the connective.

Connectives are unary, binary and n-ary. Binary (dyadic) connectives join two sentences which can be thought of as the function's operands.

Logical connectives along with quantifiers are the two main types of logical constants used in formal systems such as propositional logic and predicate logic. Semantics of a logical connective can be presented as a truth (function) table.

A logical connective is similar to but not equivalent to a conditional operator.

Logical connectives in propositional logic:
* negation (not): $\lnot$
* conjunction (and): $\land$
* disjunction (or): $\lor$
* implication (if...then): $\rightarrow$
* biconditional (iff): $\iff$


Implication

If p then q, $$p\to q$$, is always true except if p is true and q is somehow false, then the overall statement is a lie (falsity). In other cases th overall statement is true:


_If it rains, the streets are wet_: $$p \to q$$
p = "raining", q = "wet streets"

1. p=1, q=1:
  _It was raining, so the streets are wet_. Overall truth value: 1.
  "Default" case. The premise is observed.
2. p=1, q=0:
  _It was raining, but the streets are NOT wet_. Overall: 0.
   Falsity. The premise was a lie.
3. p=0, q=1:
  _It was NOT raining, but the streets are wet_. Overall: 1.
4. p=0, q=0:
  _It was NOT raining, so the streets are NOT wet_. Overall: 1.
  Dry streets imply it was NOT raining.

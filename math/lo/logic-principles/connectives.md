# Logical connectives

Logical connectives are symbols used to connect sentences in a grammatically valid way, such that the value of the compound sentence produced depends only on that of the original sentences and on the meaning of the connective.

Connectives are unary or binary (or n-ary). Binary connectives join two sentences, which can be thought of as the function's operands.

Logical connectives along with quantifiers are the two main types of __logical constants__ used in formal systems such as propositional and predicate logic.


- __negation__
  - _NOT_
  - latex: $$\neg$$, glyphs: `¬`, `~`
  - negated term: $$\neg p$$
- __conjunction__
  - _AND_
  - latex: $$\land$$, glyphs: `∧`, `&`, `.`
  - terms in conjuction are conjuncts: $$p\land q$$
- __disjunction__
  - _OR_
  - latex: $$\lor$$, glyphs: `v`
  - terms in disjuction are disjuncts: $$p\lor q$$
- __implication__
  - _IF...THEN_
  - latex: $$\supset$$, $$\to$$, $$\Rightarrow$$, $$\implies$$
  - glyphs: `⊃`, `→`, `⇒`, `⟹`
  - bigger arrow may denote higher precedence
  - terms involved in implication, $$p\to q$$, are called antecedent ($$p$$) and consequent ($$q$$).
- __bicondition__
  - _IF AND ONLY IF (IFF)_
  - latex: $$\leftrightarrow$$, $$\Leftrightarrow$$, $$\equiv$$, glyphs: `↔`, `⇔`, `≡`
  - terms in bicondition: $$p \leftrightarrow q$$
  - biconditional is a shorthand: $$(p\to q) \land (q\to p)$$


## Precedence and associativity

Negation has the highest precedence, followed by conjunction and disjunction, which have the same precedence. Implication has the lowest precedence.

Parenthesis can be used for grouping.

Implication is right-associative: `p → q → r` means `p → (q → r)`

Conjuction and disjuction are commutative operations, 
$$p \lor q \equiv q\lor p$$ and $$p \land q \equiv q\land p$$,
so no parenthesis are needed to disambiguate a sequence of conjuncts, e.g.     
`p ∧ q ∧ r` ≡ `(p ∧ q) ∧ r` ≡ `p ∧ (q ∧ r)`    
or disjuncts, e.g.     
`p v q v r` ≡ `(p v q) v r` ≡ `p v (q v r)`

However, they're needed if conjuncts and disjuncts are mixed, e.g.    
$$p \lor (q \land r) \not\equiv (p\lor q) \land r$$

Sometimes, in order to avoid proliferation of parenthesis, symbols, bigger then their relative default size, are used to disambiguate the terms and operators: the bigger the symbol the higher the precedence. For example:

`p → q → r` means `p → (q → r)` since implication is right-associative.
So, to express `(p → q) → r` without the parenthesis, we can use bigger symbols: `p ⟹ q → r`


## Implication
If $$p$$ then $$q$$, $$p\to q$$, is always true, except if $$p$$ is true and $$q$$ is somehow false, then the overall statement is a __falsity__ (a lie).

In other cases the overall statement is true:

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



## Biconditional
- Represents a binary relation
- If and only if (iff) is a biconditional logical connective.

In that it is biconditional (a statement of material equivalence), the connective can be likened to the standard material conditional ("only if", equal to "if ... then") combined with its reverse ("if"); hence the name.

The result is that the truth of either one of the connected statements requires the truth of the other (i.e either both statements are true, or both are false).

It is controversial whether the connective thus defined is properly rendered by the English "if and only if", with its pre-existing meaning. There is nothing to stop one from stipulating that we may read this connective as "only if and if", although this may lead to confusion.

In writing, phrases commonly used, with debatable propriety, as alternatives to P "if and only if" Q include:
- Q is necessary and sufficient for P
- P is equivalent (or materially equivalent) to Q (compare material implication)
- P precisely if Q
- P precisely (or exactly) when Q
- P exactly in case Q
- P just in case Q


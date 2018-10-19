# Metamathematics
μϵταματηϵματικς




## Constructive Mathematics

The crucial philosophy of mathematics, intuitionism, dates back to Luitzen Brouwer and it asserts that the whole endeavor of mathematics is inevitably human-driven. Brouwer says that mathematics should not be about seeking an absolute Platonic truth, but about what, we as humans can construct. This effectively gave birth to **constructive mathematics**, in which all objects must be constructible, and a proof is not valid if it doesn't contain a "_witness_", that is, besides showing it, one has to construct an instance of the proof. A controversial example of a difference between a classical (ZF) reasoning and a constructive one has to do with **the law of excluded middle** (EM), which asserts that what's not false is true.


**Proof by contradiction** relies on the law of excluded middle: in order to prove existence of an object, it is sufficient to (dis)prove the opposite - that it doesn't exists. If the proof yields a contradiction that means, not only the assumption is false, but that the opposite is true, i.e. that an object does exist. This is accepted practice in classical logics, but not so in constructive mathematics, of which intuitionistic logic is the most well-known representative, where such proofs are pointless since they don't construct an instance of the proof, called "witness", which is an object to stand as the witnessing example for which the proof holds.

Although it may seem counter-intuitive, intuitionism holds that "not false" is not necessarily synonymous to "true", and vice versa. Also, the existence of a third alternative to provably true or provably false is the direct implication of Gödel's first incompleteness theorem, which states that some theorems are not provable at all.


## Type theory
Type theory encapsulates each mathematical theory into a universe. One universe may satisfy the law of excluded middle, but another may not. Type theory is fit to describe both.

Type theory models objects and relations between them with types. A type can be, e.g. a theorem, and proving that theorem boils down to constructing a term of the theorem type. Construction in type theory is not synonym of constructivism in Brouwer's intuitionism, which is what is usually meant by constructive mathematics. Suppose that we have proved that assuming 10 is the biggest number leads to a contradiction. In other words, the claim that there is no number bigger than 10 is false. Then, **Brouwer's intuitionism** asserts that this proof cannot construct a number bigger than 10, and the proof is thus useless. But we can add the law of excluded middle to type theory as a function which (type-theoretically) constructs a number bigger than 10 from the proof by contradiction. Conversely though, type theory without excluded middle will not enable that construction of x from a proof by contradiction.

Proofs in type theory are sequences of instructions, which can be implemented and checked computationally, hence guaranteeing their validity.


## Homotopy type theory
Homotopy type theory studies a type theory universe with a single axiom (as opposed to the 8 axioms of ZF theory) called the **univalence axiom**. Loosely, it asserts that equivalent types are identical.




---

http://www.science4all.org/article/type-theory/
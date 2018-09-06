# μϵταματηϵματικς

The crucial philosophy of mathematics, intuitionism, dates back to Luitzen Brouwer and it asserts that the whole endeavor of mathematics is inevitably human-driven. Mathematics should not be about seeking an absolute Platonic truth, but about what, we as humans can construct. This effectively gave birth to __constructive mathematics__, in which all objects must be constructible, and a proof is not valid if it doesn't contain a witness i.e. it must construct an instance of that proof.

A controversial example of a difference between a classical ZF reasoning and a constructive one has to do with the law of excluded middle. This law asserts that what’s not false is true.

This leads to Euclid's famous proof by contradiction: to prove the existence of an object, it suffices to prove that the assumption of its non-existence implies a contradiction.

Although this sounds intuitively right, constructive mathematicians instead insist that such proofs do not yield an instance of the existing object, they don't construct the object, so they are pointless.

Not false is not necessarily synonym to true, although it sounds counter-intuitive, but, after all, the existence of a third alternative to provably true or provably false is the direct implication of Gödel’s first incompleteness theorem.

Type theory encapsulates each mathematical theory into a universe. One universe may satisfy the law of excluded middle. but another may not. And type theory is fit to describe both.

Type theory models objects and relations between them with types. A type can be e.g. a theorem and proving that theorem boils down to constructing a term of the theorem type.

Construction in type theory is not synonym of constructivism in Brouwer's intuitionism, which is what is usually meant by constructive mathematics.

Suppose that we have proved that assuming 10 is the biggest number leads to a contradiction. In other words, the claim that there is no number bigger than 10 is false. Then, Brouwer's intuitionism asserts that this proof cannot construct a number bigger than 10, and the proof is thus useless. But we can add the law of excluded middle to type theory as a function which (type-theoretically) constructs a number bigger than 10 from the proof by contradiction. Conversely though, type theory without excluded middle will not enable that construction of x from a proof by contradiction.

Homotopy type theory consists in studying a type theory universe with a single axiom (as opposed to the 8 axioms of ZF theory)! This axiom, which, by the way, is extremely elegant, is called the univalence axiom. Loosely, it asserts that equivalent types are identical.

Proofs in type theory are sequences of instructions, which can be implemented and checked computationally, hence guaranteeing their validity.


---

http://www.science4all.org/article/type-theory/
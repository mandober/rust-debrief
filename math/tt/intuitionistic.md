# Intuitionistic type theory

- Intuitionistic type theory
- constructive type theory
- Martin-Löf type theory
- Intuitionistic intensional type theory
- Intuitionistic extensional type theory
- Girard's paradox
- Intuitionistic predicative type theory
- constructive logic
- dependent types


In 1972, a Swedish mathematician and philosopher Per Martin-Löf founded a type theory that corresponded to predicate logic, called **intuitionistic type theory** or **constructive type theory**, or, as it is frequently refered to, **Martin-Löf type theory** (MTT).

MTT introduces dependent types and uses inductive types to represent unbounded data structures, such as natural numbers. One of its key features is that it unifies set theory and logic.


## Types and sets

Type theory studies types, which can be regarded as sets. The **term** in TT is analog to the notion of an element in set theory. We write $$a : A$$ to indicate that $$a$$ is a term of type $$A$$. And we write $$A : Type$$ to indicate that $$A$$ is a type.

A **judgement** is an act of knowing, or asserting a piece of knowledge about a mathematical object; it is a string of symbols that may or may not be provable from the rules of type theory. A judgement always contains the turnstile symbol, separating context from the conclusion. For example, a judgement that zero is a natural number is denoted by $$\vdash 0 : \mathbb{N}$$

Judgements can also involve asserting that something is a type, e.g. $$\vdash \mathbb{N} : Type$$
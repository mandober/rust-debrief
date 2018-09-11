# Sets

Although the theory of sets, initiated by Georg Cantor, constitutes the fundamental building blocks of all mathematics, with all mathematical objects resembling a set of some kind, the set is not formally defined , which raised many an eyebrow that expressed concerns about the sturdiness of a massive structure erected on such an undefined foundation, and shit. But, since the concept of a set is so ubiquitous and unavoidable, we must accept the term "set" as a primitive notion and let it be, sparing it from having to justify itself rigorously like everybody else.

> A set is a well-defined, unordered collection of distinct objects that share a common property; a set is considered as an object in its own right.

**Naïve set theory** is an approach to set theory which assumes the existence of a universal set, despite the fact that such an assumption leads to paradoxes. It is founded upon the rigid set of axioms of propositional and predicate logic.

**Axiomatic set theory** is a system that constrains the sets, which are allowed to be generated, by axioms. The best known systems of axiomatic set theory are **ZF**, Zermelo-Fraenkel set theory, and **ZFC**, which is ZF with the additional _Axiom of Choice_.


## ZFC
ZFC is an abbreviation for Zermelo-Fraenkel Set Theory with the Axiom of Choice. It is a system of axiomatic set theory upon which the whole of (at least conventional) mathematics can be based. Its basis consists of a system of Aristotelian logic, appropriately axiomatised, together with the Zermelo-Fraenkel axioms of Set Theory and the (controversial) Axiom of Choice.

Axioms of ZFC:
- **The Axiom of Extension**: Two sets are equal if and only if they have the same contents.
- **The Axiom of the Empty Set**: There exists a set that has no elements.
- **The Axiom of Pairing**: For any two sets, there exists a set to which only those two sets belong.
- **The Axiom of Subsets**: For every set and every condition, there corresponds a set whose elements are exactly the same as those elements of the original set for which the condition is true.
- **The Axiom of Union**: For every collection of sets, there exists a set that contains all the elements that belong to at least one of the sets in the collection.
- **The Axiom of Powers**: For each set, there exists a collection of sets that contains amongst its elements all the subsets of the given set.
- **The Axiom of Infinity**: There exists a set containing a set with no elements and the successor of each of its elements.
- **The Axiom of Replacement**: For any set S, there exists a set x such that, for any element y of S, if there exists an element z satisfying the condition P(y,z) (where P(y,z) is a propositional function), then such z appear in x.
- **The Axiom of Foundation**: For all non-null sets, there is an element of the set that shares no member with the set.
- **The Axiom of Choice**: For every set, we can provide a mechanism for choosing one element of any non-empty subset of the set.

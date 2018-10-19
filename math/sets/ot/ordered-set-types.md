# Types of Ordered Set


In order theory, different types of ordered set are studied, including:
- **Posets**, orderings in which not all pairs are comparable
- **Preorders**, generalization of poset allowing ties (represented as equivalences and distinct from incomparabilities)
- **Semiorders**, poset determined by comparison of numerical values, in which values that are too close to each other are incomparable; subfamily of posets with certain restrictions
- **Total orders**, orderings that specify order for all elements, all pairs are comparable
- **Well-orders**, total orders in which every non-empty subset has a least element
- **Well-quasi-orderings**, a class of preorders generalizing the well-orders
- **Weak orders**, generalizations of total orders allowing ties (represented either as equivalences or, in strict weak orders, as transitive incomparabilities)
- **Cyclic orders**, orderings in which triples of elements are either clockwise or counterclockwise
- **Lattices**, posets in which each pair of elements has a greatest lower bound (GLB) and a least upper bound (LUB).


---

Chains
- **Chain** is a totally ordered set or a totally ordered subset of a poset.
- **Chain complete** is a partially ordered set in which every chain has a least upper bound.
- **Antichain** is a poset in which no two elements are comparable, i.e. there are no two distinct elements x and y such that x ≤ y. In other words, the order relation of an antichain is just the identity relation.


Monotonicity
A monotonic function, or monotone function, is a function between ordered sets that preserves or reverses the given order.

- **Order-reversing**: a function $$f$$, **antitone**, between posets $$P$$ and $$Q$$ is a function for which, for all elements $$x, y$$ of $$P$$, $$x \le y$$ (in P) implies $$f(y) \le f(x)$$ (in Q).
- **Monotonically decreasing** is the name of such function in the presence of total orders (in math.Analysis).
- **Order-preserving** or **monotone** as the dual notion is called.

Bounds
- **Bounded poset** is one that has a least element and a greatest element.
- Poset is **bounded complete** if every of its subsets with some upper bound also has a least such upper bound. The dual notion is not common.
- **Closure operator** on the poset P is a function $$C:P \to P$$ that is monotone, idempotent, and satisfies $$C(x) \ge x$$ for all x in P.



https://en.wikipedia.org/wiki/Monotonicity
# Collections

Collections are grouping of elements in face of particular requirements, namely, their order and uniqueness.

With regards to order, collection can be ordered or unordered. With respect to uniqueness, whether the repeated elements are taken as distinct or as separate entities.

Elements   | Unordered| Ordered
-----------|----------|------------
Repeatable | Multiset | List
Distinct   | Set      | Ordered set


ud Set A         = {c, a, b, b, a, c} = {a, b, c}
ur Multiset C    = {c, a, b, b, a, c} = {a, a, b, b, c, c}
od Ordered set B = [c, a, b, b, a, c] = [c, a, b]
or List D        = [a, b, c]


## Multiset

A multiset (bag, mset) is a modification of the concept of a set that, unlike a set, allows for multiple instances for each of its elements.

D.Knuth lists other names that were proposed or used for this concept, including: list, bunch, bag, heap, sample, weighted set, collection, suite.

The positive integer number of instances, given for each element is called the __multiplicity__ of this element in the multiset.

As a consequence, an infinite number of multisets exist, which contain only elements `a` and `b`, but vary by the multiplicity of their elements:
- The set `{a, b}` contains only elements `a` and `b`, each having multiplicity 1, when `{a, b}` is seen as a multiset.
- In multiset `{a, a, b}`, the element `a` has multiplicity 2, and `b` has multiplicity 1.
- In multiset `{a, a, a, b, b, b}`, `a` and `b` both have multiplicity 3.

These objects are all different, when viewed as multisets, although they are the same set, since they all consist of the same elements.

As with sets, and in contrast to tuples, _order_ does not matter in discriminating multisets, so `{a, a, b}` and `{a, b, a}` denote the same multiset.

To distinguish between sets and multisets, a notation that incorporates square brackets is sometimes used: the multiset `{a, a, b}` is denoted as `[a, a, b]`.

The _cardinality_ of a multiset is constructed by summing up the multiplicities of all its elements. For example, in the multiset `{a, a, b, b, b, c}` the multiplicities of the members `a`, `b`, and `c` are respectively 2, 3, and 1, and therefore the cardinality of this multiset is 6.


One of the simplest and most natural examples is the multiset of prime factors of a number `n`. Here the underlying set of elements is the set of prime divisors of `n`. For example, the number 120 has the prime factorization $$120=2^{3}3^{1}5^{1}$$ which gives the multiset {2, 2, 2, 3, 5}.


## Sequence

A sequence is an enumerated collection of objects in which repetitions are allowed. Like a set, it contains members (elements, terms).

The number of elements (possibly infinite) is called the length of the sequence.

Unlike a set, the same elements can appear multiple times at different positions in a sequence, and order matters.

Formally, a sequence can be defined as a function whose domain is either the set of the natural numbers (for infinite sequences) or the set of the first n natural numbers (for a sequence of finite length n).

The position of an element in a sequence is its _index_ (rank); it is the natural number from which the element is the image. It depends on the context or a specific convention, if the first element has index 0 or 1. When a symbol has been chosen for denoting a sequence, the nth element of the sequence is denoted by this symbol with n as subscript; for example, the nth element of the Fibonacci sequence is generally denoted Fn.

The length of a sequence is defined as the number of terms in the sequence.

A sequence of a finite length n is also called an n-tuple. Finite sequences include the empty sequence ( ) that has no elements.

## Finite and infinite
Normally, the term _infinite_ sequence refers to a sequence that is infinite in one direction, and finite in the other - the sequence has a first element, but no final element. Such a sequence is called a __singly infinite__ sequence or a __one-sided infinite__ sequence when disambiguation is necessary.

In contrast, a sequence that is infinite in both directions, i.e. that has neither a first nor a final element is called a __bi-infinite__ sequence, __two-way infinite__ sequence, or __doubly infinite__ sequence. A function from the set Z of all integers into a set, such as for instance the sequence of all even integers ( …, −4, −2, 0, 2, 4, 6, 8… ), is bi-infinite. This sequence could be denoted $$(2n)_{n=-\infty}^{\infty}$$


## Increasing and decreasing
A sequence is said to be __monotonically increasing__, if each term is greater than or equal to the one before it.

For example, the sequence $$(a_n)_{n=1}^{\infty}$$ is monotonically increasing iff $$a_{n+1} \geq a_n$$ for all $$n \in N$$.

If each consecutive term is strictly greater thanthe previous term then the sequence is called __strictly monotonically increasing__.

A sequence is __monotonically decreasing__ if each consecutive term is less than or equal to the previous one.

A sequence is __strictly monotonically decreasing__ if each consecutive term is strictly less than the previous.

If a sequence is either increasing or decreasing it is called a __monotone sequence__. This is a special case of the more general notion of a _monotonic function_.

The terms _nondecreasing_ and _nonincreasing_ are often used in place of increasing and decreasing in order to avoid any possible confusion with strictly increasing and strictly decreasing, respectively.

## Bounded
If the sequence of real numbers $$(a_n)$$ is such that all the terms are less than some real number M, then the sequence is said to be __bounded from above__. 

In other words, this means that there exists $$M$$ such that for all $$n$$, $$an \le M$$.

Any such $$M$$ is called an __upper bound__.

Likewise, if, for some real m, $$an \ge m$$ for all n greater than some N, then the sequence is __bounded from below__ and any such m is called a __lower bound__. 

If a sequence is both bounded from above and bounded from below, then the sequence is said to be __bounded__.


## Subsequences
A subsequence of a given sequence is a sequence formed from the given sequence by _deleting_ some of the elements without disturbing the relative positions of the remaining elements.

For instance, the sequence of positive even integers (2, 4, 6, ...) is a subsequence of the positive integers (1, 2, 3, ...). The positions of some elements change when other elements are deleted. However, the relative positions are preserved.

Formally, a subsequence of the sequence $$(a_{n})_{n\in \mathbb{N}}$$ is any sequence of the form $$(a_{n_k})_{k \in \mathbb {N}}$$, where $$(n_k)_{k \in \mathbb {N}}$$ is a strictly increasing sequence of positive integers.

---

Generically, an **alternative set theory** (AST) is an alternative mathematical approach to the concept of set. It is a proposed alternative to the standard (axiomatic) set theory.

Some of the alternative set theories are:
- the theory of semisets
- the set theory New Foundations
- Positive set theory
- Internal set theory

A **semiset** is a proper class that is contained in a set.



---

https://en.wikipedia.org/wiki/Multiset
https://en.wikipedia.org/wiki/Sequence
https://en.wikipedia.org/wiki/Exact_sequence
https://en.wikipedia.org/wiki/N-tuple

# Counting theory



The mathematical theory of counting is also known as combinatorial analysis.

Counting theory includes:
- fundamental counting rules
- permutation rule
- combination rule

Fundamental counting principles
- The rule of sum
- The rule of product
- Inclusion–exclusion principle 
- Bijective proofs
- Pigeonhole principle
- Method of distinguished element
- Generating function
- Recurrence relations



In mathematics, the essence of counting a set and finding a result n, is that it establishes a one-to-one correspondence (or bijection) of the set with the set of numbers {1, 2, ..., n}. A fundamental fact, which can be proved by mathematical induction, is that no bijection can exist between {1, 2, ..., n} and {1, 2, ..., m} unless n = m; this fact (together with the fact that two bijections can be composed to give another bijection) ensures that counting the same set in different ways can never result in different numbers (unless an error is made). This is the fundamental mathematical theorem that gives counting its purpose; however you count a (finite) set, the answer is the same. In a broader context, the theorem is an example of a theorem in the mathematical field of (finite) combinatorics—hence (finite) combinatorics is sometimes referred to as "the mathematics of counting."

Many sets that arise in mathematics do not allow a bijection to be established with {1, 2, ..., n} for any natural number n; these are called infinite sets, while those sets for which such a bijection does exist (for some n) are called finite sets. Infinite sets cannot be counted in the usual sense; for one thing, the mathematical theorems which underlie this usual sense for finite sets are false for infinite sets. Furthermore, different definitions of the concepts in terms of which these theorems are stated, while equivalent for finite sets, are inequivalent in the context of infinite sets.

The notion of counting may be extended to them in the sense of establishing (the existence of) a bijection with some well-understood set. For instance, if a set can be brought into bijection with the set of all natural numbers, then it is called "countably infinite." This kind of counting differs in a fundamental way from counting of finite sets, in that adding new elements to a set does not necessarily increase its size, because the possibility of a bijection with the original set is not excluded. For instance, the set of all integers (including negative numbers) can be brought into bijection with the set of natural numbers, and even seemingly much larger sets like that of all finite sequences of rational numbers are still (only) countably infinite. Nevertheless, there are sets, such as the set of real numbers, that can be shown to be "too large" to admit a bijection with the natural numbers, and these sets are called "uncountable." Sets for which there exists a bijection between them are said to have the same cardinality, and in the most general sense counting a set can be taken to mean determining its cardinality. Beyond the cardinalities given by each of the natural numbers, there is an infinite hierarchy of infinite cardinalities, although only very few such cardinalities occur in ordinary mathematics (that is, outside set theory that explicitly studies possible cardinalities).

Counting, mostly of finite sets, has various applications in mathematics. One important principle is that if two sets X and Y have the same finite number of elements, and a function f: X → Y is known to be injective, then it is also surjective, and vice versa. A related fact is known as the pigeonhole principle, which states that if two sets X and Y have finite numbers of elements n and m with n > m, then any map f: X → Y is not injective (so there exist two distinct elements of X that f sends to the same element of Y); this follows from the former principle, since if f were injective, then so would its restriction to a strict subset S of X with m elements, which restriction would then be surjective, contradicting the fact that for x in X outside S, f(x) cannot be in the image of the restriction. Similar counting arguments can prove the existence of certain objects without explicitly providing an example. In the case of infinite sets this can even apply in situations where it is impossible to give an example.[citation needed]

The domain of enumerative combinatorics deals with computing the number of elements of finite sets, without actually counting them; the latter usually being impossible because infinite families of finite sets are considered at once, such as the set of permutations of {1, 2, ..., n} for any natural number n.
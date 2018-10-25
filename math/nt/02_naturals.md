# Natural number

- The set of natural numbers, $$\mathbb{N}$$, countably infinite set
- zero excluded: $$\mathbb{N^*}$$ or $$\mathbb{N}^+$$; zero included: $$\mathbb{N_0}$$


The **natural numbers** are numbers used for counting and ordering. Colloquially, words used for counting are **cardinal numbers** and words used for ordering are **ordinal numbers**.

There is no consensus on whether the set of natural numbers includes zero. In math, zero is usually excluded, while in CS it is included. However, with regard to definitions of a natural number, it is convenient to include zero (zero corresponds to empty set), a standard practice by set theorists and logicians.


## Notation
The set of natural numbers is denoted by $$\mathbb N$$. It is a countably infinite set, which can also be expressed by saying that the cardinality of the natural numbers set is aleph-naught, denoted by $$\aleph_0$$ (read aleph-naught or aleph-zero).

$$\mathbb N^+$$ explicitly denotes that a zero excluded.

$$\mathbb N^0$$ 

To be explicit about inclusion of zero, $$\mathbb N^0$$. And to denote that zero is excluded $$\mathbb N^+$$ is used most often.


## Definitions
Frege initially defined natural number in terms of set theory as the class of all sets that are in one-to-one correspondence with a particular set.

This definition turned out to lead to paradoxes, so it had to be modified by defining a natural number as a particular set, and any set that can be put into one-to-one correspondence with that set is said to have that number of elements.

The second class of definitions was introduced by Charles Sanders Peirce, refined by Richard Dedekind, and further explored by Giuseppe Peano - an approach that is now called Peano arithmetic.


## Definition in terms of sets

A natural number $$n$$ corresponds to a set that contains $$n$$ elements i.e. to a set of the cardinality $$n$$.

Natural numbers in terms of sets are defined inductively:
1. Zero is a natural number and it corresponds to empty set, $$0\equiv\varnothing$$
2. Natural number $$n$$ corresponds to a union of the previous number and a set containing previous number, i.e. $$n \equiv n-1 \cup \{n-1\}$$

- 0 corresponds to empty set, $$0 \equiv \varnothing$$.
- 1 corresponds to a union of zero and the set containing zero; that is,   
  union of empty set and the set containing empty set:   
  $$1 \equiv 0 \cup \{0\} = \varnothing \cup \{\varnothing\}$$, i.e. $$1 \equiv \{\varnothing\}$$
- 2 corresponds to a union of 1 and the set containing 1; that is,    
  union of the set cont. empty set and the set cont. the set cont. empty set:   
  $$2 \equiv 1 \cup \{1\} = \{\varnothing\} \cup \{\{\varnothing\}\}$$, i.e. $$2 \equiv \{ \varnothing, \{\varnothing\} \}$$


```
     0    1       2              3
0 =  ∅
1 = {∅ }
2 = {∅ , {∅} }
3 = {∅ , {∅} , {∅,{∅}} }
4 = {∅ , {∅} , {∅,{∅}} , {∅,{∅},{∅,{∅}}} }
...
n = n-1 ∪ {n-1}
```

Each number $$n$$ corresponds to a set that has $$n$$ elements.

A number contains all the previous numbers:
```
3 = { 0, 1, 2 }
2 ∪ 3 = {0,1} ∪ {0,1,2} = {0,1,2} = 3
```


## Peano axioms

**Peano axioms** are axioms for the natural numbers, presented by the 19th century Italian mathematician Giuseppe Peano, that have been used nearly unchanged in a number of metamathematical investigations, including research into fundamental questions of whether number theory is consistent and complete.

**Peano arithmetic** is based on an axiomatization of the properties of ordinal numbers: each natural number has a successor and every non-zero natural number has a unique predecessor.

The Peano axioms define the arithmetical properties of natural numbers, $$\mathbb {N}$$. The non-logical symbols for the axioms consist of a constant symbol $$0$$ and a unary function symbol $$S$$.

1. 0 is a natural number.
1. For every natural number x, x = x. That is, equality is reflexive.
1. For all natural numbers x and y, if x = y, then y = x. That is, equality is symmetric.
1. For all natural numbers x, y and z, if x = y and y = z, then x = z. That is, equality is transitive.
1. For all a and b, if b is a natural number and a = b, then a is also a natural number. That is, the natural numbers are closed under equality.
1. For every natural number n, S(n) is a natural number.
1. For all natural numbers m and n, m = n if and only if S(m) = S(n). That is, S is an injection.
1. For every natural number n, S(n) = 0 is false. That is, there is no natural number whose successor is 0.


The axiom 1 states that the constant 0 is a natural number.

The axioms 2-5 describe the equality relation. Since they are logically valid in first-order logic with equality, they are not considered to be part of "the Peano axioms" in modern treatments.

The axioms 6-8 define the arithmetical properties of the natural numbers. The naturals are assumed to be closed under a single-valued "successor" function S.



Peano's original formulation of the axioms used 1 instead of 0 as the "first" natural number. However, because 0 is the additive identity in arithmetic, most modern formulations of the Peano axioms start from 0.

Axioms 1, 6, 7, 8 define a unary representation of the intuitive notion of natural numbers: the number 1 can be defined as S(0), 2 as S(S(0)), etc. However, considering the notion of natural numbers as being defined by these axioms, axioms 1, 6, 7, 8 do not imply that the successor function generates all the natural numbers different from 0. Put differently, they do not guarantee that every natural number other than zero must succeed some other natural number.

The intuitive notion that each natural number can be obtained by applying successor sufficiently often to zero requires an additional axiom, which is sometimes called the axiom of induction.

If K is a set such that:
0 is in K, and
for every natural number n, n being in K implies that S(n) is in K,
then K contains every natural number.
The induction axiom is sometimes stated in the following form:

If φ is a unary predicate such that:
φ(0) is true, and
for every natural number n, φ(n) being true implies that φ(S(n)) is true,
then φ(n) is true for every natural number n.
In Peano's original formulation, the induction axiom is a second-order axiom. It is now common to replace this second-order principle with a weaker first-order induction scheme. There are important differences between the second-order and first-order formulations, as discussed in the section § Models below.
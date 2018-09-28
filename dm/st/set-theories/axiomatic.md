# Axiomatic set theory


**Axiomatic set theory** is a system that constrains the sets, which are allowed to be generated, by axioms. The best known systems of axiomatic set theory are **ZF**, Zermelo-Fraenkel set theory, and **ZFC**, which is ZF with the additional _Axiom of Choice_.



## "Naive Set Theory" by Paul R. Halmos

1. Axiom of extension    
  two sets are equal iff they have the same elements.
2. Axiom of unions    
  for every collection of sets there exists a set that contains all the elements that belong to at least one set of the given collection.
3. Axiom of specification    
  To every set A and to every condition S(x) there corresponds a set B whose elements are exactly those elements x of A for which S(x) holds.
4. Axiom of pairing    
  For any two sets there exists a set that they both belong to.
5. Axiom of powers    
  mFor each set there exists a collection of sets that contains among its elements all the subsets of the given set.
6. Axiom of infinity    
  There exists a set containing 0 and the successor of each of its elements.
7. Axiom of substitution    
  If S(a,b) is a sentence such that for each a in set A the set {b: S(a,b)} can be formed, then there exists a function F with domain A such that F(a) = {b:S(a,b)} for each a in A (Anything intelligent that one can do to the elements of a set yields a set).
8. Axiom of choice    
  The Cartesian product of a non-empty family of non-empty sets is non-empty


Defining natural numbers:
- a natural number is defined as a cardinality of a set
- `succ(n) = n U {n}`
- every natural number is to be equal to the set of its predecessors

First few von Neumann ordinals:

```
el#   1     2     3         4
0 = {                                      } == ∅
1 = { ∅                                    } == {0}
2 = { ∅ ,  {∅}                             } == {0,1}
3 = { ∅ ,  {∅} , {∅,{∅}}                   } == {0,1,2}
4 = { ∅ ,  {∅} , {∅,{∅}} , {∅,{∅},{∅,{∅}}} } == {0,1,2,3}

|∅| = 0
|{0}| = 1
|{0,1}| = 2
|{0,1,2}| = 3
|{0,1,2,3}| = 4

P(∅)    = {∅}         ∅ ∈ P(∅)   P(∅) = 1
P(P(∅)) = {∅, {∅}}    ∅ ∈ P(∅)   P(∅) = 1


∩ ∉ ∈
```


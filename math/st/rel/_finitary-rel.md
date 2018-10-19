A **finitary** relation has a finite number of "_places_". For example, in set theory, a binary (two "places") relation is a relation on two sets.

In the set theory and logic, a **relation** is a property that assigns **truth values** to **k-tuples** of individuals; as an example, 2-tuple, when $$k=2$$, is called an **ordered pair**. Typically, this property describes a possible connection between the components of a $$k$$-tuple.

For a given set of k-tuples, a truth value is assigned to each $$k$$-tuple according to whether the property does or does not hold.

An example of a **ternary relation** (between three objects) is "_X was introduced to Y by Z_", where $$(X,Y,Z)$$ is a 3-tuple of persons.

> Relations with a finite number of places are **finitary relations**.

The variable $$k$$ giving the number of "places" in the relation (2 for binary, 3 for ternary relation) is a non-negative integer, called the **relation's arity** (adicity, dimension). A relation with $$k$$ places is variously called a $$k$$-ary.



### Arity

There is only one 0-tuple, called the **empty tuple**, denoted by `()`, so there are two 0-place relations:
1. the one that always holds
2. the one that never holds

They are sometimes useful for constructing the base case of an induction argument.

**1-place** relation is a **unary relation** or **property**. For instance, any set can be viewed as a collection of objects that posses some property (e.g. being odd, in case of integers).

**2-place** relation is a **binary relation**. Binary relations are the most common. Examples include:
- equality, $$x=y$$
- inequality, $$x\not = y$$
- greater than, $$x\gt y$$
- a divisor of, $$x \mid y$$
- set membership, $$1\in \mathbb {N}$$

> A $$k$$-ary relation is a straightforward generalization of a binary relation.


### Definition of relation (definition 1)

A relation $$L$$ over the sets $$X_1, \dots, X_k$$ is a subset of their Cartesian product, written $$L \subseteq X_1 \times \dots \times X_k$$.

Relations are classified according to the number of sets in the defining Cartesian product i.e. according to the number of terms:
- $$Lu$$ denotes a **unary relation** or **property**;
- $$Luv$$ or $$uLv$$ denote a **homogeneous** relation when $$X_1 = X_2$$ and a **heterogeneous** relation when $$X_1 \not = X_2$$.
- $$Luvw$$ denotes a **ternary** relation;
- $$Luvwx$$ denotes a **quaternary** relation.

Relations with more than 4 terms are usually referred to as k-ary or n-ary, for example, "a 5-ary relation". A k-ary relation is simply a set of k-tuples.


### Definition of relation (definition 2)

Using recursion, relations can also be defined in the terms of binary relation.

The second definition makes use of an idiom that is common in mathematics, stipulating that "such and such is an n-tuple" in order to ensure that "such and such" a mathematical object is determined by the specification of n component mathematical objects.

In the case of a relation L over k sets, there are k + 1 things to specify, namely, the k sets plus a subset of their Cartesian product. In the idiom, this is expressed by saying that L is a (k + 1)-tuple.









---

https://en.wikipedia.org/wiki/Finitary_relation

# Reflexivity

A binary relation is **reflexive** if every element of the set is related to itself.

A relation $$R \subseteq X^2$$ is reflexive iff $$\forall x : xRx$$.

In other words, a relation $$R$$ to the set $$\mathbb{N^2}$$ is reflexive if it contains __all the diagonal (identity) pairs__ e.g. $$R=\{(1,1),(2,2),(3,3),\dots\}$$.

If it contains __no diagonal pairs__ then the relation is __irreflexive__.

A relation $$R \subseteq X^2$$ is irreflexive iff $$\forall x : x\not{\,R} 
x$$.



**Reflexive**: relation R on set A is called Reflexive if ∀a∈A is related to a (aRa holds)
Example: relation R={(a,a),(b,b)} on set X={a,b} is reflexive.

**Irreflexive** relation R on set A is called Irreflexive if no a∈A is related to a (aRa does not hold).
Example
The relation R={(a,b),(b,a)} on set X={a,b} is irreflexive.

---

# Reflexivity

Properties
- reflexive
- non-reflexive
- irreflexive (anti-reflexive)
- non-irreflexive
- coreflexive
- non-coreflexive
- quasi-refexive


**Reflexive**    
Every element of the set **must be** related to itself.
A relation $$R \subseteq X^2$$ is reflexive iff 
$$\forall x \in X:(x,x) \in R$$,
i.e. if every element in the set X is related to itself, 
$$\forall x \in X:xRx$$.

## Examples

1. Relation "_is equal to_", i.e. `=`, on the set of real numbers is reflexive since every real number is equal to itself. Replacing $$R$$ with `=` we see that $$xRx$$ becomes $$x=x$$, which holds for any $$x$$.

Other examples include:
- "_is greater than or equal to_"
- "_is less than or equal to_"
- "_is a subset of_" (set inclusion)
- "_divides_" (divisibility)

2. Let $$X=\{1,2,3,4\}$$. For relation $$R$$ to be reflefive, it must contain all the identity pairs, $$R=\{(1,1), (2,2), (3,3), (4,4)\}$$ of the set $$X^2$$. It can additionally contain any extra pairs, but missing a single id pair will make this relation __non-reflexive__. If the relation contains **only** id pairs then this is **identity** relation.


## Features

- empty relation is reflexive
- universal relation is reflexive
- every identity relation is reflexive


---

> A binary relation is **irreflexive**, or **anti-reflexive**, if no element is related to itself.

Irreflexive relation prohibits membership for identity pairs. In an endorelation, $$X^2$$, on a set $$X=\{1,2,3\}$$, the relation is irreflexive if it doesn't contain any of the identity pairs i.e. $$(1,1)$$, $$(2,2)$$ or $$(3,3)$$. If it contains even a single id pair, it is __non-irreflexive__. If it contains all the id pairs it is reflexive.

An example is the "_greater than_" relation on the set of real numbers, since no real number is greater than itself.

Not every relation which is not reflexive is irreflexive; it is possible to define relations where some elements are related to themselves but others are not.

For example, the binary relation "_the product of x and y is even_" is reflexive on the set of even natural numbers, irreflexive on the set of odd natural numbers, and neither reflexive nor irreflexive on the set of natural numbers.

Examples of irreflexive relations include:
- $$\not =$$, "_is not equal to_"
- $$\lt$$, "_is less than_"
- $$\gt$$, "_is greater than_"
- $$\subset$$, "_is a proper subset of_"

---

A binary relation is coreflexive

An example of a coreflexive relation is the relation on integers in which each odd number is related to itself and there are no other relations.

The equality relation is the only example of a both reflexive and coreflexive relation.

Any coreflexive relation is a subset of the identity relation.

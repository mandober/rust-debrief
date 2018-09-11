# Zermelo–Fraenkel set theory

Zermelo–Fraenkel set theory is an axiomatic theory of sets free of paradoxes. Abbreviated as `ZF` when it refers to the Zermelo–Fraenkel set theory with the axiom of choice excluded, otherwise known as `ZFC`. The axiom of choice is independent of other axioms of ZF set theory. ZFC is one of the widely accepted candidates for the foundation of mathematics.

- named after mathematicians Ernst Zermelo and Abraham Fraenkel
- theory proposed in the early XX century
- C in ZFC stands for "choice"

Includes 10 axioms:
1. Axiom of Extensionality
2. Axiom of Regularity
3. Axiom Schema of Specification
4. Axiom of Pairing
5. Axiom of Union
6. Axiom Schema Of Replacement
7. Axiom of Infinity
8. Axiom of Power Set
9. Well-ordering theorem
10. Axiom of Choice



1. **Axiom of extensionality**

_Two sets are equal (are the same set) if they have the same elements_.

$$\forall x\forall y[\forall z(z\in x\iff z\in y)\to x=y]$$

The converse of this axiom follows from the substitution property of equality. If the background logic does not include equality "=", x=y may be defined as an abbreviation for the following formula:[4]

{\displaystyle \forall z[z\in x\Leftrightarrow z\in y]\land \forall w[x\in w\Leftrightarrow y\in w].}
In this case, the axiom of extensionality can be reformulated as

{\displaystyle \forall x\forall y[\forall z(z\in x\Leftrightarrow z\in y)\Rightarrow \forall w(x\in w\Leftrightarrow y\in w)],}
which says that if x and y have the same elements, then they belong to the same sets.


2. **Axiom of regularity** 

Also called **Axiom of foundation**

_Every non-empty set x contains a member y such that x and y are disjoint sets_.

{\displaystyle \forall x[\exists a(a\in x)\Rightarrow \exists y(y\in x\land \lnot \exists z(z\in y\land z\in x))].}[6]
This implies, for example, that no set is an element of itself and that every set has an ordinal rank.


3. **Axiom schema of Specification**

Also called __Axiom Schema Of Separation Or Of Restricted Comprehension__

_Subsets are commonly constructed using set builder notation_.

For example, the even integers can be constructed as the subset of the integers \mathbb {Z}  satisfying the congruence modulo predicate x\equiv 0{\pmod {2)):

\{x\in \mathbb {Z} :x\equiv 0{\pmod {2))\}.
In general, the subset of a set z obeying a formula \phi (x) with one free variable x may be written as:

\{x\in z:\phi (x)\}.

The axiom schema of specification states that this subset always exists (it is an axiom schema because there is one axiom for each \phi ). Formally, let \phi  be any formula in the language of ZFC with all free variables among {\displaystyle x,z,w_{1},\ldots ,w_{n)) (y is not free in \phi ). Then:

\forall z\forall w_{1}\forall w_{2}\ldots \forall w_{n}\exists y\forall x[x\in y\Leftrightarrow (x\in z\land \phi )].
Note that the axiom schema of specification can only construct subsets, and does not allow the construction of sets of the more general form:

\{x:\phi (x)\}.
This restriction is necessary to avoid Russell's paradox and its variants that accompany naive set theory with unrestricted comprehension.

In some other axiomatizations of ZF, this axiom is redundant in that it follows from the axiom schema of replacement and the axiom of the empty set.

On the other hand, the axiom of specification can be used to prove the existence of the empty set, denoted \varnothing , once at least one set is known to exist (see above). One way to do this is to use a property \phi  which no set has. For example, if w is any existing set, the empty set can be constructed as

\varnothing =\{u\in w\mid (u\in u)\land \lnot (u\in u)\}.
Thus the axiom of the empty set is implied by the nine axioms presented here. The axiom of extensionality implies the empty set is unique (does not depend on w). It is common to make a definitional extension that adds the symbol \varnothing  to the language of ZFC.


4. **Axiom of pairing**

_If x and y are sets, then there exists a set which contains x and y as elements_.

\forall x\forall y\exists z(x\in z\land y\in z).
The axiom schema of specification must be used to reduce this to a set with exactly these two elements. The axiom of pairing is part of Z, but is redundant in ZF because it follows from the axiom schema of replacement, if we are given a set with at least two elements. The existence of a set with at least two elements is assured by either the axiom of infinity, or by the axiom schema of specification and the axiom of the power set applied twice to any set.



5. **Axiom of union**

_The union over the elements of a set exists_.

For example, the union over the elements of the set \{\{1,2\},\{2,3\}\} is \{1,2,3\}.

The axiom of union states that for any set of sets {\mathcal {F)) there is a set A containing every element that is a member of some member of {\mathcal {F)):

\forall {\mathcal {F))\,\exists A\,\forall Y\,\forall x[(x\in Y\land Y\in {\mathcal {F)))\Rightarrow x\in A].
Although this formula doesn't directly assert the existence of {\displaystyle \cup {\mathcal {F))}, the set {\displaystyle \cup {\mathcal {F))} can be constructed from A in the above using the axiom schema of specification:

{\displaystyle \cup {\mathcal {F)):=\{x\in A:\exists Y(x\in Y\land Y\in {\mathcal {F)))\}.}


6. **Axiom schema of replacement**

_The axiom schema of replacement asserts that the image of a set under any definable function will also fall inside a set_.


Formally, let \phi  be any formula in the language of ZFC whose free variables are among {\displaystyle x,y,A,w_{1},\dotsc ,w_{n)), so that in particular B is not free in \phi . Then:

{\displaystyle \forall A\forall w_{1}\forall w_{2}\ldots \forall w_{n}{\bigl [}\forall x(x\in A\Rightarrow \exists !y\,\phi )\Rightarrow \exists B\ \forall x{\bigl (}x\in A\Rightarrow \exists y(y\in B\land \phi ){\bigr )}{\bigr ]}.}
In other words, if the relation \phi  represents a definable function f, A represents its domain, and f(x) is a set for every x\in A, then the range of f is a subset of some set B. The form stated here, in which B may be larger than strictly necessary, is sometimes called the axiom schema of collection.


7. **Axiom of infinity**

First few von Neumann ordinals:
0	= { }	= ∅
1	= { 0 }	= {∅}
2	= { 0, 1 }	= { ∅, {∅} }
3	= { 0, 1, 2 }	= { ∅, {∅} , {∅, {∅)) }
4	= { 0, 1, 2, 3 }	= { ∅, {∅}, {Ø, {Ø)), {Ø, {Ø}, {Ø, {Ø))} }

Let S(w) abbreviate {\displaystyle w\cup \{w\)), where w is some set. (We can see that \{w\} is a valid set by applying the Axiom of Pairing with {\displaystyle x=y=w} so that the set z is {\displaystyle \{w\))).

_Then there exists a set X such that the empty set \varnothing  is a member of X and, whenever a set y is a member of X, then S(y) is also a member of X_.

{\displaystyle \exists X\left[\varnothing \in X\land \forall y(y\in X\Rightarrow S(y)\in X)\right].}

More colloquially, there exists a set X having infinitely many members. (It must be established, however, that these members are all different, because if two elements are the same, the sequence will loop around in a finite cycle of sets. The axiom of regularity prevents this from happening.) The minimal set X satisfying the axiom of infinity is the von Neumann ordinal ω, which can also be thought of as the set of natural numbers \mathbb {N} .


8. **Axiom of power set**

By definition a set z is a subset of a set x if and only if every element of z is also an element of x:

(z\subseteq x)\Leftrightarrow (\forall q(q\in z\Rightarrow q\in x)).

The Axiom of Power Set states that for any set x, there is a set y that contains every subset of x:

\forall x\exists y\forall z[z\subseteq x\Rightarrow z\in y].
The axiom schema of specification is then used to define the power set P(x) as the subset of such a y containing the subsets of x exactly:

P(x)=\{z\in y:z\subseteq x\}
Axioms 1–8 define ZF. Alternative forms of these axioms are often encountered, some of which are listed in Jech (2003). Some ZF axiomatizations include an axiom asserting that the empty set exists. The axioms of pairing, union, replacement, and power set are often stated so that the members of the set x whose existence is being asserted are just those sets which the axiom asserts x must contain.


The following axiom is added to turn ZF into ZFC:

9. **Well-ordering theorem**

_For any set X, there is a binary relation R which well-orders X. This means R is a linear order on X such that every nonempty subset of X has a member which is minimal under R_.

\forall X\exists R(R\;{\mbox{well-orders))\;X).

Given axioms 1–8, there are many statements provably equivalent to axiom 9, the best known of which is the axiom of choice.


**Axiom of Choice (AC)**

_Let X be a set whose members are all non-empty. Then there exists a function f from X to the union of the members of X, called a "choice function", such that for all Y ∈ X one has f(Y) ∈ Y_.

Since the existence of a choice function when X is a finite set is easily proved from axioms 1–8, AC only matters for certain infinite sets. AC is characterized as nonconstructive because it asserts the existence of a choice set but says nothing about how the choice set is to be "constructed". Much research has sought to characterize the definability (or lack thereof) of certain sets whose existence AC asserts.

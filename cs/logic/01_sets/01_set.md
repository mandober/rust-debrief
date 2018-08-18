# Sets

<!-- TOC -->

- [Set membership](#set-membership)
- [Extensionality](#extensionality)

<!-- /TOC -->

Sets are the most fundamental building blocks of mathematical objects. Almost every mathematical object can be seen as a set of some kind. The theory of sets, initiated by Georg Cantor, constitutes the basis of almost all modern mathematics.

The set concept is one of the basics so the term "set" must be accepted as a primitive notion, without formal definition. Informally, a set is a __collection__ (aggregate, class, totality, family) of objects of any kind, disregarding their repetion and the order in which they were specified in.

A set can be denoted by listing its elements between the curly braces, e.g. $$\{2,4,6,8\}$$, or it can be described by a property that all of its members share, e.g. $$\{x: x \text{ is a superhero}\}$$, where the $$x$$ stands for the property that it has to have in order to be counted among the elements of the set.

Sets are usually denoted with an uppercase letter; the set $$\{a,b,c\}$$ may be declared as $$A=\{a,b,c\}$$, and now we can use $$A$$ to stand in for the set $$\{a,b,c\}$$.


## Set membership

The objects belonging to a set are called the __elements__ or __members__. The fact that $$a$$ is an element of the set $$A$$ is denoted by $$a \in A$$. Expression $$a,b \in A$$ indicates the set membership for several things at once. An element that is not a member of the set $$A$$ is denoted by $$d \notin A$$

The order in which the elements of a set are specified or their repetition is of no consequence with regards to sets, e.g. this is the same set: $$\{a,b,c\}=\{b,c,b,a,a\}$$

Once a set has been formed, it is a __new entity__, different from any of its elements. A set containing one element is not the same as that element alone: $$p \neq \{p\}$$.


## Extensionality

Two sets are equal if they contain the same elements. If $$X$$ and $$Y$$ are sets, then $$X = Y$$ if and only if every element of $$X$$ is also an element of $$Y$$, and vice versa.

Extensionality gives us a way for showing that sets are identical by showing that whenever $$x\in X$$ then also $$x\in Y$$, and whenever $$y\in Y$$ then also $$y\in X$$.

Sets need not contain just mathematical entities as elements, anything can be a member of a set, even other sets. A set whose elements are other sets are __collections of sets__. If an element $$a$$ is in the set $$A$$ and the set $$A$$ is in the set $$\mathscr{M}$$, that doesn't mean the element $$a$$ is in the set $$\mathscr{M}$$.

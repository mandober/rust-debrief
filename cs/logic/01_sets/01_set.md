# Sets

- sets
- set notation
- set elements
- set equality
- fundamental sets
- empty set
- cardinality
- infinite and finite sets
- enumerable sets


Sets are the most fundamental building blocks of mathematical objects. Almost every mathematical object can be seen as a set of some kind. In logic, as in other parts of mathematics, sets and set-theoretical talk is ubiquitous.


## Introduction to sets

The theory of sets, initiated by Georg Cantor (1845–1918), constitutes the basis of almost all modern mathematics. A set is often described as a __collection__ (aggregate, class, totality, family) of objects of any specified kind, but the set concept is one of the basics so the term _set_ must be accepted as a primitive notion, without definition.

The order of the elements or their repetition is of no consequence to sets, $$\{a,b,\{\}\}=\{b,\{\},b,\varnothing,a\}$$.

__Extensionality__: Two sets are __equal__ if they contain exactly the same elements. if $$X$$ and $$Y$$ are sets, then $$X = Y$$ iff every element of $$X$$ is also an element of $$Y$$, and vice versa. Extensionality gives us a way for showing that sets are identical by showing that whenever $$x\in X$$ then also $$x\in Y$$, and whenever $$y\in Y$$ then also $$y\in X$$.

Sets are usually denoted with uppercase letters: the set $$\{2,4,6,8\}$$ may be declared as $$A=\{2,4,6,8\}$$ and then use $$A$$ to stand for $$\{2,4,6,8\}$$.

The objects belonging to a set are called its __elements__ or __members__. To express that $$2$$ is an element of the set $$A$$, we write $$2 \in A$$. Expressions like $$2,4,6 \in A$$ indicate that several things are in a set. If an element is not in a set: $$3 \notin A$$. A set of one element is not the same as that element alone: $$p \neq \{p\}$$.

Once a set has been formed, it is a __new entity__, different from any of its elements. A newly formed set may be an element of some other set. Sets whose elements are other sets are __collections of sets__. If an element $$a$$ is in the set $$A$$ and set $$A$$ is in the set $$\mathscr{M}$$, that doesn't imply the element $$a$$ is in the set $$\mathscr{M}$$: $$x \in A \land A \in \mathscr{M} \not \rightarrow x \in A$$.

We are mainly concerned with sets whose elements are mathematical entities, such as numbers, points, functions, etc., but sets need not have just numbers as elements. Sets can even have other sets as elements.


### Fundamental sets
Some sets are so prevalent they have a special symbol assigned for them.   
- $$\mathbb{B}$$ is the set of boolean (truth) values: $$\{\bot,\top\}$$ or $$\{F,T\}$$ or $$\{0,1\}$$
- $$\mathbb{N}$$ is the set of natural numbers, either $$\{1,2,3,\dots\}$$ or $$\{0,1,2,3,\dots\}$$
- $$\mathbb{Z}$$ is the set of integers $$\{-2,-1,0,1,2,\dots\}$$ and $$\mathbb{Z^+}=\{1,2,3,\dots\}$$
- $$\mathbb{Q}$$ is the set of rational numbers: $$\{p/q : p \in\mathbb{Z}, q\in \mathbb{N}\land q\neq 0\}$$
- $$\mathbb{R}$$ is the set of real numbers. $$\mathbb{N}\subseteq \mathbb{Z}\subseteq \mathbb{Q}\subseteq \mathbb{R}$$
- $$\mathbb{C}$$ is the set of complex numbers: $$\{a,b\,i:a,b\in\mathbb{R}\}$$
- $$\varnothing$$ is the empty set.


### Cardinality
__Infinite__ set has infinitely many elements; otherwise it is a __finite__ set. Infinite sets: $$\{1,2,3,\dots\}$$, or $$\{\dots,1,2,3,\dots\}$$. A sets with terminal member: $$\{1,2,3, \dots,100\}$$, or $$\{1,2,\dots,n\}$$.

If $$X$$ is a finite set, its __cardinality__ or size is the number of elements it has, and this number is denoted as $$|X|$$.


### Empty set
Another special set, regarded as an existing object, is the __empty set__, i.e. a set that has no elements: $$\{\}$$. It is denoted as $$\varnothing$$, so $$\{\}=\varnothing$$. The empty set is the only set whose cardinality is zero: $$|\varnothing|=0$$. Note: $$\varnothing=\{\}$$ and $$\{\varnothing\}=\{\{\}\}$$, so $$\varnothing\neq \{\varnothing\} $$.
- empty set is a subset of any set
- empty set is a proper subset of any non-empty set

### Set comprehensions
A special notation called __set-builder notation__ is used to describe sets that are too big or complex to list between braces. Its general syntax is $$X=\{exp:rule\}$$. For example, $$E=\{2n:n\in \mathbb{Z}\}$$ which builds the infinite set of even integers, which can be read as "E equals the set of all things of form $$2n$$, such that $$n$$ is an element of $$\mathbb{Z}$$".



## The Cartesian Product

- An __ordered pair__ is a list $$(x, y)$$ of two things $$x$$ and $$y$$
- __Cartesian product__ of two sets $$A$$ and $$B$$ is another set, denoted as $$A\times{B}$$ and defined as     
$$A\times{B} = \{(a,b) : a\in A, b\in B\}$$
- Thus $$A\times{B}$$ is a set of ordered pairs of elements from $$A$$ and $$B$$.
- If $$A$$ and $$B$$ are finite sets, then $$|A|\times |B| = |A|*|B|$$



## Indexed sets

$$$
\bigcap_{i=0}^n A_i = A_0 \cap A_1 \cap \dots \cap A_n
$$$

$$$
\bigcap^{n+1}_{i=0} A_i = \bigcap_{i=0}^n A_i \cap A_{n+1}
$$$


$$$
\bigcup_{i=0}^n A_i = A_0 \cup A_1 \cup \dots \cup A_n
$$$


## Russell's Paradox

Sets may be elements of other sets, but can a set be a member of itself? For instance, since all sets form a collection of objects, can we collect them into a single set - the set of all sets? And it, being a set, would be an element of the set of all sets.

Russell's paradox arises when we consider the property of not having itself
as an element.

For example, set of natural numbers is not in the set of natural numbers, $$\mathbb{N}\notin \mathbb{N}$$, since it is a set, not a natural number. Powerset of the set of real numbers is not an element of itself, $$\mathfrak{P}(\mathbb{R})\notin \mathfrak{P}(\mathbb{R})$$, since it is a set of sets of real numbers, not a set of real numbers.

What if we suppose that there is a set of all sets that do not have themselves as an element? Does $$R=\{x : x\notin x\}$$ exist?

If R exists, it makes sense to ask if $$R\in R$$ or not - it must be either $$R\in R$$ or $$R\notin R$$.

Suppose the former is true, i.e. $$R\in R$$ was defined as the set of all sets that are not elements of themselves, and so if $$R\in R$$, then $$R$$ does not have this defining property of $$R$$. But only sets that have this property are in $$R$$, hence, $$R$$ cannot be an element of $$R$$, i.e. $$R\notin R$$. But $$R$$ can't both be and not be an element of $$R$$, so we have a contradiction.

Since the assumption that $$R\in R$$ leads to a contradiction, we have $$R\notin R$$. But this also leads to a contradiction! If $$R\notin R$$, it does have the defining property of $$R$$, and so it would be an element of $$R$$ just like all the other non-self-containing sets. And again, it can't both not be and be an element of $$R$$.

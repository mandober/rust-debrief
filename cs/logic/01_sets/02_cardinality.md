# Set cardinality

<!-- TOC -->

- [Fundamental sets](#fundamental-sets)
- [Empty set](#empty-set)
- [Set comprehensions](#set-comprehensions)
- [The Cartesian Product](#the-cartesian-product)
- [Indexed sets](#indexed-sets)
- [Russells Paradox](#russells-paradox)

<!-- /TOC -->

Sets come in two sizes: finite and infinite. An __infinite set__ has infinitely many elements. This is denoted by using elipsis, as in: $$\{1,2,3,\dots\}$$, or, if a set is infinite on both ends, $$\{\dots,1,2,3,\dots\}$$.

A __finite set__ has an finite number of elements, but even so a large sets can be denoted by using elipsis and a terminal member, such as $$\{1,2,3, \dots,100\}$$, and $$\{1,2,\dots,n\}$$.

If $$X$$ is a finite set, its __cardinality__, i.e. the number of its elements, is denoted by $$|X|$$. Given a set $$A=\{1,2,\dots,10\}$$, its cardinality is $$|A|=10$$.


## Fundamental sets

{% include "./fundamental-sets.md" %}
<!-- @import "./fundamental-sets.md" -->


Even though they are all infinite sets, it turns out that $$\mathbb{N}, \mathbb{Z}, \mathbb{Q}$$ have the same number of elements, while $$\mathbb{R}$$ has a whole lotta more. The former three are __enumerable__ and infinite whereas the latter R is __non-enumerable__ and infinite.


## Empty set

Another special set, regarded as an existing object, is the __empty set__, i.e. a set that has no elements: $$\{\}$$. It is denoted as $$\varnothing$$, so $$\{\}=\varnothing$$. The empty set is the only set whose cardinality is zero: $$|\varnothing|=0$$. Note: $$\varnothing=\{\}$$ and $$\{\varnothing\}=\{\{\}\}$$, so $$\varnothing\neq \{\varnothing\} $$.
- empty set is a subset of any set
- empty set is a proper subset of any non-empty set

$$\varnothing$$ is the empty set: $$\quad\varnothing = \emptyset = \{\}$$



## Set comprehensions
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



## Russells Paradox

{% include "./russells-paradox.md" %}
<!-- @import "./russells-paradox.md" -->

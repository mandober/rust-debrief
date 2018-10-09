

> If all elements of the set $$X$$ are also elements of the set $$Y$$, then $$X$$ is the __subset__ of $$Y$$, denoted by $$X \subseteq Y$$. At the same time, set $$Y$$ is a **superset** of $$X$$, denoted by $$Y \supseteq X$$.

The relation $$\subseteq$$ is the __inclusion relation__.

A set $$X$$ is a __proper subset__ of a set $$Y$$, denoted by $$X\subset Y$$ if set $$Y$$ has additional elements besides those that are also in set $$X$$. That is if every element of $$X$$ is an element of $$Y$$ and $$|X| < |Y|$$.

For example, $$X=\{1,2\}$$ and $$Y=\{1,2,3,4\}$$. Here, $$X\subseteq Y$$, but, more precisely, $$X$$ is a proper subset of $$Y$$ i.e. $$X\subset Y$$.

> Every set is a subset of itself

$$$
\{a,b,c\} \subseteq \{a,b,c\}
$$$

> The empty set is subset of every set.

$$$
\varnothing \subseteq X
$$$


## Powerset
If a set $$\mathcal{P}$$ contains all the possible subsets of a set $$X$$ (including the empty set), then  $$\mathcal{P}$$ is a **powerset** of $$X$$, denoted as $$\mathcal{P}(X)$$.

For example, if $$X=\{a,b\}$$, then $$\mathcal{P}(X) =\{\{a\},\{b\},\{ab\},\{\varnothing\}\}$$

The cardinality of a power set of a set $$X$$ is $$2^{|X|}$$.




## The empty set
A set containing no elements, $$\{\}$$, is the __empty set__, and it is nevertheless regarded as an existing object, usually denoted by $$\varnothing$$, so $$\{\}=\varnothing = \emptyset$$

The empty set is the only set whose cardinality is zero, $$|\varnothing|=0$$.

$$$
\varnothing = \{x:x\not\in X\}
$$$

> The empty set is a subset of any set.

$$$
\varnothing \subseteq X , \ \ \text{where X is any set}
$$$

> The empty set is a proper subset of any non-empty set.

$$$
\varnothing \subset X , \ \ \text{where X is a non-empty set}
$$$

Note: $$\varnothing=\{\}$$, $$\{\varnothing\}=\{\{\}\}$$, so $$\varnothing\neq \{\varnothing\} $$.


The __empty set__ is the set that has no elements. It is denoted as $$\varnothing$$ and it represents $$\{\}$$. The empty set is the only set whose cardinality is zero.


- $$\varnothing=\{\}\neq\{\{\}\}=\{\varnothing\} $$
- the empty set is regarded as an existing object.
- empty set is a subset of any set
- empty set is a proper subset of any non-empty set


# Indexed sets

$$$
\bigcap_{i=0}^n A_i = A_0 \cap A_1 \cap \dots \cap A_n
$$$

$$$
\bigcap^{n+1}_{i=0} A_i = \bigcap_{i=0}^n A_i \cap A_{n+1}
$$$


$$$
\bigcup_{i=0}^n A_i = A_0 \cup A_1 \cup \dots \cup A_n
$$$


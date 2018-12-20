- a set contining no elements is the **empty** set.
- a set containing 1 element is a **singleton** set, or unit set
- a set containing 2 elements is a **pair**, unordered pair, 2-tuple
- a set containing $$k$$ elements is a **k-tuple**

A set $$X$$ is called an __ordered pair__ if $$X = \{\{x\},\{x,y\}\}$$ for some $$x,y$$. This is commonly abbreviated by writing an ordered pair as $$\langle x,y \rangle$$ or $$\{(x,y)\}$$.

Two ordered pairs are equal if their respective components are the same: let $$(a, b)$$ and $$(c, d)$$ be ordered pairs; then $$(a, b) = (c, d)$$ iff $$a = c$$ and $$b = d$$.

An ordered pair $$\langle x, x \rangle$$ is $$\{\{x\},\{x, x\}\}$$ by definition, which first collapses into $$\{\{x\},\{x\}\}$$ and then into $$\{\{x\}\}$$. Therefore $$\langle x, x \rangle = \{\{x\}\}$$.

https://en.wikipedia.org/wiki/Tuple


A set containing two elements may be referred to as an **unordered pair**.

A set $$X$$ is called an __ordered pair__ if $$X = \{\{x\},\{x,y\}\}$$ for some $$x,y$$. It is common to abbreviate the right hand expression by writing an ordered pair as  $$\langle x,y \rangle$$ or even $$(x,y)$$.


An ordered pair is a pair of objects, $$(a,b)$$, where the object $$a$$ is called the __first entry__, the object $$b$$ is the __second entry__ of the pair. Unlike unordered pairs (i.e. sets with two elements), an ordered pair is affected by the order of its two elements, so $$(a,b)\neq (b,a)$$. 


Two ordered pairs are equal if their respective components are the same: let $$(a, b)$$ and $$(c, d)$$ be ordered pairs; then $$(a, b) = (c, d)$$ iff $$a = c$$ and $$b = d$$.

The __Cartesian product__ of two sets $$A$$ and $$B$$ is another set, denoted as $$A\times{B}$$ and defined as $$A\times{B} = \{(a,b) : a\in A, b\in B\}$$

Thus $$A\times{B}$$ is a set of all ordered pairs of elements from $$A$$ and $$B$$.

If $$A$$ and $$B$$ are finite sets, then $$|A\times B| = |A|*|B|$$

The Cartesian (cross) product A × B of two sets is defined
as A × B = {(a, b) : a ∈ A, b ∈ B}

So, the × operation pairs the elements of A with the elements of B in such a way that the elements of A appear as first components, and the elements of B appear as second components.

It is also possible to define Cartesian products for more than two factors, in which case we would not have a pair i.e. 2-tuple, but 3-tuple. Even so, all the n-tuples can be represented by an ordered pair (recursively), e.g. an 4-tuple, $$(a,b,c,d)$$, can be represented as $$(a,(b,(c,d)))$$.



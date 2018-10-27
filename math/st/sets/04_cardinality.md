# Set cardinality

- Cardinality
- Infinite and finite set
- Set equality
- Set equivalence
- Enumerable and inenumerable set



**Cardinality** is the number of elements in a set. Cardinality of a set $$X$$ is denoted by $$|X|$$.

An **infinite set** has infinitely many elements, thereby an infinite cardinality; a **finite set**, e.g. $$\{1,2,3, \dots,100\}$$ has finite cardinality (here the element after elipses is called terminal set member).


## Set equivalence

Two sets are equal, $$X=Y$$, iff they have identical elements, or formally, if they are subsets of each other. However, two sets are **equivalent** iff they have the same cardinality. **Set equivalence** is denoted as $$X\simeq Y$$ or $$X\sim Y$$. **Set inequivalence** is denoted as $$X\not\simeq{Y}$$ or $$X\not\sim Y$$.

To see if two sets, $$X$$ and $$Y$$, have the same cardinality, you don't need to count their elements - you can just couple their elements: pairing each element $$x$$ of the set $$X$$ with an element $$y$$ of a set $$Y$$ so that every element belongs to exactly one pair, $$(x,y)$$.

If no unpaired element remains, such pairing of elements from two sets is called **one-to-one** (1-1) or **correspondence**.


## Bijection

Sets $$X$$ and $$Y$$ are equivalent iff they have the same cardinality, that is, if there exists a **bijection function**, $$f:X\to Y$$, from the elements of $$X$$ to those of $$Y$$.

Bijection is a function (and all functions are relations) that associates each element of $$X$$ to exactly one element of $$Y$$, such that all elements of $$Y$$ are associated.


## Countable set

Georg Cantor introduced the term countable set, contrasting sets that are countable with those that are uncountable. Today, countable sets form the foundation of a branch of mathematics called discrete mathematics.

> A countable set is a set with the same cardinality as some subset of the set of natural numbers.

A countable set is either countably finite or countably infinite set. Whether finite or infinite, the elements of a countable set can always be counted one at a time and, although the counting may never finish, every element of the set is associated with a unique natural number.

A set $$S$$ is **countable** if there exists an injective function $$f$$ from $$S$$ to the set of natural numbers, $$\mathbb N$$.

If such injective function is also surjective (and thus bijective), then the set $$S$$ is called **countably infinite**.

> A set is countably infinite if it has one-to-one correspondence with the set of natural numbers.


## Cardinality of infinity

In order to figure out the cardinality of some (possibly infinite) set, we try to find a one-to-one correspondance between that set and the set of natural numbers - such a set is enumerable if the correspondance exists.

To count a set $$S$$ is to establish a bijection function between the set $$S$$ and an initial segment $$\mathbb{N_n}$$ of the set of natural numbers, $$\mathbb{N}$$; the initial segment of the set of natural numbers (excluding zero) is denoted by $$N_n$$.

If $$S\simeq \mathbb{N_n}$$, then a set $$S$$ is **countable finite** and it has $$n$$ elements. If $$S\simeq \mathbb{N}$$, then a set $$S$$ is **countably infinite**.

Between 1874 and 1897, Cantor was the first to work on the task of finding a one-to-one correspondence between number infinite sets (integers, rationals, reals, etc.) and the set of natural numbers. Infinite sets that have this bijection function are countable infinite sets, those that don't (like the set of real numbers) are called **uncountable infinite sets**.

For example, the set of squares of natural numbers can be placed in a 1-1 correspondance with the set of natural numbers themselves:

$$
1 \quad 2\quad 3\quad \dots\quad n \\
1 \quad 4\quad 9\quad \dots\quad n^2
$$

> Infinite set is enumerable (denumerable, countable) if it can be placed in a one-to-one correspondance with the set of natural numbers.

To show that a set is enumerable we present a bijection function that indicates how its members, without repetitions, can be placed in an "infinite" list. That way the first element of an infinite set will be at position 1, the second at position 2, etc; each element will have its **index**.

The set of integers can be enumerated by listing them in the following order:

$$0,\ 1, -1,\ 2, -2,\ \dots$$.

Also enumerable is the set of rational numbers. The way to enumerate rationals is fairly complex (criss-cross path in a matrix), so here is the starting sequance of the positive rationals only:

$$1, 2 ,\frac{1}{2}, \frac{1}{3}, 3, 4, \frac{3}{2}, \frac{2}{3}, \frac{1}{4}, \frac{1}{5}, 5, 6, \frac{5}{2}, \frac{4}{3}, \frac{3}{4}, \frac{2}{5}, \frac{1}{6}, \frac{1}{7}, \frac{3}{5}, \frac{5}{3}, 7, 8, \frac{7}{2}, \frac{5}{4}, \frac{4}{5}, \frac{2}{7}, \frac{1}{8}, \frac{1}{9}, \frac{3}{7}, \frac{7}{3}, 9, \dots$$

```
1/1   1/2 → 1/3   1/4 → 1/5 ...
 ↓  ↗     ↙     ↗     ↙
2/1   2/2   2/3   2/4 ...
    ↙     ↗     ↙
3/1   3/2   3/3 ...
 ↓  ↗     ↙
4/1   4/2 ...
...
```


## Comparing sets

While the cardinality of a finite set is just the count of its elements, extending this notion to infinite sets starts by defining the notion of comparison between arbitrary, possibly infinite, sets.

$$|A| = |B|$$    
Two sets $$A$$ and $$B$$ have the same cardinality if there's a bijection from $$A$$ to $$B$$. Such sets are **equivalent** (equinumerous, equipotent, equipollent), denoted by $$A \simeq B$$ (or $$A \sim B$$ or $$A \approx B$$).

$$|A| \le |B|$$   
Cardinality of $$A$$ has cardinality less than or equal to $$B$$ if there's an injective function from $$A$$ into $$B$$.

$$|A| \lt |B|$$    
Cardinality of $$A$$ is strictly less than $$B$$ if there's an injective (but no bijective) function, from $$A$$ to $$B$$.

**Schröder–Bernstein theorem**    
If $$|A| \le |B|$$ and $$|B| \le |A|$$ then $$|A| = |B|$$

The axiom of choice is equivalent to the statement that $$|A| \le |B|$$ and $$|B| \le |A|$$ for every $$A, B$$.


## Equivalence

The study of cardinality is often called **equinumerosity** (equalness-of-number); the terms equipollence (equalness-of-strength) and equipotence (equalness-of-power) are also used.

The statement that two sets are equinumerous is denoted:
$$A\approx B$$ or $$A\sim B$$ or $$|A|=|B|$$.


Equinumerosity has the characteristic properties of an equivalence relation i.e. reflexivity, symmetry and transitivity.

**Reflexivity**   
Given a set A, the identity function on A is a bijection from A to itself, showing that every set A is equinumerous to itself: A ~ A.

**Symmetry**    
For every bijection between two sets A and B there exists an inverse function which is a bijection between B and A, implying that if a set A is equinumerous to a set B then B is also equinumerous to A: A ~ B implies B ~ A.

**Transitivity**    
Given three sets A, B and C with two bijections f : A → B and g : B → C, the composition g ∘ f of these bijections is a bijection from A to C, so if A and B are equinumerous and B and C are equinumerous then A and C are equinumerous: A ~ B and B ~ C together imply A ~ C.


## Cardinal numbers

Apart from the functional definition (given above), another way to define cardinality is to define it as a specific object.

The relation of having the same cardinality is called equinumerosity, and this is an equivalence relation on the class of all sets.

The equivalence class of a set A under this relation then consists of all those sets which have the same cardinality as A.

There are two ways to define the cardinality of a set:
* The cardinality of a set A is defined as its equivalence class under equinumerosity.
* A representative set is designated for each equivalence class. The most common choice is the initial ordinal in that class. This is usually taken as the definition of cardinal number in axiomatic set theory.

Assuming axiom of choice (AC), the cardinalities of the infinite sets are denoted $$\aleph_{0} \lt \aleph_{1} \lt \aleph_{2} \lt \ldots$$

For each ordinal $$\alpha, \aleph_{\alpha+1}$$ is the least cardinal number greater than $$\aleph_{\alpha}$$.

## Cardinality of the continuum

The cardinality of the natural numbers is denoted aleph-null, $$\aleph_{0}$$, while the cardinality of the real numbers is denoted by $$\mathfrak{c}$$, also referred to as the **cardinality of the continuum**.

Cantor showed, using the **diagonal argument**, that $$\mathfrak{c} > \aleph_{0}$$.

We can show that $$\mathfrak{c} = 2^{\aleph_{0}}$$ i.e. the cardinality of the powerset of $$\mathbb{N}$$.

The **continuum hypothesis** says that $$\aleph_1 = 2^{\aleph_0}$$, i.e. 
$$2^{\aleph_0}$$ is the smallest cardinal number bigger than 
$$\aleph_0$$, i.e. there is no set whose cardinality is strictly between that of the integers and that of the real numbers.

The continuum hypothesis is independent of ZFC, that is, it is impossible to prove the continuum hypothesis or its negation from ZFC (provided ZFC is consistent).

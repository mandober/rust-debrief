# Fundamental Counting Rules


### The rule of sum
The rule of sum or __addition principle__ states that if there are $$x$$ possible outcomes for an event and $$y$$ possible outcomes for another event, and the two events cannot occur at the same time, then there are $$x + y$$ total possible outcomes for these events.

In terms of set theory, it states that sum of the sizes of a finite collection of disjoint sets is the size of the union of these sets: if $$S_1, S_2,\dots, S_n$$ are disjoint sets, then $$|S_1| + |S_2| + \dots +|S_n| = |S_1\cup S_2\cup \cdots \cup S_n|$$


### The rule of product
The rule of product or **multiplication principle** is also known as the **fundamental counting principle**.

It states that if there are $$x$$ ways of doing something and $$y$$ ways of doing another thing, then there are $$x · y$$ ways of performing both actions.

Generally, if there are $$x_1, x_2, \dots , x_n$$ number of ways of performing $$n$$ number of independent events separately, then the total unique ways by which all of them can be performed together will be the product: $$x_1 · x_2 · x_3 · (\dots ) · x_n$$.


### The inclusion–exclusion principle
This rule relates the size of the union of overlapping sets with their individual sizes and the size of their intersection.

The name comes from the idea that the principle is based on over-generous inclusion, followed by compensating exclusion.

It states that the number of elements in the union of sets $$A$$ and $$B$$ is equal to the sum of the number of elements in $$A$$ and $$B$$, minus the number of elements in their intersection, $$|A \cup B| = |A| + |B| - |A\cap B|$$


### Bijective proof
Bijective proof is a proof technique that finds a bijective function, $$f: A \to B$$, between two finite sets $$A$$ and $$B$$ (or a size-preserving bijective function between two combinatorial classes) thus proving that they have the same number of elements, $$|A| = |B|$$.

This technique is used to determine the size of a set that offers no direct way to count its elements. In such case, we can establish a bijection from such set $$A$$ to a more easily enumerable set $$B$$.


### Pigeonhole principle
The pigeonhole principle often ascertains the existence of something or is used to determine the minimum or maximum number of something in a discrete context. 

It states that if $$a$$ items are each put into one of $$b$$ boxes, where $$a \gt b$$, then one of the boxes contains more than one item.

The formal statement is that there does not exist an injective function whose codomain is smaller than its domain.

Using this one can, for example, demonstrate the existence of some element in a set with some specific properties. Or to demonstrate possibly unexpected results, e.g. that there is above 50% probability that two persons share the same birthday in a group of 23 people.


### Method of distinguished element
The method of distinguished element singles out a "distinguished element" of a set to prove some result.

In the mathematical field of enumerative combinatorics, identities are sometimes established by arguments that rely on singling out one "distinguished element" of a set.

Double counting is a technique that equates two expressions that count the size of a set in two ways.

Many combinatorial identities arise from double counting methods or the method of distinguished element. 


### Generating functions
Generating functions and recurrence relations are powerful tools that can be used to manipulate sequences, and can describe if not resolve many combinatorial situations.

Generating functions can be thought of as polynomials with infinitely many terms whose coefficients correspond to terms of a sequence. This new representation of the sequence opens up new methods for finding identities and closed forms pertaining to certain sequences. 

### Recurrence relation
A recurrence relation defines each term of a sequence in terms of the preceding terms. Recurrence relations may lead to previously unknown properties of a sequence, but generally closed-form expressions for the terms of a sequence are more desired.

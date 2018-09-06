# Set

- Cardinality of the set X is denoted by $$|X| = n$$
- Cardinality of the powerset of the set X is $$|\mathcal{P}(X)| = 2^{|X|} = 2^n$$
- Cardinality of the Cartasian product of the set X: $$|X \times X| = |X^2| = n^2$$
- Cardinality of the powerset of the Cartasian product of the set X is $$|\mathcal{P}(X^2)| = 2^{(n^2)}$$
- Each relation is a subset of the Cartasian product of the set X: $$R \subseteq X^2$$
- Each relation is an element in the powerset of the Cartasian product of the set X: $$R \in P(X^2)$$
- Number of all possible relations on a $$X^2$$ set: $$|\mathcal{P}(X^2)| = 2^{(n^2)}$$

---

## Cardinalities

$$n$$ | $$\mathcal{P}: 2^n$$ | $$\mathcal{C}: n^2$$| $$\mathcal{R}: 2^{(n^2)}$$
------|--------------|-------------|------------
0     | $$2^0$$ = 1  | $$0^2$$ = 0 | $$2^0$$ = 1
1     | $$2^1$$ = 2  | $$1^2$$ = 1 | $$2^1$$ = 2
2     | $$2^2$$ = 4  | $$2^2$$ = 4 | $$2^4$$ = 16
3     | $$2^3$$ = 8  | $$3^2$$ = 9 | $$2^9$$ = 512
4     | $$2^4$$ = 16 | $$4^2$$ = 16| $$2^{16}$$ = 65,536
5     | $$2^5$$ = 32 | $$5^2$$ = 25| $$2^{25}$$
6     | $$2^6$$ = 64 | $$6^2$$ = 36| $$2^{36}$$
7     | $$2^7$$ = 128| $$7^2$$ = 49| $$2^{49}$$
8     | $$2^8$$ = 256| $$8^2$$ = 64| $$2^{64}$$
9     | $$2^9$$ = 512| $$9^2$$ = 81| $$2^{81}$$

This table shows: 
- the first column shows the cardinality of a n-element set.
- the second column shows the cardinality of the powersets of an n-element set 
- the third column shows the cardinality of the Cartesian product of an n-element set with itself.
- the fourth column shows the number of ditinct relations of an n-element set


---

## Number of relations

Let set A:
$$$A = \{\ \ 1, \ \ 2\ \ \}$$$

Powerset of A:
$$$P(A) = \{\ \ \varnothing,\ \ \{1\},\ \ \{2\},\ \ \{1,2 \}\ \ \}$$$

Cartesian product of the set $$A$$:

$$$
A\times A = \{ \ \ (1,1),\ \ (1,2),\ \ (2,1),\ \ (2,2)\ \ \}
$$$


Enumerated powerset of the Cartesian product of the set $$A$$ i.e. $$\mathcal{P}(A\times A) = \mathcal{P}(A^2)$$ is given below. There are 16 elements in this powerset i.e. there are 16 possible relations:
- 1 possible way to choose no pairs (0-place): $$4\choose 0$$
- 4 possible ways to choose 1 pair (1-place): $$4\choose 1$$
- 6 possible ways to choose 2 pairs (2-place): $$4\choose 2$$
- 4 possible ways to choose 3 pairs (3-place): $$4\choose 3$$
- 1 possible way to choose all pairs (n-place): $$4\choose 4$$

The formula to choose k-places out of n elements:

$$$
{n \choose k} = {\frac{n!}{k!(n-k)!}}
$$$

Enumerated powerset of the Cartesian product of the set $$A$$:

$$$
{\\
\mathcal{P}(\ \ \{\ \ (1,1),\ \ (1,2),\ \ (2,1),\ \ (2,2)\ \ \}\ \ ) = \\
\Huge\{\\
R_1:  \varnothing, \\
R_2:  \{(1,1)\},   \\
R_3:  \{(1,2)\},   \\
R_4:  \{(2,1)\},   \\
R_5:  \{(2,2)\},   \\
R_6:  \{(1,1), (1,2)\}, \\
R_7:  \{(1,1), (2,1)\}, \\
R_8:  \{(1,1), (2,2)\}, \\
R_9:  \{(1,2), (2,1)\}, \\
R_{10}:  \{(1,2), (2,2)\}, \\
R_{11}: \{(2,1), (2,2)\}, \\
R_{12}: \{(1,1), (1,2), (2,1)\}, \\
R_{13}: \{(1,1), (1,2), (2,2)\}, \\
R_{14}: \{(1,1), (2,1), (2,2)\}, \\
R_{15}: \{(1,2), (2,1), (2,2)\}, \\
R_{16}: \{(1,1), (1,2), (2,1), (2,2)\} \\
\Huge\}
\\}
$$$

Relations:
- null (1): $$R_1$$
- identity (1): $$R_8$$
- universal (1): $$R_{16}$$
- reflaxive (4): $$R_1, R_8, R_{13}, R_{16}$$
- irreflexive (4): $$R_1, R_3, R_4, R_9$$
- symmetric (8): $$R_1, R_2, R_5, R_8, R_9, R_{12}, R_{15}, R_{16}$$
- anti-symmetric (12): $$R_1-R_8, R_{10}, R_{11}, R_{13}, R_{14}$$
- asymmetric (5): $$R_1-R_5$$
- transitive (13): only $$R_9$$ and $$R_{15}$$ are not transitive

Summary:
- The number of distinct binary relations on an n-element set is $$2^{(n^2)}$$
- The number of reflexive and irreflexive relations is the same.
- The number of strict partial orders (irreflexive transitive relations) is the same as that of partial orders.
- the number of equivalence relations is the number of partitions, which is the **Bell number**.


---

Ordered pair $$(1,2) = \{\{1\},\{1,2\}\}$$
Powerset of A: $$P(A)$$
$$= \{\varnothing, \{1\}, \{2\}, \{1,2\}\}$$
$$= \{\varnothing, \{1\}, \{2\}, \{1,2\}, \{2,1\}\}$$
$$= \{\varnothing, \{1\},\{1,2\}, \{2\},\{2,1\}\}$$
$$= \{\varnothing, (1,2), (2,1) \}$$

Ordered pair $$(x,y) = \{\{x\},\{x,y\}\}$$
if $$x=y$$ then $$(a,a)$$
$$=\{\{a\},\{a,a\}\}$$
$$=\{\{a\},\{a\}\}$$
$$=\{\{a\}\}$$



**Sets**
- A set is an unordered collection of distinct objects (that share some common property) considered as an object in its own right.
- defined intensionaly (semantic desc): "A set of odd positive ints"
- defined extensionaly (enumeration): $$\{a,b,c\}$$
- denoted with a capital letter: $$A=\{a,b,c\}$$, elements with lowercase
- If $$a$$ is an element of set $$B$$ then: $$a\in B$$, if not $$a\notin B$$
- Order or repetition unimportant: $$\{\{a\},b,\varnothing\}=\{b, \{a\},\{\},b,\varnothing,\{\}\}$$
- A set can contain other sets, $$x \in A, A \in \mathscr{M}$$, so $$x\in A$$ but $$x \notin \mathscr{M}$$.
- Two sets are equal if they contain the same elements.

**Types**
- An empty set contains no elements: $$\varnothing = \{\}$$
- A set, $$\{a,b\}$$, is distinguished from the ordered pair, $$(a,b)$$
- $$n$$-element set is different from ordered $$n$$-tuple: $$\{x_1, x_2,\dots,x_n\} \neq (x_1,\dots, x_n)$$
- Two ordered pairs are equal __iff__: $$(a,b) = (x,y) \iff a=x \land b = y$$
- The order of the elements or their repetition is of no consequence to sets, $$\{a,b,\{\}\}=\{b,\{\},b,\varnothing,a\}$$.
- $$\{\varnothing\}=\{\{\}\}$$


**Subset and powerset**
- if all elements of a set $$A$$ are also elements of a set $$B$$, then $$A$$ is a __subset__ of $$B$$: $$A \subseteq B$$ and $$B$$ is a __superset__ of $$A$$: $$B \supseteq A$$.
- a set $$B$$ is __equal__ to $$A$$ if the two sets consist of exactly the same elements: $$A = B \iff A \subseteq B \land B \subseteq A$$.
- if $$A \subseteq B$$ but $$B \neq A$$ then $$A$$ is a __proper subset__ of $$B$$: $$A \subset B$$.
- if a set $$P$$ contains all possible subsets of a set $$A$$, then $$P$$ is a __powerset__ of $$A$$, denoted as $$P(A)$$.
- e.g. if $$A=\{a,b\}$$, then $$P=\{\{a\},\{b\},\{ab\},\{\varnothing\}\}$$
- here, the cardinality (number of elements) of $$P$$ is 4, denoted as $$|P|=4$$


Properties of sets: $$\forall A,B,C$$
- $$A \subseteq A$$ (reflexivity)
- $$A \subseteq B \land B \subseteq C \rightarrow A \subseteq C$$  (transitivity)
- $$A \subseteq B \land B \subseteq A \rightarrow A=B$$ (anti-symmetry, axiom of extensionality)
- $$\varnothing \subseteq A$$
- $$A\subseteq \varnothing \rightarrow A=\varnothing$$


# Introduction to Sets

- sets
- set elements
- infinite and finite sets
- set equality
- set notation
- special sets


The theory of sets, initiated by Georg Cantor (1845–1918), constitutes the basis of almost all modern mathematics. A set is often described as a __collection__ (aggregate, class, totality, family) of objects of any specified kind, but the set concept is one of the basics so the term _set_ must be accepted as a primitive notion, without definition. The objects belonging to a set are called its __elements__ or __members__.

__Infinite__ set has infinitely many elements; otherwise it is a __finite__ set. Infinite sets: $$\{1,2,3,\dots\}$$, or $$\{\dots,1,2,3,\dots\}$$. A sets with terminal member: $$\{1,2,3, \dots,100\}$$, or $$\{1,2,\dots,n\}$$

Two sets are __equal__ if they contain exactly the same elements. The order of the elements or their repetition is of no consequence to sets, $$\{a,b,\{\}\}=\{b,\{\},b,\varnothing,a\}$$

Sets are usually denoted with uppercase letters: the set $$\{2,4,6,8\}$$ may be declared as $$A=\{2,4,6,8\}$$ and then use $$A$$ to stand for $$\{2,4,6,8\}$$. To express that $$2$$ is an element of the set $$A$$, we write $$2 \in A$$. Expressions like $$2,4,6 \in A$$ indicate that several things are in a set. If an element is not in a set: $$3 \notin A$$. A set of one element is not the same as that element alone: $$p \neq \{p\}$$.

Once a set has been formed, it is a __new entity__, different from any of its elements. A newly formed set may be an element of some other set. Sets whose elements are other sets are __collections of sets__. If an element $$a$$ is in the set $$A$$ and set $$A$$ is in the set $$\mathscr{M}$$, that doesn't imply the element $$a$$ is in the set $$\mathscr{M}$$: $$x \in A \land A \in \mathscr{M} \not \rightarrow x \in A$$.

We are mainly concerned with sets whose elements are mathematical entities, such as numbers, points, functions, etc., but sets need not have just numbers as elements. Sets can even have other sets as elements.

Some sets are so prevalent they have a __special symbol__ assigned for them:
- The set of boolean (truth) values: $$\mathbb{B}=\{\bot,\top\}=\{F,T\}=\{0,1\}$$
- The set of natural numbers, either $$\mathbb{N}=\{1,2,3,\dots\}$$ or $$\mathbb{N}=\{0,1,2,3,\dots\}$$
- The set of integers, $$\mathbb{Z}=\{-2,-1,0,1,2,\dots\}$$; $$\mathbb{Z^+}=\{1,2,3,\dots\}$$
- The set of rational numbers: $$\mathbb{Q}=\{p/q : p \in\mathbb{Z}, q\in \mathbb{N}\}$$
- The set of real numbers, $$\mathbb{R}$$
- The set of complex numbers: $$\mathbb{C}=\{a,b\,i:a,b\in\mathbb{R}\}$$

$$$$
\mathbb{N}\subseteq \mathbb{Z}\subseteq \mathbb{Q}\subseteq \mathbb{R}
$$$$

The __empty set__ is the set that has no elements. It is denoted as $$\varnothing$$ and it represents $$\{\}$$. The empty set is the only set whose cardinality is zero.


- $$\varnothing=\{\}\neq\{\{\}\}=\{\varnothing\} $$
- the empty set is regarded as an existing object.
- empty set is a subset of any set
- empty set is a proper subset of any non-empty set





A special notation called set-builder notation is used to describe sets that are too big or complex to list between braces X={exp,exp2,...|rule,rule2,...}

The expression |X| means absolute value if X is a number and cardinality if X is a set

It is an unfortunate notational accident that (a,b) can denote both an interval on the line and a point on the plane. The difference is usually clear from context.


## 2 The Cartesian Product

An ordered pair is a list (x, y) of two things x and y,
enclosed in parentheses and separated by a comma.  The things in an ordered
pair don’t have to be numbers

The Cartesian product of two sets A and B is another
set, denoted as A £B and defined as A £B ˘ '(a,b) : a 2 A,b 2 B“
Thus A £ B is a set of ordered pairs of elements from A and B
If A and B are finite sets, then |A|x|B| = |A|*|B|
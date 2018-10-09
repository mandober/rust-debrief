# Set Theory: Debrief

- subset is an example of inclusion relation
- inclusion relation is antisymmetric
- set membership (belonging) is an example of membership relation
- membership (elementhood) relation is reflexive
- inclusion relation is transitive, membership relation is not


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

Let set A: $$A = \{\ \ 1, \ \ 2\ \ \}$$

Powerset of A: $$P(A) = \{\ \ \varnothing,\ \ \{1\},\ \ \{2\},\ \ \{1,2 \}\ \ \}$$

Cartesian product of the set $$A$$: $$A\times A = \{ \ \ (1,1),\ \ (1,2),\ \ (2,1),\ \ (2,2)\ \ \}$$

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
- defined intensionaly (semantically): "A set of odd positive ints"
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


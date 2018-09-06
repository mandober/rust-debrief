# Relations on sets

We can consider relations as a special sort of set.

If a and b are two objects, we can combine them into the ordered pair (a,b).

If X and Y are sets, then the Cartesian product $$X \times Y$$ of X and Y is the set of all pairs (a,b) with $$a\in X$$ and $$b\in Y$$. In particular, $$X^2 = X \times X$$ is the set of all pairs from X.

Considering a relation on a set, e.g. the `<` relation on the set $$\mathbb{N}$$ of natural numbers, and considering the set of all pairs of numbers (n,m) where n < m, i.e.

$$$R=\{(n,m):n,m \in \mathbb{N}, n \lt m\}$$$

then there is a close connection between the number n being less than a number m and the corresponding pair (n,m) being a member of R, namely, n < m iff $$(n,m) \in R$$. In a sense we can consider the set R to be the `<` relation on the set $$\mathbb{N}$$.

A **binary relation** on a set $$X$$ is a subset of $$X^2$$.
If $$R \subseteq X^2$$ is a binary relation on $$X$$ and $$x, y \in X$$, we write $$xRy$$ (or $$Rxy$$) for $$(x,y) \in R$$.

The set $$\mathbb{N^2} = \mathbb{N} \times \mathbb{N}$$ of ordered pairs of natural numbers (starting and ending curly-braces elided):

$$$
\begin{matrix}
  (0,0), &  (0,1), &  (0,2), &  (0,3), &  \dots \\
  (1,0), &  (1,1), &  (1,2), &  (1,3), &  \dots \\
  (2,0), &  (2,1), &  (2,2), &  (2,3), &  \dots \\
  (3,0), &  (3,1), &  (3,2), &  (3,3), &  \dots \\
  \vdots &  \vdots &  \vdots &  \vdots &  \ddots
\end{matrix}\\
\text{fig.1}
$$$

The subset of all pairs resting on the diagonal (fig.1), 
$$(0,0), (1,1), (2, 2),\dots$$, 
is the **identity relation** on $$\mathbb{N}$$,
$$I_{d}=\{(x,x):x \in X\}$$ for any set $$X$$.

The subset of all pairs above the diagonal, 
$$L=\{(0,1), (0,2), (0,3),\dots , (1,2), (1,3), \dots , (2,3), \dots\}$$, 
is the __less than relation__,
$$nLm \iff n \lt m$$ (the symbol $$\iff$$ means "if and only if").

The subset of all pairs below the diagonal 
is the **greater than relation**,
$$nGm \iff n \gt m$$.

The union $$L_{e} = L \cup I_{d}$$ 
is the **less than or equal to relation**,
$$nL_{e}m \iff n \le m$$.

The union $$G_{e} = G \cup I_{d}$$ 
is the **greater than or equal to relation**,
$$nG_{e}m \iff n \ge m$$.

$$L, G, L_{e}, G_{e}$$ are special kinds of relations called **orders**.

$$L$$ and $$G$$ have the property that no number bears $$L$$ or $$G$$ to itself i.e. for all $$n$$, neither $$nLn$$ nor $$nGn$$ (e.g. neither 3 < 3 nor 3 > 3). Relations with this property are called _irreflexive_, and, if they also happen to be orders, they are called _strict orders_.

Any subset of $$X^2$$ is a relation on $$X$$. Even the empty set is a relation on any set i.e. the **empty relation**, which consists of no pairs.

Also $$X^2$$ itself is a relation on $$X$$ called **universal (full) relation**, consisting of all the pairs of the Cartasian product $$X \times X$$.


## Special Properties of Relations
Some relations are so common they've been given special names. For instance, $$\le$$ and $$\subseteq$$ both relate their respective domains (e.g. $$\mathbb{N}$$ in the case of $$\le$$, and $$\mathcal{P}(X)$$ in the case of $$\subseteq$$) in similar ways.

To get at exactly how these relations are similar, and how they differ, we categorize them according to some special properties that relations can have. It turns out that (combinations of) some of these special properties are especially important, like orders and equivalence relations.

### Reflexivity
A relation is **reflexive** if every element of the set is related to itself:
a relation $$R \subseteq X^2$$ is reflexive iff $$\forall x \in X:(x,x) \in R$$.
To be reflexive, e.g. a relation $$R$$ to the set $$\mathbb{N^2}$$ (see fig.1), must contain all the identity (diagonal) pairs; it may also contain other, non-identity pairs.

Examples
- the empty relation is not reflexive
- the universal relation is reflexive

A relation is **irreflexive** if it doesn't conatin no identity (diagonal) pairs (it may contain other pairs as long as they're not id pairs). Irreflexive relation, $$R \subseteq X^2$$, is denoted by $$\forall x \in X : (x,x)\not \in R$$.

Examples
- the empty relation is irreflexive
- the universal relation is not irreflexive


### Transitivity
A relation is **transitive** iff, whenever $$xRy$$ and $$yRz$$, then also $$xRz$$. To be transitive a relation must satisfy this condition: if it contains the pair (x,y) AND the pair (y,z), ONLY THEN it must also contain the pair (x,z). This means that empty relation is transitive. A relation that contains the pairs (x,y) and (x,z) is transitive as well.

Only when it contains (x,**y**) _and_ (**y**,z) i.e. only when there's a _connecting element_: an element that's the second entry in one pair and the first entry in another, then it must also contain the pair (x,z) in order to be a transitive relation.

Examples
- the empty relation is transitive
- the universal relation is transitive

### Symmetry
A relation $$R \subseteq X^2$$ is **symmetric** iff, whenever $$xRy$$, then also $$yRx$$. If a relation contains a pair (x,y) then it must also contain a pair (y,x) to be symmetric.

Examples
- the empty relation is symmetric
- the universal relation is symmetric
- a relation conting only a pair (1,1) is symmetric

A relation $$R \subseteq X^2$$ is **anti-symmetric** iff, whenever both $$xRy$$ and $$yRx$$ then $$x = y$$; i.e. if $$x \neq y$$ then either $$\lnot xRy$$ or $$\lnot yRx$$. **If** a relation contains a pair (x,y) **then** it must not also contain a pair (y,x) in order to be anti-symmetric, **unless** that pair was (x,x) to begin with.

Examples
- the empty relation is anti-symmetric
- a relation conting only a pair (1,2) is anti-symmetric
- a relation conting only a pair (1,1) is anti-symmetric, even though it acually contains (x,y) and (y,x), but since x=y, it is allowed.

---

- In a symmetric relation, xRy and yRx always hold together, or neither holds.
- In an anti-symmetric relation, the only way for xRy and yRx to hold is if x = y. This doesn't require that xRy and yRx hold when x = y, only that it isn't ruled out.
- an anti-symmetric relation can be reflexive
- not every anti-symmetric relation is reflexive
- being anti-symmetric and not being symmetric are different conditions
- a relation can be both symmetric and anti-symmetric at the same time, e.g. the **identity relation**.



**Connectivity**: A relation $$R \subseteq X^2$$ is connected if $$\forall x,y \in X:x \neq y$$, then either $$xRy$$ or $$yRx$$.

**Partial order**: A relation $$R \subseteq X^2$$ that is reflexive, transitive, and anti-symmetric.

**Linear order**: A partial order that is also connected.

**Equivalence relation**: A relation $$R \subseteq X^2$$ that is reflexive, symmetric and transitive.



## Orders

Very often we are interested in comparisons between objects, where one object may be less or equal or greater than another in a certain respect.
Size is the most obvious example of such a comparative relation, or order.
But not all such relations are alike in all their properties.
For instance, some comparative relations require any two objects to be comparable, others don’t. (If they do, we call them linear or total).
Some include identity (like ≤) and some exclude it (like <).

- **Preorder**: a relation which is both reflexive and transitive; `Re+Tr`
- **Partial order**: a preorder which is also anti-symmetric; `Re+vS+Tr`
- **Linear (total) order**: a partial order which is also connected. `Re+vS+Tr+Co`


# Relations

A **relation** $$R$$ from set $$X$$ to set $$Y$$, where $$x \in X$$ and $$y \in Y$$, is denoted as $$xRy$$, and it is a subset of their __Cartesian product__, $$X \times Y$$.

$$xRy : x \in X, y\in Y\\$$

$$R \subseteq X \times Y$$

The sets $$X$$ and $$Y$$ may be the same set $$X$$ in which case its Cartesian product is usually denoted by $$X^2$$ and relations by $$xRx$$.

Binary relations are used to model concepts like "is greater than", "is equal to", and similar. The concept of function is defined as a special kind of binary relation.

A __binary relation__ between sets $$A$$ and $$B$$ is __a subset__ of their Cartesian product, $$A \times B$$.

Or equivalently, it is an element in the powerset of their Cartesian product.

Any subset of the Cartesian product forms a relation, whether it is a named (e.g. symmetric) or unnamed relation; even the empty set (being a subset of the Cartesian product set) is a relation.

A single set's Cartesian product (with itself) is commonly denoted as $$\mathbb{N^2} = \mathbb{N} \times \mathbb{N}$$.



If $$X$$ and $$Y$$ are sets, the Cartesian product $$X \times Y$$ is the set of all ordered pairs $$(x,y)$$ with $$x\in X$$ and $$y \in Y$$. And the set $$X^2 =X\times X$$ is the set where all pair of $$x\in X$$.



The set $$\mathbb{N^2} = \mathbb{N} \times \mathbb{N}$$ of ordered pairs of natural numbers (starting and ending curly-braces demarking this set are not showndue to formatting):

$$$
\begin{matrix}
  (1,1), &(1,2),  &(1,3), &\dots \\
  (2,1), &(2,2),  &(2,3), &\dots \\
  (3,1), &(3,2),  &(3,3), &\dots \\
  \vdots &\vdots  &\vdots &\ddots
\end{matrix}\\
\text{figure 1.1}
$$$


*Any* subset of this set forms a relation:
- __full relation__, where every pair participates, is the set of the __Cartesian product__ itself.
- on the other side of the extreme is the __empty relation__ which is the __empty set__; even though no pair participates, it is still considered a relation.
- between these two extremes are all other relations, some of which have a name, being more popular then others. The most popular ones, come with a name and a special symbol attached.

__Less than__ (LT, `<`) relation is formed by the subset of all pairs lying above the diagonal, and __greater than__ (GT, `>`) by the subset of all pairs below the diagonal. The union of these two with identity relation form __less than or equal to__ (LE, `<=`) and __greater than or equal to__ (GE, `>=`) relations, respectively.

Relations are categorized by the special properties they hold.
Some are very important like orders and equivalence relations.


## Reflexivity

A binary relation is **reflexive** if every element of the set is related to itself.

A relation $$R \subseteq X^2$$ is reflexive iff $$\forall x : xRx$$.

In other words, a relation $$R$$ to the set $$\mathbb{N^2}$$ is reflexive if it contains __all the diagonal (identity) pairs__ e.g. $$R=\{(1,1),(2,2),(3,3),\dots\}$$.

If it contains __no diagonal pairs__ then the relation is __irreflexive__.

A relation $$R \subseteq X^2$$ is irreflexive iff $$\forall x : x\not{\,R} 
x$$.


## Transitivity
A relation R ⊆ X2 is transitive iff, whenever Rxy and Ryz, then also Rxz.


## Symmetry
A relation R ⊆ X2 is symmetric iff, whenever Rxy, then also Ryx.

**Anti-symmetry**
A relation R ⊆ X2 is anti-symmetric iff, whenever both Rxy and Ryx, then x = y (or, in other words: if x 6= y then either :Rxy or :Ryx).


---

A **chain** is a totally ordered set or a totally ordered subset of a poset. **Chain complete** is a partially ordered set in which every chain has a least upper bound. An **antichain** is a poset in which no two elements are comparable, i.e., there are no two distinct elements x and y such that x ≤ y. In other words, the order relation of an antichain is just the identity relation.

An **antitone** function f between posets P and Q is a function for which, for all elements x, y of P, x ≤ y (in P) implies f(y) ≤ f(x) (in Q). Another name for this property is **order-reversing**. In analysis, in the presence of total orders, such functions are often called **monotonically decreasing**, but this is not a very convenient description when dealing with non-total orders. The dual notion is called monotone or order-preserving.

A **bounded poset** is one that has a least element and a greatest element. A poset is **bounded complete** if every of its subsets with some upper bound also has a least such upper bound. The dual notion is not common.

A **closure operator** on the poset P is a function C : P → P that is monotone, idempotent, and satisfies C(x) ≥ x for all x in P.

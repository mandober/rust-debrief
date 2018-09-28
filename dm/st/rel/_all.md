# Relations

- A relation $$R$$ from set $$X$$  (the set of departure) to set $$Y$$ (the set of destination or codomain), where $$x \in X$$ and $$y \in Y$$, is denoted as $$(x,y) \in R$$ or $$xRy$$ (x is R-related to y).
- If the sets are equal, the Cartesian product, $$X\times X$$, is denoted by $$X^2$$ and a relation is $$xRx$$
- **A relation is a subset** of the Cartesian product, $$R \subseteq X\times Y$$
- **A relation is an element** of the powerset of the Cartesian product of sets, $$R \in \mathcal{P}(X\times Y)$$
- Total number of relations of an n-element set with itself is $$|\mathcal{P}(X^2)| = 2^{n^2}$$



---

$$xRy : x \in X, y\in Y\\$$
$$R \subseteq X \times Y$$
$$X$$ and $$Y$$
$$X^2$$
$$xRx$$

---

Binary relations are used to model concepts like "is greater than", "is equal to", and similar. The concept of function is defined as a special kind of binary relation.

Relations are categorized by the special properties they hold.

If $$X$$ and $$Y$$ are sets, **the Cartesian product** $$X \times Y$$ is the set of all ordered pairs $$(x,y)$$ with $$x\in X$$ and $$y \in Y$$. And the set $$X^2 =X\times X$$ is the set where all pair of $$x\in X$$.

**A binary relation** between sets $$A$$ and $$B$$ is a **subset** of their Cartesian product, $$A \times B$$. Or equivalently, it is an **element** in the powerset of their Cartesian product.

Any subset of the Cartesian product forms a relation: the Cartesian product itself forms a universal (full) relation and the empty set (being a subset of the Cartesian product set) forms an empty (or null) relation.

A single set's Cartesian product (with itself) is commonly denoted as $$\mathbb{N^2} = \mathbb{N} \times \mathbb{N}$$.

---

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


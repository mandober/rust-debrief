# Relations





An ordered pair is a pair of objects, $$(a,b)$$, where the object $$a$$ is called the __first entry__, the object $$b$$ is the __second entry__ of the pair. Unlike unordered pairs (i.e. sets with two elements), an ordered pair is affected by the order of its two elements, so $$(a,b)\neq (b,a)$$. 


A binary relation $$R$$ from set $$X$$ to set $$Y$$, where $$x \in X$$ and $$y \in Y$$, is denoted as $$xRy$$, and it is a subset of the __Cartesian product__ $$X \times Y$$:

$$$
xRy : x \in X, y\in Y\\
R \subseteq X \times Y
$$$

so, a binary relation $$R$$ on the sets $$A$$ and $$B$$ is in fact an element in their power set:

$$$
R \in \mathfrak{P}(A \times B)
$$$

The sets $$X$$ and $$Y$$ may be the same set $$X$$ in which case its Cartesian product is usually denoted by $$X^2$$ and relations by $$xRx$$.

Binary relations are used to model concepts like "is greater than", "is equal to", and similar. The concept of function is defined as a special kind of binary relation.

A __binary relation__ between sets $$A$$ and $$B$$ is __a subset__ of their Cartesian product, $$A \times B$$. Or equivalently, it is an element in the powerset of their Cartesian product.

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


## Types of Relations
(using figure 1.1 as example)

- __Full relation__ is the Cartesian product set itself, $$R=\mathbb{N^2}$$
- __Empty relation__ is the __empty set__, $$R=\emptyset$$
- __Identity__ relation on $$\mathbb{N}$$: the subset consisting of the pairs lying on the diagonal i.e. $$\{(1,1),(2,2),(3,3),\dots\}$$. It can be defined as $$Id_x=\{(x,x) : x \in X\}$$
- __Inverse__ relation is the set of inverted pairs, usually denoted by $$R'$$.



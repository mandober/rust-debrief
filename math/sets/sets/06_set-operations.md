# Set operations
math.dm.st.sets.ops

- disjoint
- overlapping
- Universal set
- Union
- Intersection
- Difference
- Complement

Two sets that don't have any elements in common are called **disjoint** sets. Properties of disjoint sets:
$$A \cap B= \varnothing$$
$$|A \cup B| = |A| + |B|$$

Two sets that have at least one common element are called **overlapping** sets. Properties of overlapping sets:
$$|A \cup B| = |A| + |B| - |A \cap B|$$

$$|A \cup B| = |A-B| + |B-A| + |A \cap B|$$

$$|A| = |A-B| + |A \cap B|$$

$$|B| = |B-A| + |A \cap B|$$


__Universal__ set is a collection of all elements in a particular context. All the sets in that context are essentially subsets of this universal set. Universal sets are represented as $$\mathcal{U}$$.



Commutative Union and intersection operations are
commutative: i.e.,
S ∪ T = T ∪ S
S ∩ T = T ∩ S

Associative Union and intersection operations are
associative: i.e.,
R ∪ (S ∪ T ) = (R ∪ S) ∪ T
R ∩ (S ∩ T ) = (R ∩ S) ∩ T

Identity The identity under set union is Ø and
the identity under intersection is U.
S ∪ ∅ = ∅ ∪ S = S
S ∩ U = U ∩ S = S

Distributive The union operator distributes over
the intersection operator and vice versa.
R ∩ (S ∪ T ) = (R ∩ S) ∪ (R ∩ T )
R ∪ (S ∩ T ) = (R ∪ S) ∩ (R ∪ T )

DeMorgan’s law The complement of S ∪ T is given by:
(S ∪ T )c = Sc ∩ T c.

The complement of S ∩ T is given by:
(S ∩ T )c = Sc ∪ T c

De Morgan’s law is named afterAugustus De Morgan, a nineteenth century English mathematician who was a contemporary of George Boole.

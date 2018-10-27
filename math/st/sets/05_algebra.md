# Algebra of sets

https://www.wikiwand.com/en/Algebra_of_sets


- algebra of sets
- set properties
- laws of sets
- union
- intersection
- complementation
- set equality
- set inclusion
- Boolean algebra

The algebra of sets defines the properties and laws of sets, the set-theoretic operations of union, intersection, and complementation and the relations of set equality and set inclusion.

Any set of sets closed under the set operations forms a Boolean algebra:
- **join** operator: union
- **meet** operator: intersection
- **complement** operator: complement
- **bottom** : empty set
- **top**: the universe set under consideration


Set union and intersection satisfy many laws (identities):
* Commutative laws:   
  $$\quad A\cup B=B\cup A$$   
  $$\quad A\cap B=B\cap A$$
* Associative laws:   
  $$\quad (A\cup B)\cup C=A\cup (B\cup C)$$   
  $$\quad (A\cap B)\cap C=A\cap (B\cap C)$$
* Distributive laws:   
  $$\quad A\cup (B\cap C)=(A\cup B)\cap (A\cup C)$$   
  $$\quad A\cap (B\cup C)=(A\cap B)\cup (A\cap C)$$

Union and intersection os sets are analogous to addition and multiplication of numbers: they are commutative and associative. Intersection distributes over union (just like mult. distributes over addition), but, unlike their arithmetic counterparts (addition doesn't distributes over mult.), union also distributes over intersection.


Two additional pairs of laws involve the special sets called the empty set Ø and the universe set U; together with the complement operator (A^{C} denotes the complement of A. This can also be written as {\displaystyle A^{')), read as A prime). The empty set has no members, and the universe set has all possible members (in a particular context).



The algebra of sets is the set-theoretic analogue of the algebra of numbers. Addition and multiplication are associative and commutative, and so are union and intersection; a relation "less than or equal" is reflexive, antisymmetric and transitive and so is the set subset relation.



## Set operations

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

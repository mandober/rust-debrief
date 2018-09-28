# Types of relations

There are many ways two sets, $$X$$ and $$Y$$, can be related to each.
- Two sets, $$X$$ and $$Y$$, can also be equal i.e. the same set, $$X$$, in which case the Cartesian product is sometimes denoted as $$X^2 = X\times X$$
- A relation is subset of the Cartesian product of sets, $$R \subseteq X\times Y$$
- A relation is an element of the powerset of the Cartesian product of two sets, $$R \in \mathcal{P}(X\times Y)$$
- Total number of relations of n-element set with itself is $$2^{n^2}$$


relation                 | notation
------------------------ | ----------
a relation               | $$R \subseteq X\times Y$$
inverse relation         | $$R'$$
non-empty relation       | $$R \neq \varnothing$$
null (empty)             | $$R = \varnothing$$
universal                | $$R = X\times Y$$
inclusion (containment)  | $$X\subseteq Y$$
equality                 | $$x = y$$
inequality               | $$x\not = y$$
less than                | $$x\lt y$$
less than or equal to    | $$x\le y$$
greater than             | $$x\gt y$$
greater than or equal to | $$x\ge y$$
reflexivity              | $$\forall x\in X:(x,x)\in R$$
irreflexivity            | $$\forall x\in X:(x,x)\not\in R$$
transitivity             | $$\forall x,y,z \in X : xRy\land yRz \to xRz$$
identity | $$\forall x\in X:(x,x)\in R$$


Relation
- any: `L`
- inverse, `L'`
- empty: `E`,
- non-empty: `R`
- universal, `U`
- identity, `Id`: Re


Properties:
- null relation
- full relation
- Reflexivity
  - reflefive, `Re`: Id+
  - non-reflefive, `!Re`
  - irreflefive, `iR`
  - non-irreflefive, `!iR`
  - coreflexive, `cR `
  - non-coreflexive, `!cR `
- Symmerty
  - symmertic, `Sy `
  - non-symmertic, `!Sy `
  - anti-symmertic, `vS `
  - non-antisymmertic, `!vS `
  - asymmertic, `aS `
  - non-asymmertic, `!aS `
- Transitivity
  - transitive, `Tr `
  - non-transitive, `!Tr `


- reflexive: `Sy+Tr+Serial`
- equivalence, `EQ` = `Re+Sy+Tr`
- partial equivalence, `pEQ`: `Sy+Tr`
- partial order: `pOrd` = `Re+vS+Tr`
- linear (total) order: partial order that is total, `Re+vS+Tr+`
- linear (total) order: partial order that is total, `Re+vS+Tr+Total`
- well-order: linear order where every non-empty subset has a least element.

- The relationship of one set being a subset of another is called inclusion or sometimes containment.


relation    |s|props
------------|-|--------
universal   |U|Re,
empty       |E|Sy,Tr

---


Some important types of binary relations $$R$$ between two sets $$X$$ and $$Y$$ (to emphasize that $$X$$ and $$Y$$ can be different sets, some authors call these heterogeneous relations):

Basic relations
- **Empty** relation between two sets is the empty set
- **Full** relation: the Cartesian product between two sets
- **Identity** relation on a set $$X^2$$ is $$R_{Id} = \{(x,x):x\in R\}$$
- **Inverse** relation, $$R'$$, of a relation $$R$$ is $$R'=\{(y,x):(x,y)\in R\}$$.

Types of relations
- Reflexive
- Irreflexive
- Coreflexive
- Symmetric
- Antisymmetric
- Asymmetric
- Transitive

Compound relations
* Equivalence


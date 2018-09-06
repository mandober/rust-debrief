# Relations

Discrete Mathematics: Relations
https://www.tutorialspoint.com/discrete_mathematics/discrete_mathematics_quick_guide.htm


## Domain and Range
If there are two sets A and B, and relation R have order pair (x, y), then
- The domain of R, Dom(R), is the set {x|(x,y)∈RforsomeyinB}
- The range of R, Ran(R), is the set {y|(x,y)∈RforsomexinA}

Examples:
Let, A={1,2,9} and B={1,3,7}
Case 1 − If relation R is 'equal to' then R={(1,1),(3,3)}
Dom(R) = {1,3},Ran(R)={1,3}
Case 2 − If relation R is 'less than' then R={(1,3),(1,7),(2,3),(2,7)}
Dom(R) = {1,2},Ran(R)={3,7}
Case 3 − If relation R is 'greater than' then R={(2,1),(9,1),(9,3),(9,7)}
Dom(R) = {2,9},Ran(R)={1,3,7}

## Representation of Relations using Graph
A relation can be represented using a directed graph.

The number of vertices in the graph is equal to the number of elements in the set from which the relation has been defined. For each ordered pair (x, y) in the relation R, there will be a directed edge from the vertex ‘x’ to vertex ‘y’. If there is an ordered pair (x, x), there will be self- loop on vertex ‘x’.

Suppose, there is a relation R={(1,1),(1,2),(3,2)} on set S={1,2,3}, it can be represented by the following graph


Types of Relations
- __Empty__ relation between sets X and Y, or on E, is the empty set ∅
- __Full__ relation between sets X and Y is the set X×Y
- **Identity** Relation on set X is the set {(x,x)|x∈X}
- **Inverse** Relation R' of a relation R is defined as − R′={(b,a)|(a,b)∈R}
  Example: If R={(1,2),(2,3)} then R′ will be {(2,1),(3,2)}
- **Reflexive**: relation R on set A is called Reflexive 
  if ∀a∈A is related to a (aRa holds)
  Example: relation R={(a,a),(b,b)} on set X={a,b} is reflexive.
- **Irreflexive**
  relation R on set A is called Irreflexive if no a∈A is related to a 
  (aRa does not hold).
  Example − The relation R={(a,b),(b,a)} on set X={a,b} is irreflexive.
- **Symmetric**
  relation R on set A is called Symmetric if xRy implies yRx, ∀x∈A and ∀y∈A.
  Example − The relation R={(1,2),(2,1),(3,2),(2,3)} on set A={1,2,3} is symmetric.
- **Antisymmetric**
  relation R on set A is called Anti-Symmetric if xRy and yRx implies x=y∀x∈A and ∀y∈A.
  Example − The relation R={(x,y)→N|x≤y} is anti-symmetric since x≤y and y≤x implies x=y.
- **Transitive**
  relation R on set A is called Transitive if xRy and yRz implies xRz,∀x,y,z∈A.
  Example − The relation R={(1,2),(2,3),(1,3)} on set A={1,2,3} is transitive.
- **Equivalence**
  relation is an Equivalence Relation if it is reflexive, symmetric, and transitive.
  Example − The relation R={(1,1),(2,2),(3,3),(1,2),(2,1),(2,3),(3,2),(1,3),(3,1)} on set A={1,2,3} is an equivalence relation since it is reflexive, symmetric, and transitive.


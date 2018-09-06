# Substructural type systems

A substructural type system restricts one or more structural rules in order to control the number of times a value may be used.

Substructural type systems are useful for constraining access to system resources by keeping track of changes and preventing invalid states.

## Structural rules
- **Weakening**    
  where the hypotheses or conclusion of a sequent may be extended with additional members.
- **Contraction**    
  where two unifiable members on the same side of a sequent may be replaced by a single member or a common instance.
- **Exchange**    
  where two members on the same side of a sequent may be swapped.
- **Cut rule**    
  is a generalisation of the modus ponens. Although suspected merely a tool for abbreviating proofs, its superfluity is unproven.

With regards to most substructural type system, the rules of weakening and contraction are controlled. The former means a value of the type may be used less than once, while the latter allows it to be used more than once.

Structural rules that are usually controlled:
- Weakening:   a value of the type can be used less than once
- Contraction: a value of the type can be used more than once

Substructural type systems:
* **Relevant**: values of relavant types must be used **at least once**
* **Linear**:   values of linear   types must be used **exactly once**
* **Affine**:   values of affine   types can  be used **once**

All the combinations of these properties gives us 4 interesting types:
1. can be used any number of times (default)
2. can't be used more than once (**affine**)
3. must be used at least once (**relevant**)
4. must be used exactly once (**linear**)

For added confusion, sometimes linear or affine is used as a synonym for the whole substructural system.


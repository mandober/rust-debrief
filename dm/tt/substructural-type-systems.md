# Substructural type systems

A substructural type system restricts one or more structural rules in order to control the number of times a value may be used.

Substructural type systems are useful for constraining access to system resources by keeping track of changes and preventing invalid states.


## Structural rules

**Exchange lemma**: if
$$Γ_{_1}, x_1:T_{_1}, x_2:T_{_2}, Γ_{_2} \vdash t:T$$ then 
$$Γ_{_1}, x_2:T_{_2}, x_1:T_{_1}, Γ_{_2} \vdash t:T$$

**Weakening lemma**: if
$$Γ_{_1},\quad \quad \quad \ Γ_2 \vdash t:T$$ then 
$$Γ_{_1}, x_1:T_{_1}, Γ_{_2} \vdash t:T$$

**Contraction lemma**: if
$$Γ_{_1}, x_2 : T_{_1}, x_3 :T_{_1}, Γ_{_2} \vdash t:T_{_2}$$ then 
$$Γ_{_1}, \quad \ x_1:T_{_1},\quad \quad Γ_{_2} \vdash [x_2,x_1][x_3,x_1]t:T_{_2}$$



- **Weakening**    
  where the hypotheses or conclusion of a sequent may be extended with additional members.
- **Contraction**    
  where two unifiable members on the same side of a sequent may be replaced by a single member or a common instance.
- **Exchange**    
  where two members on the same side of a sequent may be swapped.
- **Cut rule**    
  is a generalisation of the modus ponens. Although suspected merely a tool for abbreviating proofs, its superfluity is unproven.

With regards to most substructural type system, the rules of weakening and contraction are controlled. Restricting weakening means a value of the type may be used less than once, while restricting contraction allows a value of the type to be used more than once.

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



## Structural Properties

Basic structural properties:
1. **Weakening** indicates that adding extra, unneeded assumptions to the context, does not prevent a term from type checking.
1. **Exchange** indicates that the order in which we write down variables in the context is irrelevant. A corollary of exchange is that if we can type check a term with the context Γ, then we can type check that term with any permutation of the variables in Γ. 
1. **Contraction** states that if we can type check a term using two identical assumptions ($$x_2:T_1$$ and $$x_3:T_1$$) then we can check the same term using a single assumption.


$$
\text{Lemma [Exchange]: if}\\
Γ_{_1}, x_1:T_{_1}, x_2:T_{_2}, Γ_{_2} \vdash t:T \text{ then}\\
Γ_{_1}, x_2:T_{_2}, x_1:T_{_1}, Γ_{_2} \vdash t:T\\
\ \\
\text{Lemma [Weakening]: if}\\
Γ_{_1},\quad \quad \quad \ Γ_2 \vdash t:T \text{ then}\\
Γ_{_1}, x_1:T_{_1}, Γ_{_2} \vdash t:T\\
\ \\
\text{Lemma [Contraction]: if}\\
Γ_{_1}, x_2 : T_{_1}, x_3 :T_{_1}, Γ_{_2} \vdash t:T_{_2} \text{ then}\\
Γ_{_1}, \quad \ x_1:T_{_1},\quad \quad Γ_{_2} \vdash [x_2,x_1][x_3,x_1]t:T_{_2}
$$


Substructural type systems that restrict certain properties:
* **Linear** type systems ensure that every variable is used _exactly once_ by restricting weakening and contraction.

* **Affine** type systems ensure that every variable is used at most once by restricting contraction.

* **Relevant** type systems ensure that every variable is used at least once by restricting weakening.

* **Ordered** type systems ensure that every variable is used exactly once and in the order of declaration. Ordered type systems restrict all three structural properties.


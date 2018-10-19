# Sequent Calculus

https://en.wikipedia.org/wiki/Sequent_calculus

Sequent calculus, invented by Gerhard Gentzen, is a style of formal logical argumentation where every line of a proof, called a sequent, is a __conditional tautology__, rather then an unconditional tautology.
 
Each conditional tautology is inferred from other conditional tautologies on earlier lines in a formal argument, according to the rules of inference.


Sequent calculus generalizes the form of a natural deduction judgment to a syntactic object called a sequent:
$$A_{1}, \ldots, A_{n} \vdash B_{1}, \ldots, B_{k}$$

As in natural deduction, theorems are those $$B$$ where $$\vdash B$$ is the conclusion of a valid proof.


The sequents,     
$$A_1, \dots, A_n \vdash B_1, \dots, B_k$$, 
and     
$$\vdash (A_1 \land\dots\land A_n)\to(B_1 \lor\dots\lor B_k)$$     
are equivalent in the sense that a proof of either one may be extended to a proof of the other.


In a classical context the semantics of the sequent can also (by propositional tautology) be expressed as:

$$\vdash (\neg A_1 \lor \dots \lor \neg A_n ) \lor
(B_1 \lor \dots \lor B_k)$$

which means that it cannot be the case that all of the As (antecedents) are true and all of the Bs (succedents) are false.

```
⊢  CNF → DNF
⊢ ¬CNF ∨ DNF
 ¬¬CNF ⊢ DNF
   CNF ⊢ DNF

⊢  (A1 ∧ ⋯ ∧ An)  → (B1 ∨ ⋯ ∨ Bk)  ⟺
⊢ ¬(A1 ∧ ⋯ ∧ An)  ∨ (B1 ∨ ⋯ ∨ Bk)  ⟺
⊢ (¬A1 ∨ ⋯ ∨ ¬An) ∨ (B1 ∨ ⋯ ∨ Bk)  ⟺
    A1 ∧ ⋯ ∧ An   ⊢  B1 ∨ ⋯ ∨ Bk   ⟺
    A1 , ⋯ , An   ⊢  B1 , ⋯ , Bk
```

This means that a symmetry such as De Morgan's laws, which manifests itself as logical negation on the semantic level, translates directly into a left-right symmetry of sequents — and indeed, the inference rules in sequent calculus for dealing with conjunction are mirror images of those dealing with disjunction.

# Structural rules

The **structural rules** are rules of inference that don't operate on logical connectives, but on the structure of a deduction system directly (e.g. on judgments and sequents directly). Common structural rules are weakening, contraction, exchange, identity, cut rule, etc.

**Substructural logics**, logics that restrict one or more of the structural rules, include linear, affine, relevant, bunched and other logics.


## Weakening

**Weakening** (thinning) is a structural rule dictating that hypotheses or conclusion of a sequent may be extended with additional members.

$$\underline{Γ\ \ \vdash B\ \ }\\
Γ, A \vdash B$$



**Monotonicity of entailment** is a property of many logical systems that states that the hypotheses of any derived fact may be freely extended with additional assumptions.

In sequent calculi this property can be captured by an inference rule called **weakening** and in such systems one may say that entailment is monotone iff this rule is admissible.



In the natural deduction sequent, on the basis of a list of assumptions $$Γ$$, one can prove $$B$$; this sequent can be weakened by adding an addition assumption $$A$$.

Logical systems with this property are occasionally called **monotonic** logics in order to differentiate them from **non-monotonic** logics.


For example, the popular syllogism about mortality:

```
All men are mortal.    
Socrates is a man.    
∴ Socrates is mortal.
```

can be weakened by adding an additional premise:

```
All men are mortal.      
Socrates is a man.    
Cows produce milk.    
∴ Socrates is mortal.
```

> Validity of the original conclusion is not changed by the addition of a new premise.




**Non-monotonic logics**:       
In most logics, weakening is either an inference rule or a _metatheorem_ if the logic doesn't have an explicit rule; notable exceptions are:
- **Relevant logic** or **strict logic** where every hypothesis must be necessary for the conclusion.
- **Linear logic** which disallows arbitrary contraction in addition to arbitrary weakening.
- _Bunched implications_ where weakening is restricted to additive composition.
- Various types of default reasoning.
- _Abductive reasoning_, the process of deriving the most likely explanations of the known facts.
- _Reasoning about knowledge_, where statements specifying that something is not known need to be retracted when that thing is learned.




## Contraction

**Idempotency of entailment** is a property of logical systems that states that one may derive the same consequences from many instances of a hypothesis as from just one. This property can be captured by a structural rule called contraction, and in such systems one may say that entailment is idempotent if and only if contraction is an admissible rule.

**Contraction** rule dictates that two unifiable members on the same side of a sequent may be replaced by a single member or a common instance. 

Contraction is duplication of a formula in a cedent, or when two unifiable members on the same side of a sequent are replaced by a single member or a common instance.

In classical/intuitionistic/constructive logic, the rule of contraction allows duplicated assumptions to be contracted, or an assumption to be duplicated.

$$
\underline{Γ,\ A, \ A, \vdash B\ \ }\\
Γ, A \vdash B
$$



## Exchange
**Exchange** is swapping of two cedent formulae.

In classical/intuitionistic/constructive logic there are two rules that allow assumptions to be discarded, or introduced from nowhere: weakening and contraction.

**Exchange** ("interchange"): where two members on the same side of a sequent may be swapped.


## The cut rule

**The cut rule** is a generalisation of the modus ponens. Although suspected merely a tool for abbreviating proofs, its superfluity is unproven.




---
https://blog.tchatzigiannakis.com/structural-rules-and-substructural-logics/
https://plato.stanford.edu/entries/logic-substructural/

https://ncatlab.org/nlab/show/substructural+logic
https://ncatlab.org/nlab/show/cut+rule
https://ncatlab.org/nlab/show/contraction+rule
https://ncatlab.org/nlab/show/exchange+rule
https://ncatlab.org/nlab/show/weakening+rule

https://en.wikipedia.org/wiki/Structural_proof_theory
https://en.wikipedia.org/wiki/Cut-elimination
https://en.wikipedia.org/wiki/Natural_deduction
https://en.wikipedia.org/wiki/Analytic_proof
https://en.wikipedia.org/wiki/Logical_harmony
https://en.wikipedia.org/wiki/Hypersequent
https://en.wikipedia.org/wiki/Calculus_of_structures
https://en.wikipedia.org/wiki/Nested_sequent_calculus
https://en.wikipedia.org/wiki/Monotonicity_of_entailment
https://en.wikipedia.org/wiki/Substructural_logic

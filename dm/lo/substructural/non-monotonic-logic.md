# Non-monotonic logics

A non-monotonic logic (NML) is a formal logic whose **consequence relation** is not monotonic.

Non-monotonic logics are devised to capture and represent **defeasible inferences** i.e. inferences used to draw provisional conclusions that can be retracted based on new evidence.

Most formal logics have a **monotonic consequence relation** - adding a formula to a theory never produces a retraction of its set of consequences.

> Intuitively, **monotonicity** indicates that a new piece of knowledge cannot redact the set of existing knowledge.

A **monotonic logic** can't handle various reasoning tasks including
- **Reasoning by default**: consequences may be derived only because evidence of the contrary is lacking.
- **Abductive reasoning**: consequences are only deduced as most likely explanations.
- **Reasoning about knowledge**: the ignorance of a consequence must be retracted when the consequence becomes known.
- **Belief revision**: new knowledge may contradict old beliefs.

The challenge tackled in the domain of NMLs is to provide for defeasible reasoning forms what classical logic or intuitionistic logic provide for mathematical reasoning: namely, a formally precise account that is materially adequate, where material adequacy concerns the question of how broad a range of examples is captured by the framework, and the extent to which the framework can do justice to our intuitions on the subject (at least the most entrenched ones).



## Abductive reasoning
**Abductive reasoning** is the process of deriving the most likely explanations of the known facts.

An **abductive logic** should not be monotonic because the most likely explanations are not necessarily correct.

For example, the most likely explanation for seeing wet grass is that it rained; however, this explanation has to be retracted when learning that the real cause of the grass being wet was a sprinkler.

Since the old explanation is retracted because of the addition of a new piece of knowledge, any logic that models explanations is non-monotonic.


## Reasoning about knowledge
If a logic includes formulae that mean that something is not known, this logic should not be monotonic.

Indeed, learning something that was previously not known leads to the removal of the formula specifying that this piece of knowledge is not known.

This second change (a removal caused by an addition) violates the condition of monotonicity.

A logic for reasoning about knowledge is the **autoepistemic logic**.


## Belief revision
**Belief revision** is the process of changing beliefs to accommodate a new belief that might be inconsistent with the old ones.

In the assumption that the new belief is correct, some of the old ones have to be retracted in order to maintain consistency.

This retraction in response to an addition of a new belief makes any logic for belief revision to be non-monotonic.

The belief revision approach is alternative to **paraconsistent logics**, which tolerate inconsistency rather than attempting to remove it.


## Proof-theoretic vs model-theoretic

Proof-theoretic versus model-theoretic formalizations of non-monotonic logics

**Proof-theoretic** formalization of a non-monotonic logic begins with adoption of certain non-monotonic rules of inference, and then prescribes contexts in which these non-monotonic rules may be applied in admissible deductions.

This typically is accomplished by means of **fixed-point equations** that relate the sets of premises and the sets of their non-monotonic conclusions.

**Default logic** and **autoepistemic logic** are the most common examples of non-monotonic logics that have been formalized that way.


**Model-theoretic formalization** of a non-monotonic logic begins with restriction of the semantics of a suitable monotonic logic to some special models, for instance, to minimal models, and then derives the set of non-monotonic rules of inference, possibly with some restrictions in which contexts these rules may be applied, so that the resulting deductive system is sound and complete with respect to the restricted semantics.

Unlike some proof-theoretic formalizations that suffered from well-known paradoxes and were often hard to evaluate with respect of their consistency with the intuitions they were supposed to capture, model-theoretic formalizations were paradox-free and left little, if any, room for confusion about what non-monotonic patterns of reasoning they covered.

Examples of proof-theoretic formalizations of non-monotonic reasoning, which revealed some undesirable or paradoxical properties or did not capture the desired intuitive comprehensions, that have been successfully (consistent with respective intuitive comprehensions and with no paradoxical properties, that is) formalized by model-theoretic means include first-order circumscription, closed-world assumption, and autoepistemic logic.



---

**References**   
- https://www.wikiwand.com/en/Non-monotonic_logic
- https://plato.stanford.edu/entries/logic-nonmonotonic/

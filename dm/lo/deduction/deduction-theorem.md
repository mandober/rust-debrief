# Deduction Theorem

https://en.wikipedia.org/wiki/Deduction_theorem

The deduction theorem states that a sentence of the form $$\phi \to \psi$$ is provable from a set of axioms $$A$$ iff the sentence $$\psi$$ is provable from the system whose axioms consist of $$\phi$$ and all the axioms of $$A$$.


The deduction theorem, a metatheorem of propositional and first-order logic, is a formalization of the common proof technique in which an implication $$A \to B$$ is proved by assuming $$A$$ and then deriving $$B$$ from this assumption conjoined with known results.

The deduction theorem explains why proofs of conditional sentences in mathematics are logically correct.

Though it seemed "obvious" to mathematicians for centuries 
that proving B from A conjoined with a set of theorems is tantamount to proving the implication A → B based on those theorems alone, it was left to Herbrand and Tarski to show (independently) this was logically correct in the general case.


The deduction theorem states that if a formula $$B$$ is deducible from a set of assumptions $$\Delta \cup \{A\}$$, where $$A$$ is a closed formula, then the implication $$A \to B$$ is deducible from $$\Delta$$.

In symbols, $$\Delta \cup \{A\}\vdash B$$ implies $$\Delta \vdash A\to B$$.

In the special case where $$\Delta$$ is the empty set, the deduction theorem shows that 
$$\{A\}\vdash B$$ implies $$\vdash A\to B$$.


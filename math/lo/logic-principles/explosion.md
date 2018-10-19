# The principle of explosion

**Ex falso quodlibet**, `EFQ`

The principle of explosion states that any statement can be proven from a contradiction. That is, once a contradiction has been asserted, any proposition (including their negations) can be inferred from it. This is known as **deductive explosion**. This principle is recognized in classical, intuitionistic and similar logical systems.

For any statements $$p$$ and $$q$$, if $$p$$ and $$\lnot p$$ are both true, then $$q$$ is true:

$$\forall p \forall q : (p\land \lnot p) \vdash q$$

Formal proof of the principle:

| line | derivation              | justification     |
|:-----|:------------------------|------------------:|
| 1    | $$\quad P\land \neg P$$ | assumption        |
| 2    | $$\quad \quad P$$       | $${\land e_1}$$ 1 |
| 3    | $$\quad \quad \neg P$$  | $${\land e_2}$$ 1 |
| 4    | $$\quad \quad P\lor Q$$ | $${\lor i}$$  2   |
| 5    | $$\quad Q$$             | DM 3,4            |
| 6    | $$P\land \neg P \to Q$$ | $${\to i}$$  1-5  |



<details><summary>Example</summary><br>

For example, we can prove "_if all lemons are yellow and all lemons are not yellow, then unicorns exist_". Let $$P$$ stands for "_all lemons are yellow_" and $$Q$$ for "_unicorns exist_":    
$$P\land\neg P \vdash Q$$

We need to start from the assumption, $$P\land\neg P$$, and arrive to the conclusion, $$Q$$.

- From the assumption, $$P\land\neg P$$ i.e.    
  "_all lemons are yellow and not all lemons are yellow_" (line 1)
- we can infer, using and-elimination-right, that    
  "_all lemons are yellow_" (2)
- also, using and-elimination-left, that    
  "_not all lemons are yellow_" (3)
- From "_all lemons are yellow_" (2), we infer "_all lemons are yellow or unicorns exist_" (4) using or-intro ($$T\lor x$$ is still true for any $$x$$)
- From "_not all lemons are yellow_" (3) and "_all lemons are yellow or unicorns exist_" (4) we infer that "_unicorns exist_" (5), using disjunctive syllogism.
- Therefore, if all lemons are yellow and not all lemons are yellow, then unicorns exist.

</details><br>


Due to the principle of explosion, the existence of a contradiction (inconsistency) in a formal axiomatic system is disastrous; since any statement can be proved true it trivializes the concepts of truth and falsity. In a different solution to this problems, a few mathematicians have devised alternate theories of logic called **paraconsistent logics**, which eliminate the principle of explosion. These allow some contradictory statements to be proved without affecting other proofs.


## Semantic argument
An alternate argument for the principle stems from model theory.

A sentence P is a semantic consequence of a set of sentences \Gamma  only if every model of \Gamma is a model of P. But there is no model of the contradictory set {\displaystyle (P\wedge \lnot P)}. 

A fortiori, there is no model of {\displaystyle (P\wedge \lnot P)} that is not a model of Q.

Thus, vacuously, every model of {\displaystyle (P\wedge \lnot P)} is a model of Q. Thus Q is a semantic consequence of {\displaystyle (P\wedge \lnot P)}.


## Paraconsistent logic
Paraconsistent logics have been developed that allow for sub-contrary forming operators. Model-theoretic paraconsistent logicians often deny the assumption that there can be no model of \{\phi , \lnot \phi \} and devise semantical systems in which there are such models. Alternatively, they reject the idea that propositions can be classified as true or false. Proof-theoretic paraconsistent logics usually deny the validity of one of the steps necessary for deriving an explosion, typically including disjunctive syllogism, disjunction introduction, and reductio ad absurdum.


## Use
The metamathematical value of the principle of explosion is that for any logical system where this principle holds, any derived theory which proves ⊥ (or an equivalent form, \phi \land \lnot \phi) is worthless because all its statements would become theorems, making it impossible to distinguish truth from falsehood. That is to say, the principle of explosion is an argument for the law of non-contradiction in classical logic, because without it all truth statements become meaningless.


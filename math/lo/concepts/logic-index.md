# Logic Indices


* Logical reasoning
  - Deduction
  - Induction
  - Abduction

* Structural rules
  - Monotonicity of entailment, weakening
  - Idempotency of entailment, contraction
  - Exchange
  - The cut rule
  - De Morgan duality

* Properties
  - Satisfiability
  - Validity
  - Soundness
  - Well-formedness
  - Compactness
  - Completeness
  - Consistency
  - Truth functional (operators)
  - Defeasibility
  - Substitution

* Logical connectives
  - negation
  - conjunction
  - disjunction
  - implication
  - bicondition
  - Sheffer's stroke
  - Pierce's arrow
  - XOR


* Principles
  - Law of identity, `ID`, $$\forall x: x=x$$     
    $${p\land\top\equiv p},\ {p\lor\bot\equiv p}$$
  - Law of non-contradiction, `NC`: $$\lnot(p\land \lnot p)$$
  - Law of excluded middle, `EM`, _Tertium non datur_, `TND`: $$p\lor \lnot p$$    
    $$\top\equiv\lnot\lnot\top\equiv\lnot\bot, \quad \bot\equiv\lnot\lnot\bot\equiv\lnot\top$$
  - Principle of explosion, _Ex falso quodlibet_, `EFQ`     
    $$\forall p \forall q : (p\land \lnot p) \vdash q$$
  - Principles of bivalence: $$\top\lor\bot$$, not both, not neither
  - Independence of premise, Kreisel–Putnam rule, `KPR`
  - Negation as failure, `NAF`

* Rules
  - Commutativity
    - Conjunction: $$p\land q \vdash q \land p$$
    - Disjunction: $$p\lor q \vdash q \lor p$$
  - Associativity
    - Conjunction: $$p\land (q\land r) \vdash (p\land q)\land r$$
    - Disjunction: $$p\lor (q\lor r) \vdash (p\lor q)\lor r$$
  - Distributivity:
    - $$p\land (q\lor r) \vdash (p\land q) \lor (p\land r)$$
    - $$p\vee (q\land r) \vdash (p\vee q) \land (p\vee r)$$
  - Absorption: $$p\to q\vdash p\to (p\land q)$$
  - De Morgan's laws
    - Negation of conjunction: $$\neg (p\land q) \vdash (\neg p \lor \neg q)$$
    - negation of disjunction: $$\neg (p\lor q)\vdash (\neg p\land \neg q)$$
  - Material implication: $$p\to q \equiv \lnot p\lor q\ _{(MI)}$$
  - Idempotency
  - Domination laws: $${p\lor\top\equiv \top},\ {p\land\bot\equiv \bot}$$
  - Negation laws
  - Double negation
  - Transposition
  - Material implication
  - Exportation
  - Tautology
  - Negation introduction

* Inference
  - Derivability, derived rule
  - Admissibility, Admissible rule
  - Discharged assumption
  - Conditional proof assumption, `CPA`

* Inference rules

  * **Negations**
    - Negation
      - not-introduction, _Reductio ad absurdum_, $$p\to q, p\to\neg q \vdash\neg p\ _{(\lnot i)}$$
      - not-elimination, Noncontradiction, $$\neg p\vdash p\to r\ _{(\lnot e)}$$
    - Double negation (depends on `EM`)
      - DN-introduction, $$p \vdash \lnot \lnot p \ _{(\lnot\lnot i)}$$
      - DN-elimination, $$\lnot \lnot p\vdash p\ _{(\lnot \lnot e)}$$
    - De Morgan's laws
      - Negation of conjunction: $$\neg (p\land q) \vdash (\neg p \lor \neg q)\ _{(DM)}$$
      - Negation of disjunction: $$\neg (p\lor q)\vdash (\neg p\land \neg q)\ _{(DM)}$$

  * **Conjunction**
    - and-introduction, Adjunction: $$p,q\vdash p\land q\ _{\land i}$$
    - and-elimination, Simplification: $$p\land q\vdash p\ _{(\land e_1)}$$ and $$p\land q\vdash q\ _{(\land e_2)}$$
    - Commutativity: $$p\land q \vdash q \land p$$
    - Associativity: $$p\land (q\land r) \vdash (p\land q)$$
    - De Morgan's law: $$\neg p \land \neg q \vdash \neg (p\lor q) \ _{(DM)}$$

  * **Disjunction**
    - or-introduction: $$p\vdash p\lor q\ _{(\lor i)}$$
    - or-elimination: $$p\lor q,p\to r,q\to r \vdash r\ _{(\lor e)}$$
    - Disjunctive syllogism, `DS`: $$p\lor q,\lnot q\vdash p\ _{(DS)}$$
    - Commutativity: $$p\lor q \vdash q \lor p$$
    - Associativity: $$p\lor (q\lor r) \vdash (p\lor q)$$
    - De Morgan's law: $$\neg p \lor \neg q \vdash \neg (p\land q) \ _{(DM)}$$
    - Material implication: $$\lnot p\lor q \vdash p\to q\ _{(MI)}$$

  * **Implication**
    - _Modus ponens_, `MP`, $$p\to q, p \vdash q\ _{(MP)}$$
    - _Modus tollens_, `MT`, $$p\to q,\lnot q \vdash\lnot p\ _{(MT)}$$
    - Material implication: $$p\to q \vdash \lnot p\lor q\ _{(MI)}$$
    - Hypothetical syllogism: $$p\to q, q\to r \vdash p\to r\ _{(HS)}$$
    - Implication introduction in conditional proof:
      $$(p \vdash q) \vdash p\to q\ _{(\to i)}$$
    - Reflexivity: $$p \vdash p\to p$$
    - Absorption: $$p\to q \vdash p\to (p \to q)$$

  * **Biconditional**
    - iff-introduction: $$p\to q, q\to p \vdash p\leftrightarrow q\ _{(\leftrightarrow i)}$$
    - iff-elimination: $$p\leftrightarrow q\vdash p\to q,q\to p\ \ _{(\leftrightarrow e)}$$

  * **Universal quantifier**
    - ∀-introduction, Generalization, `GEN`
    - ∀-elimination
    - De Morgan's: $$\forall x P(x) \equiv \neg (\exists x\,\neg P(x))$$

  * **Existential quantifier**
    - ∃-introduction
    - ∃-elimination
    - De Morgan's: $$\exists x P(x)\equiv \neg (\forall x\,\neg P(x))$$

* Reiteration, Copy, `CPY` 
* _Modus ponendo tollens_, `MPT`
* Deduction theorem
- Constructive dilemma
- Destructive dilemma


---

<kbd>ID</kbd> 
<kbd>NC</kbd> 
<kbd>EM</kbd> 
<kbd>EFQ</kbd> 
<kbd>KPR</kbd> 
<kbd>NAF</kbd> 

<kbd>MP</kbd> 
<kbd>MT</kbd> 

<kbd>NOT</kbd> 
<kbd>AND</kbd> 
<kbd>OR</kbd> 
<kbd>TO</kbd> 
<kbd>IFF</kbd> 

<kbd>NAND</kbd> 
<kbd>NOR</kbd> 
<kbd>XOR</kbd> 
<kbd>XNOR</kbd> 

# Properties of logic systems
<!-- TOC -->

- [Law of identity](#law-of-identity)
- [Law of non-contradiction](#law-of-non-contradiction)
- [Law of excluded Middle](#law-of-excluded-middle)
- [Principle of explosion](#principle-of-explosion)
- [Negation as failure](#negation-as-failure)
- [The principles of bivalence](#the-principles-of-bivalence)

<!-- /TOC -->


## Law of identity
- The law of identity, `ID`
- $$\forall x: x = x$$
- "_Everything is. Everything is identical to itself._"
- B.Russell in PM, 1912: "_Whatever is, is_"
- Every proposition implies itself.


## Law of non-contradiction
- The law of (non)contradiction, `NC`
- $$\lnot(p\land \lnot p)$$
- Also known as the law of contradiction
- No thing having a given quality also has the negative of that quality.
- No proposition is both true and false.
- B.Russell in PM, 1912: "Nothing can both be and not be".
- Two or more contradictory statements cannot both be true in the same sense at the same time.
- Aristotle: "one cannot say of something that it is and that it is not in the same respect and at the same time".


## Law of excluded Middle
- The law of excluded middle, `EM`
- $$p\lor \lnot p$$
- Every thing either has a given quality or the negative of that quality.
- Every proposition is true or false (not both and not nether)
- B.Russell in PM, 1912: "Everything must either be or not be".
- for every proposition, either its positive or negative form is true.
- $$\forall P: P\chi \lor \lnot P\chi$$


## Principle of explosion
_Ex falso quodlibet_, (`EFQ`) or _Ex contradictione quodlibet_ (ECQ)    
The principle of explosion states that any statement can be proven from a contradiction. That is, once a contradiction has been asserted, any proposition (including their negations) can be inferred from it. This is known as deductive explosion. It is recognized on classical, intuitionistic and similar logical systems.


## Negation as failure
Negation as failure, `NAF`, is a non-monotonic inference rule in logic programming, used to derive $$\mathrm{not} \sim p$$ (i.e. that $$\sim p$$ is assumed not to hold) from failure to derive $$\sim p$$.

Note that $$\mathrm{not} \sim p$$ can be different from the statement $$\lnot p$$ of the logical negation of $$\sim p$$, depending on the completeness of the inference algorithm and thus also on the formal logic system.

Negation as failure has been an important feature of logic programming since the earliest days of both Planner and Prolog. In Prolog, it is usually implemented using Prolog's extralogical constructs.



## The principles of bivalence
Something is either true or false, not both, not neither.


**Independence of premise**      
Independence of premise states that if φ and ∃ x θ are sentences in a formal theory and φ → ∃ x θ is provable, then ∃ x (φ → θ) is provable. Here x cannot be a free variable of φ.

The principle is valid in classical logic. Its main application is in the study of intuitionistic logic, where the principle is not always valid.



**Commutativity of conjunction**
In propositional logic, the commutativity of conjunction is a valid argument form and truth-functional tautology. It is considered to be a law of classical logic.

**Constructive dilemma**
Constructive dilemma is a valid rule of inference of propositional logic. It is the inference that, if P implies Q and R implies S and either P or R is true, then Q or S has to be true.

**Contraposition**
In traditional logic, contraposition is a form of immediate inference in which a proposition is inferred from another and where the former has for its subject the contradictory of the original logical proposition's predicate.

**Converse**
In logic, the converse of a categorical or implicational statement is the result of reversing its two parts. For the implication P → Q, the converse is Q → P. For the categorical proposition All S are P, the converse is All P are S. In neither case does the converse necessarily follow from the original statement.

**De Morgan's laws**
In propositional logic and boolean algebra, De Morgan's laws are a pair of transformation rules that are both valid rules of inference. They are named after Augustus De Morgan, a 19th-century British mathematician.

**Destructive dilemma**
Destructive dilemma is the name of a valid rule of inference of propositional logic. It is the inference that, if P implies Q and R implies S and either Q is false or S is false, then either P or R must be false.

**Disjunctive syllogism**
In classical logic, disjunctive syllogism (historically known as modus tollendo ponens (MTP), Latin for "mode that affirms by denying") is a valid argument form which is a syllogism having a disjunctive statement for one of its premises.An example in English: The breach is a safety violation, or it is not subject to fines. The breach is not a safety violation.

**Double negation**
In propositional logic, double negation is the theorem that states that "If a statement is true, then it is not the case that the statement is not true." This is expressed by saying that a proposition A is logically equivalent to not (not-A), or by the formula A ≡ ~(~A) where the sign ≡ expresses logical equivalence and the sign ~ expresses negation.Like the law of the excluded middle, this principle is considered to be a law of thought in classical logic, but it is disallowed by intuitionistic logic. The principle was stated as a theorem of propositional logic by Russell and Whitehead in Principia Mathematica as: ∗ 4 ⋅ 13.

**Universal generalization**
In predicate logic, generalization (also universal generalization or universal introduction, GEN) is a valid inference rule. It states that if ⊢ P ( x ) {\displaystyle \vdash P(x)} has been derived, then ⊢ ∀ x P ( x ) {\displaystyle \vdash \forall x\,P(x)} can be derived.

**Hypothetical syllogism**
In classical logic, hypothetical syllogism is a valid argument form which is a syllogism having a conditional statement for one or both of its premises. An example in English: If I do not wake up, then I cannot go to work.

**Law of excluded middle**
In logic, the law of excluded middle (or the principle of excluded middle) states that for any proposition, either that proposition is true or its negation is true. It is the third of the three classic laws of thought.

**Law of noncontradiction**
In classical logic, the law of non-contradiction (LNC) (also known as the law of contradiction, principle of non-contradiction (PNC), or the principle of contradiction) states that contradictory statements cannot both be true in the same sense at the same time, e.g. the two propositions "A is B" and "A is not B" are mutually exclusive.

**Modus ponendo tollens**
Modus ponendo tollens (MPT; Latin: "mode that denies by affirming") is a valid rule of inference for propositional logic. It is closely related to modus ponens and modus tollens.

**Modus ponens**
In propositional logic, modus ponens (; MP; also modus ponendo ponens (Latin for "mode that affirms by affirming") or implication elimination) is a rule of inference. It can be summarized as "P implies Q and P is asserted to be true, therefore Q must be true." Modus ponens is closely related to another valid form of argument, modus tollens.

**Modus tollens**
In propositional logic, modus tollens (; MT; also modus tollendo tollens (Latin for "mode that denies by denying") or denying the consequent) is a valid argument form and a rule of inference. It is an application of the general truth that if a statement is true, then so is its contra-positive.

**Obversion**
In traditional logic, obversion is a "type of immediate inference in which from a given proposition another proposition is inferred whose subject is the same as the original subject, whose predicate is the contradictory of the original predicate, and whose quality is affirmative if the original proposition's quality was negative and vice versa". The quality of the inferred categorical proposition is changed but the truth value is the same to the original proposition.

**Simplification**
(Conjunction elimination) In propositional logic, conjunction elimination (also called and elimination, ∧ elimination, or simplification) is a valid immediate inference, argument form and rule of inference which makes the inference that, if the conjunction A and B is true, then A is true, and B is true. The rule makes it possible to shorten longer proofs by deriving one of the conjuncts of a conjunction on a line by itself.

**Transposition**
In propositional logic, transposition is a valid rule of replacement that permits one to switch the antecedent with the consequent of a conditional statement in a logical proof if they are also both negated. It is the inference from the truth of "A implies B" the truth of "Not-B implies not-A", and conversely.

**Absorption law**
In algebra, the absorption law or absorption identity is an identity linking a pair of binary operations. Two binary operations, ¤ and ⁂, are said to be connected by the absorption law if: a ¤ (a ⁂ b) = a ⁂ (a ¤ b) = a.A set equipped with two commutative, associative and idempotent binary operations ∨ {\displaystyle \scriptstyle \lor } ("join") and ∧ {\displaystyle \scriptstyle \land } ("meet") that are connected by the absorption law is called a lattice.

**Intermediate logic**
In mathematical logic, a superintuitionistic logic is a propositional logic extending intuitionistic logic. Classical logic is the strongest consistent superintuitionistic logic; thus, consistent superintuitionistic logics are called intermediate logics (the logics are intermediate between intuitionistic logic and classical logic).

**Strict conditional**
In logic, a strict conditional is a conditional governed by a modal operator, that is, a logical connective of modal logic. It is logically equivalent to the material conditional of classical logic, combined with the necessity operator from modal logic.

**Zeroth-order logic**
Zeroth-order logic is first-order logic without variables or quantifiers. Some authors use the phrase "zeroth-order logic" as a synonym for the propositional calculus, but an alternative definition extends propositional logic by adding constants, operations, and relations on non-Boolean values.



**Soundness**       
A logic is a set of rules which allow inference from a group of sentences in some language to a new sentence. If the logic is **sound**, then these rules correspond to the meanings of the sentences of the language in the following way: the rules preserve truth.
This means that whenever the rules permit drawing some conclusion C from a set of premises S, then if all the sentences in S are true, then C will also be true. Put another way, in a sound logic the rules only admit inference from S to C if whenever the truth conditions for all the sentences in S are satisfied, then so will be the truth conditions for C, in more jargon, the conjunction of the truth conditions for the sentences in S entails the truth condition for C.

**Consistency**
In classical deductive logic, a consistent theory is one that does not contain a contradiction. The lack of contradiction can be defined in either semantic or syntactic terms.

**Completeness**
In mathematical logic and metalogic, a formal system is called complete with respect to a particular property if every formula having the property can be derived using that system, i.e. is one of its theorems; otherwise the system is said to be incomplete.

**Functional completeness**
In logic, a functionally complete set of logical connectives or Boolean operators is one which can be used to express all possible truth tables by combining members of the set into a Boolean expression. A well-known complete set of connectives is { AND, NOT }, consisting of binary conjunction and negation.

**Resolution**
In mathematical logic and automated theorem proving, resolution is a rule of inference leading to a refutation theorem-proving technique for sentences in propositional logic and first-order logic. In other words, iteratively applying the resolution rule in a suitable way allows for telling whether a propositional formula is satisfiable and for proving that a first-order formula is unsatisfiable.




**Characteristic properties of classical logic**
- Law of excluded middle
  - double negative
- Law of noncontradiction
- The principle of explosion
- Monotonicity of entailment
- Idempotency of entailment
- Commutativity of conjunction
- De Morgan duality: every logical operator is dual to another


**Boolean logic**
In mathematics and mathematical logic, Boolean algebra is the branch of algebra in which the values of the variables are the truth values true and false, usually denoted 1 and 0 respectively. Instead of elementary algebra where the values of the variables are numbers, and the prime operations are addition and multiplication, the main operations of Boolean algebra are the conjunction and denoted as ∧, the disjunction or denoted as ∨, and the negation not denoted as ¬.





























---

https://www.wikiwand.com/en/Law_of_thought#/The_three_traditional_laws
https://www.wikiwand.com/en/Identity_(philosophy)
https://www.wikiwand.com/en/Law_of_identity
https://www.wikiwand.com/en/Law_of_contradiction
https://www.wikiwand.com/en/Law_of_excluded_middle
https://www.wikiwand.com/en/Identity_of_indiscernibles
https://www.wikiwand.com/en/Dictum_de_omni_et_nullo
https://www.wikiwand.com/en/Independence_of_premise
https://www.wikiwand.com/en/Principle_of_bivalence
https://www.wikiwand.com/en/De_Morgan%27s_laws
https://www.wikiwand.com/en/Double_negation
https://www.wikiwand.com/en/Principle_of_explosion
https://www.wikiwand.com/en/Commutativity_of_conjunction
https://www.wikiwand.com/en/Classical_logic
https://www.wikiwand.com/en/Truth_function
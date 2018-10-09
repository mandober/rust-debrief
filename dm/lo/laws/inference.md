# Logical Inference

<!-- TOC -->

- [Inference](#inference)
- [Admissible rule](#admissible-rule)
- [Rule of inference](#rule-of-inference)
- [The standard form](#the-standard-form)
- [Hilbert systems](#hilbert-systems)
- [Admissibility and derivability](#admissibility-and-derivability)

<!-- /TOC -->



## Inference
Inferences are steps in reasoning, moving from premises to logical consequences. They can be divided into 3 kinds: 
- deduction
- induction
- abduction

Formal logic is the study of inference with purely formal content. An inference possesses a purely formal content if it can be expressed as a particular application of a wholly abstract rule i.e. a rule that is not about any particular thing or property. In many definitions of logic, logical inference and inference with purely formal content are the same.

Symbolic logic studies symbolic abstractions that capture the formal features of logical inference.


Inference must not to be confused with implication; an **implication** is a sentence of the form "if p then q" that can be true or false. The Stoic logician Philo of Megara was the first to define the truth conditions of such an implication: false only when the antecedent p is true and the consequent q is false, in all other cases true.

An **inference**, on the other hand, consists of two separately asserted propositions of the form "p therefore q". An inference is not true or false, but valid or invalid. However, there is a connection between implication and inference: if the implication "if p then q" is true, the inference "p therefore q" is valid.

This was given an apparently paradoxical formulation by Philo, who said that the implication "_if it is day, it is night_" is true only at night, so the inference "_it is day, therefore it is night_" is valid at nighttime, but not during daytime.



The theory of inference (or 'consequences') was systematically developed in medieval times by logicians such as William of Ockham42 and Walter Burley64. It is uniquely medieval, though it has its origins in Aristotle's Topics65 and Boethius66' _De Syllogismis hypotheticis_. This is why many terms in logic are Latin. For example, the rule that licenses the move from the implication 'if p then q' plus the assertion of its antecedent p, to the assertion of the consequent q is known as modus ponens67 (or 'mode of positing'). Its Latin formulation is 'Posito antecedente ponitur consequens'. The Latin formulations of many other rules such as 'ex falso quodlibet' (anything follows from a falsehood), 'reductio ad absurdum' (disproof by showing the consequence is absurd) also date from this period. 

However, the theory of consequences, or of the so-called 'hypothetical syllogism68' was never fully integrated into the theory of the 'categorical syllogism'. This was partly because of the resistance to reducing the categorical judgment 'Every S is P' to the so-called hypothetical judgment 'if anything is S, it is P'. The first was thought to imply 'some S is P', the second was not, and as late as 1911 in the Encyclopædia Britannica69 article on Logic, we find the Oxford logician T.H. Case arguing against Sigwart's and Brentano's modern analysis of the universal proposition. 


## Admissible rule
https://en.wikipedia.org/wiki/Admissible_rule
In logic, a rule of inference is admissible in a formal system if the set of theorems of the system does not change when that rule is added to the existing rules of the system. In other words, every formula that can be derived using that rule is already derivable without that rule, so, in a sense, it is redundant. 


## Rule of inference
A rule of inference is a logical form consisting of a function which takes premises, analyzes their syntax, and returns a conclusion(s).

The rule is __valid__ with respect to the semantics of classical (and many other) logics, in the sense that if the premises are true, then so is the conclusion.

Typically, a rule of inference preserves truth, which is a semantic property. In many-valued logic, it preserves a general designation.

A rule of inference's action is purely syntactic, and does not need to preserve any semantic property: any function from sets of formulae to formulae 
counts as a rule of inference.

Usually only rules that are recursive are important; i.e. rules such that there is an effective procedure for determining whether any given formula is the conclusion of a given set of formulae according to the rule. An example of a rule that is not effective in this sense is the __infinitary ω-rule__.


## The standard form

In formal logic rules of inference are usually given in the standard form:

$$
\text{Premise}_1\\
\text{Premise}_2\\
\vdots\\
\underline{\text{Premise}_n}\\
\text{Conclusion}
$$

This expression states that whenever in the course of some logical derivation the given premises have been obtained, the specified conclusion can be taken for granted as well.


The exact formal language that is used to describe premises and conclusions depends on the actual context of the derivations. In a simple case, one may use logical formulae, such as in:

$$
\phi\to \psi\\
{\underline {\phi\quad \quad}}\\
\quad \psi
$$

This is the _modus ponens_ rule of propositional logic. Rules of inference are often formulated as _schemata_ employing _metavariables_. In the rule  above, the metavariables $$\phi$$ and $$\psi$$ can be instantiated to any element of the universe (or sometimes, by convention, a restricted subset such as propositions) to form an infinite set of inference rules.

A proof system is formed from a set of rules chained together to form proofs, also called _derivations_. Any derivation has only one final conclusion, which is the statement proved or derived.


## Hilbert systems

In a Hilbert system, the premises and conclusion of the inference rules are simply formulae of some language, usually employing metavariables.

The formal language for classical propositional logic can be expressed using just negation, implication and propositional symbols.

A well-known axiomatization, comprising 3 axiom schemata and 1 inference rule is:

    
    (CA1) ⊢ A → (B → A)  
    (CA2) ⊢ (A → (B → C)) → ((A → B) → (A → C))  
    (CA3) ⊢ (¬A → ¬B) → (B → A)  
    (MP)  A → B, A ⊢ B
    

> It may seem redundant to have two notions of inference, `⊢` and `→`. In classical propositional logic, they do coincide; the deduction theorem states that `A ⊢ B` if and only if `⊢ A → B`. However, there is a distinction: the first notation describes a deduction, that is an activity of passing from sentences to sentences, whereas `A → B` is simply a formula made with a logical connective.

For some non-classical logics, the deduction theorem does not hold. For example, the _three-valued logic_ of Łukasiewicz can be axiomatized as:

    
    (CA1) ⊢ A → (B → A)  
    (LA2) ⊢ (A → B) → ((B → C) → (A → C))  
    (CA3) ⊢ (¬A → ¬B) → (B → A)  
    (LA4) ⊢ ((A → ¬A) → A) → A  
    (MP)  A → B, A ⊢ B
    

The classical deduction theorem does not hold for this logic, however a modified form does: `A ⊢ B` iff `⊢ A → (A → B)`.



## Admissibility and derivability

In a set of rules, an inference rule could be redundant in the sense that it is _admissible_ or _derivable_. A derivable rule is one whose conclusion can be derived from its premises using the other rules.

An __admissible rule__ is one whose conclusion holds whenever the premises hold. All derivable rules are admissible.

To appreciate the difference, consider the following set of rules for defining the natural numbers:

$$
\begin{matrix}
\frac{}{0\ \mathbb{N}} &
\frac{n\ \mathbb{N}}{succ(n)\ \mathbb{N}}
\end{matrix}
$$

The first rule states that 0 is a natural number, and the second states that `succ(n)` is a natural number if `n` is.

In this proof system, the following rule, demonstrating that the second successor of a natural number is also a natural number, is derivable:

$$
\frac{n\ \mathbb{N}}{succ(succ(n))\ \mathbb{N}}
$$

Its derivation is the composition of 2 uses of the successor rule above.

The following rule for asserting the existence of a predecessor for any nonzero number is merely admissible:

$$
\frac{succ(n)\ \mathbb{N}}{n\ \mathbb{N}}
$$

This is a true fact of natural numbers, as can be proven by induction.

To prove that this rule is admissible, assume a derivation of the premise and induct on it to produce a derivation of $$n\ \mathbb{N}$$.

However, it is __not derivable, because it depends on the structure of the derivation of the premise__.

Because of this, __derivability is stable under additions to the proof system, whereas admissibility is not__.


To see the difference, suppose the following nonsense rule were added to the proof system:

$$
\frac{}{succ(-3)\ \mathbb{N}}
$$

In this new system, the double-successor rule is still derivable. However, the rule for finding the predecessor is no longer admissible, because there is no way to derive $$-3\ \mathbb{N}$$.

The brittleness of admissibility comes from the way it is proved: since the proof can induct on the structure of the derivations of the premises, extensions to the system add new cases to this proof, which may no longer hold.

Admissible rules can be thought of as theorems of a proof system. For instance, in a sequent calculus where _cut elimination_ holds, the _cut rule_ is admissible.


---

https://en.wikipedia.org/wiki/Rule_of_inference




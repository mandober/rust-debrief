# Logics

Logic
- Classical logic
- Non-classical logic

Logic
- Informal logic
- Formal logic

Formal logic
- Symbolic logic
- Mathematical logic

Symbolic logic
- Propositional (sentential, zeroth-order) logic)
- Predicate (first-order) logic
- Higher-order logic

Mathematical logic
- model theory
- proof theory
- set theory
- computability (recursion) theory

Logics in CS
- Modal logic
- Temporal logic
- Fuzzy logic
- Probabilistic logic
- Nonmonotonic logic

Variants
- propositional
- first-order
- higher-order
- intuitionistic
- combinations of these
- ad hoc variants

Non-classical logic
- Computability logic 
- Many-valued logic
  - three-valued logic
  - infinitely-valued logics
    - fuzzy logic
- Intuitionistic (constructive) logic
- Linear logic
- Modal logic
- Paraconsistent logic
  - relevance logic
- Quantum logic
- non-monotonic logic
- Non-reflexive (Schrödinger) logic


Formal logic
- traditional Aristotelian syllogistic logic
- modern symbolic logic

- Intuitionistic logic
- Term (traditional, syllogistic, Aristotelian) logic
- Temporal logic
- Mathematical logic




In mathematics, predicate logic has been a great success, to the point of being often deemed as _the logic_: it is in principle sufficient for mathematics, has a sound and complete deduction system, and satisfies important semantic results. Lindstrom's theorems show that there can be no logical system with more expressive power than predicate logic and with the same good semantic properties.

By contrast, CS has seen a multitude of logics suited for different purposes.
- __Modal logic__ for reasoning about concepts like possibility or necessity.
- __Temporal logic__ is a branch of modal logic with operators to talk about the
passage of time.
- __Fuzzy logic__ deals with approximate, vague concepts such as the distinction between "warm" and "hot".
- __Probabilistic logic__ has the truth values of formulas as probabilities.
- __Nonmonotonic logic__: in this logic, an established piece of knowledge may have to be retracted if additional facts are later known.

Moreover, these logics come in different flavors, usually admitting propositional, first-order, higher-order, and intuitionistic presentations, as well as combinations of these and many ad hoc variants.

## Informal logic
Informal logic is the study of natural language arguments. The study of fallacies is an important branch of informal logic. Since much informal argument is not strictly speaking deductive, on some conceptions of logic, informal logic is not logic at all.

## Formal logic
Formal logic is the study of inference with purely formal content.

An inference possesses a purely formal content if it can be expressed as a particular application of a wholly abstract rule, that is, a rule that is not about any particular thing or property. The works of Aristotle contain the earliest known formal study of logic. Modern formal logic follows and expands on Aristotle.

In many definitions of logic, logical inference and inference with purely formal content are the same. This does not render the notion of informal logic vacuous, because no formal logic captures all of the nuances of natural language.

## Symbolic logic
Symbolic logic is the study of symbolic abstractions that capture the formal features of logical inference. It is often divided into two main branches:
- propositional logic
- predicate logic

## Mathematical logic
Mathematical logic is an extension of symbolic logic into other areas, in particular to the study of model theory, proof theory, set theory, and recursion theory.


## Logical systems

A formal system is an organization of terms used for the analysis of deduction. It consists of an alphabet, a language over the alphabet to construct sentences, and a rule for deriving sentences. Among the important properties that logical systems can have are:
- __Consistency__, which means that no theorem of the system contradicts another
- __Validity__, which means that the system's rules of proof never allow a false inference from true premises.
- __Completeness__, which means that if a formula is true, it can be proven, i.e. is a theorem of the system.
- __Soundness__, meaning that if any formula is a theorem of the system, it is true. This is the converse of completeness.

In a distinct philosophical use of the term, an argument is sound when it is both valid and its premises are true.

Some logical systems do not have all 4 properties. As an example, Kurt Gödel's incompleteness theorems show that sufficiently complex formal systems of arithmetic cannot be consistent and complete. However, first-order predicate logics, not extended by specific axioms to be arithmetic formal systems with equality, can be complete and consistent.



## Non-classical logic

https://www.wikiwand.com/en/Non-classical_logic

_Non-classical logics_ are _formal systems_ that significantly differ from _standard logical systems_ such as propositional and predicate logic.

There are several ways in which this is done, including by way of _extensions_, _deviations_, and _variations_. The aim of these departures is to make it possible to construct different models of _logical consequence_ and _logical truth_.





There are many kinds of non-classical logic including:
- __Computability logic__ is a semantically constructed formal theory of computability (as opposed to classical logic, which is a formal theory of truth) integrates and extends classical, linear and intuitionistic logics.
- __Many-valued logic__ rejects bivalence, allowing for truth values other than true and false. The most popular forms are _three-valued logic_, as initially developed by Jan Łukasiewicz, and _infinitely-valued logics_ such as _fuzzy logic_, which permits any real number between 0 and 1 as a truth value.
- __Intuitionistic logic__ rejects the law of the excluded middle, double negation elimination, and part of De Morgan's laws;
- __Linear logic__ rejects idempotency of entailment as well;
- __Modal logic__ extends classical logic with non-truth-functional,"modal", operators.
- __Paraconsistent logic__ (e.g. relevance logic) rejects the principle of explosion, and has a close relation to dialetheism;
- __Quantum logic__
- __Relevance logic__, _linear logic_, and __non-monotonic logic__ reject monotonicity of entailment;
- __Non-reflexive logic__ (also known as "_Schrödinger logics_") rejects or restricts the law of identity.


## Intuitionistic logic
https://www.wikiwand.com/en/Intuitionistic_logic

Intuitionistic logic, sometimes more generally called _constructive logic_, refers to systems of symbolic logic that differ from the systems used for classical logic by more closely mirroring the notion of constructive proof. In particular, systems of intuitionistic logic do not include the law of the excluded middle and double negation elimination, which are fundamental inference rules in classical logic.


## Term logic
Term logic, also known as traditional logic, syllogistic logic or Aristotelian logic, is a loose name for an approach to logic that began with Aristotle and that was dominant until the advent of modern predicate logic in the late XIX century.


## Temporal logic
https://www.wikiwand.com/en/Temporal_logic

In logic, temporal logic is any system of rules and symbolism for representing, and reasoning about, propositions qualified in terms of time. In a temporal logic we can then express statements like "I am always hungry", "I will eventually be hungry", or "I will be hungry until I eat something". Temporal logic is sometimes also used to refer to tense logic, a particular modal logic-based system of temporal logic introduced by Arthur Prior in the late 1950s, and important results were obtained by Hans Kamp. Subsequently it has been developed further by computer scientists, notably Amir Pnueli, and logicians.

Temporal logic has found an important application in formal verification, where it is used to state requirements of hardware or software systems. For instance, one may wish to say that whenever a request is made, access to a resource is eventually granted, but it is never granted to two requestors simultaneously. Such a statement can conveniently be expressed in a temporal logic.

## Algebraic logic
https://www.wikiwand.com/en/Algebraic_logic
In mathematical logic, algebraic logic is the reasoning obtained by manipulating equations with free variables.

What is now usually called classical algebraic logic focuses on the identification and algebraic description of models appropriate for the study of various logics (in the form of classes of algebras that constitute the algebraic semantics for these deductive systems) and connected problems like representation and duality. Well known results like the representation theorem for Boolean algebras and Stone duality fall under the umbrella of classical algebraic logic (Czelakowski 2003).

Works in the more recent abstract algebraic logic (AAL) focus on the process of algebraization itself, like classifying various forms of algebraizability using the Leibniz operator (Czelakowski 2003).

## Mathematical logic
https://www.wikiwand.com/en/Mathematical_logic
Mathematical logic is a subfield of mathematics exploring the applications of formal logic to mathematics. 

The unifying themes in mathematical logic include the study of the expressive power of formal systems and the deductive power of formal proof systems.

Mathematical logic is often divided into the fields of _set theory_, _model theory_, _computability (recursion) theory_, and _proof theory_ and _constructive mathematics_. These areas share basic results on logic, particularly first-order logic, and definability.



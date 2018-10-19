# Relevance logic


**Relevance logic** (also relevant logic) is a substructural logic that requires that antecedent and consequent of implications be relevantly related.


Relevance logic aims to capture aspects of implication that are ignored by the "material implication" operator in classical truth-functional logic, namely the notion of relevance between antecedent and conditional of a true implication.

This idea is not new: C. I. Lewis was led to invent modal logic, and specifically strict implication, on the grounds that classical logic grants paradoxes of material implication such as the principle that a falsehood implies any proposition.

Hence "if I'm a donkey, then two and two is four" is true when translated as a material implication, yet it seems intuitively false since a true implication must tie the antecedent and consequent together by some notion of relevance. And whether or not I'm a donkey seems in no way relevant to whether two and two is four.

How does relevance logic formally capture a notion of relevance? In terms of a syntactical constraint for a propositional calculus, it is necessary, but not sufficient, that premises and conclusion share atomic formulae (formulae that do not contain any logical connectives).

In a predicate calculus, relevance requires sharing of variables and constants between premises and conclusion. This can be ensured (along with stronger conditions) by, e.g., placing certain restrictions on the rules of a natural deduction system.

In particular, a Fitch-style natural deduction can be adapted to accommodate relevance by introducing tags at the end of each line of an application of an inference indicating the premises relevant to the conclusion of the inference.

Gentzen-style sequent calculi can be modified by removing the weakening rules that allow for the introduction of arbitrary formulae on the right or left side of the sequents.

A notable feature of relevance logics is that they are paraconsistent logics: the existence of a contradiction will not cause "explosion".

This follows from the fact that a conditional with a contradictory antecedent that does not share any propositional or predicate letters with the consequent cannot be true (or derivable).


---

https://en.wikipedia.org/wiki/Relevance_logic
http://plato.stanford.edu/entries/logic-relevance/


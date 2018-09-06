# Propositional logic



Propositional logic is also known as propositional calculus, sentential logic, zeroth-order logic.

Propositional logic is based on **propositions**, which are declarative sentences that have a __truth value__, which means they can be evaluated to true or false (i.e. true and false are truth values).

The argument is valid if the conclusion logically follows from the propositions - whether the propositions are actually true is not important, only that the conclusion logically follows from the propositions. We are only concerned with the logical structure of statements, not with their semantics, because we want to translate declarative sentences into formulas in order to make arguments rigorous. To this end, we also need to develop a (symbolic) language in which we can express sentences in a way that brings out their logical structure.

Translating declarative sentences into symbols allows us to concentrate on the argumentation alone. Similarly to converting declarative sentences from a natural language, we can convert a programming language, opening up the possibility for automatic program verification based on the rules of logic.

By identifying __atomic__ (indecomposable) declarative statements we can assign symbols, e.g. $$p,q,\dots$$, to each atom, allowing us to make more complex statements by composing the atoms and logical connectives according to the rules.

For example, using these atoms:
- $$p$$: "I won the lottery last week".
- $$q$$: "I've purchased a lottery ticket".
- $$r$$: "I won the last week's sweepstakes".

we can form additional, more complex, sentences according to the rules:
- __negation__: $$\neg$$ (NOT)    
The negation of $$p$$, denoted by $$\neg p$$, expresses "I did not win the lottery last week".
- __disjunction__: $$\lor$$ (OR)    
Given $$p$$ and $$r$$ we can state that at least one of them is true, denoted as $$p \lor r$$, as in "I won the lottery last week, or I won last week's sweepstakes".
- __conjunction__: $$\land$$ (AND)    
Given $$p$$ and $$r$$ we can state that both are true, denoted as $$p \land r$$, as in "Last week I won the lottery and the sweepstakes".
- __implication__: $$\rightarrow$$ (IF...THEN)   
Given $$p$$ and $$q$$ we state an implication between them as $$p \rightarrow q$$, as in "If I won the lottery last week, then I purchased a lottery ticket". Here, $$p$$ is _assumption_ and $$q$$ is _conclusion_.



We can use these rules of constructing propositions repeatedly. For example, we can form the proposition: $$p \land q \to \lnot r \lor q$$.

This shows we need binding priorities of these symbols: $$\lnot$$ binds more tightly than $$\lor$$ and $$\land$$, and the latter two bind more tightly than $$\to$$. Implication is right-associative: expressions of the form $$p \to q \to r$$ denote  $$p \to (q \to r)$$.

<details>
<summary>if...then</summary>

The natural language meaning of "if...then" often implicitly assumes a causal role of the assumption somehow enabling its conclusion. The logical meaning of implication is different: it states the preservation of truth which might happen without any causal relationship.

For example, "If all birds can fly, then Bob Dole was never president" is a true statement, but there is no known causal connection between propositions.

</details><br>


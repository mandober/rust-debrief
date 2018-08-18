# Propositional logic

_aka: {propositional, sentential, zeroth-order} x {calculus, logic}_

The aim of logic in computer science is to develop languages to model the situations we encounter as computer science professionals, in such a way that we can reason about them formally.

Reasoning about situations means constructing arguments about them; we want to do this formally, so that the arguments are valid and can be defended rigorously, or executed on a machine.


<details>
<summary>Example</summary>

If the train arrives late and there are no taxis at the station, then John is late for his meeting. John is not late for his meeting. The train did arrive late. Therefore, there were taxis at the station.

Intuitively, the argument is valid, since if we put the first sentence and the third sentence together, they tell us that if there are no taxis, then John will be late. The second sentence tells us that he was not late, so it must be the case that there were taxis.

The argument is valid if the conclusion logically follows from the propositions before it.

In developing logics, we are not concerned with what the sentences really mean, but only in their logical structure, so we can translate this example into:

If p and not q, then r. Not r. p. Therefore, q.

</details><br>



## Declarative sentences

In order to make arguments rigorous, we need to develop a language in which we can express sentences in such a way that brings out their logical structure.

Propositional logic is based on propositions, or declarative sentences which one can, in principle, argue as being true or false.

Using logic we can develop and use a formal language to model the programming constructs so that we can reason about them. Reasoning about various programming constructs means converting them to valid and provable arguments.

Propositional logic is a branch of logic based on __propositions__, which are declarative sentences that have a __truth value__: they are true or false; their thruthfulness can be a subject of discussion.

Translating a subset of declarative sentences into symbols allows us to concentrate on the argumentation. Similarly to converting a natural language, we can convert a subset of a programming language, opening up the possibility for automatic program verification based on the rules of logic.

By identifying __atomic__ (indecomposable) declarative statements we can assign symbols (e.g. $$p, q, r,\dots$$) to each atom, allowing us to make more complex statements by composition of atoms. Using these atoms and logical connectives, we can form additional, complex sentences according to the rules.

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


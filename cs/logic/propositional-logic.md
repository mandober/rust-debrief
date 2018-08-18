# Propositional logic

_aka: {propositional, sentential, zeroth-order} x {calculus, logic}_

Using logic we can develop and use a formal language to model the programming constructs so that we can reason about them. Reasoning about various programming constructs means converting them to valid and provable arguments.

Propositional logic is a branch of logic based on __propositions__, which are declarative sentences that have a __truth value__: they are true or false; their thruthfulness can be a subject of discussion.

Translating a subset of declarative sentences into symbols allows us to concentrate on the argumentation. Similarly to converting a natural language, we can convert a subset of a programming language, opening up the possibility for automatic program verification based on the rules of logic.

By identifying __atomic__ (indecomposable) declarative statements we can assign symbols (e.g. $$p, q, r,\dots$$) to each atom, allowing us to make more complex statements by composition of atoms. Using these atoms and logical connectives, we can form additional, complex sentences according to the syntax rules.


## Logical connectives

Logical connectives and their precedence:
- __negation__: $$\lnot$$ (NOT), the biggest precedence
- __conjunction__: $$\land$$ (AND), the same precedence as OR
- __disjunction__: $$\lor$$ (OR), , the same precedence as AND
- __implication__: $$\rightarrow$$ (IF...THEN)
- implication is right-associative: expressions of the form $$p \rightarrow q \rightarrow r$$ denote $$p \rightarrow (q \rightarrow r)$$
- parenthesis can be used to group the terms
- __bicondition__: $$\iff$$ (IFF) is a shorthand for $$(p\to q) \land (q\to p)$$ i.e. p if and only if q.

## Syntactic rules

Given a set $$A=\{p,q,r,\dots\}$$ of atomic propositions, the language of propositional logic is constructed according to the following rules:   

$$$\phi ::= p ∈ A |\ p\ | \lnot \phi | \phi \lor \phi | \phi \land \phi | \phi \to \phi | \phi \iff \phi$$$

We can now construct propositions such as: $$(p\land q) \rightarrow (\lnot r \lor q)$$, i.e. if $$p$$ and $$q$$ then not $$r$$ or $$q$$.


<details>
<summary>Example</summary>

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

</details>




## Proposition and conclusion

We can represent the sentence "If he won the lottery last week, then he purchased a lottery ticket." as $$p \rightarrow q$$, where $$p$$ is the _assumption_ and $$q$$ is the _conclusion_. Modern logic offers two approaches, that used to be merged together in the time of syllogisms, to determine whether the conclusion is valid consequence of a set of propositions, __model theory__ and __proof theory__.

## Model theory 
In model theory it is necessary to assign a meaning to the formulas, to define a semantics for the language. The central notion is that of truth and of deciding the circumstances under which a formula is true. The more complex the logic, the more difficult this assignment is and hence the more complex the semantics.

In propositional logic, we have to start by assigning arbitrary values to the atomic propositions: a __valuation__, $$V$$, is defined as a function that maps atomic propositions to either 0 (false) or 1 (true). The meaning $$\mathcal{I}^{V} (\phi)$$ of an arbitrary formula $$\phi$$ is defined recursively.

# Propositional logic

Using logic we can develop and use a formal language to model the programming constructs in such a manner
that they we can reason about them. Reasoning about various programming constructs means converting them to valid and provable arguments.

Propositional logic is based on propositions, which are declarative sentences that have a truth value: they are true or false; their thruthfulness can be a subject of discussion.

Translating a subset of (english) declarative sentences into symbols allows us to concentrate on the argumentation. Similarly to converting a natural language, we can convert a subset of a programming language (since it is also specified as a sequence of declarative statements), opening up the possibility for automatic program verification based on the rules of logic.

By identifying _atomic_ or _indecomposable_ declarative statements as we can assign symbols (e.g. $p, q, r,\dots$) to each atom, allowing us to make more complex statements by composition of atoms.

For example, using these atoms
- $p$: "I won the lottery last week".
- $q$: "I've purchased a lottery ticket".
- $r$: "I won the last week's sweepstakes".

we can form additional, more complex, sentences according to these rules:

* __negation__: $\neg$ (NOT)    
The negation of $p$, denoted by $\neg p$, expresses "I did not win the lottery last week".

* __disjunction__: $\vee$ (OR)    
Given $p$ and $r$ we can state that at least one of them is true, denoted as $p \vee r$, as in "I won the lottery last week, or I won last week's sweepstakes".

* __conjunction__: $\wedge$ (AND)    
Given $p$ and $r$ we can state that both are true, denoted as $p \wedge r$, as in "Last week I won the lottery and the sweepstakes".

* __implication__: $\rightarrow$ (IF...THEN)   
Given $p$ and $q$ we state an implication between them as $p \rightarrow q$, as in "If I won the lottery last week, then I purchased a lottery ticket". Here, $p$ is _assumption_ and $q$ is _conclusion_.

We can now construct propositions such as:    
$(p \wedge q) \rightarrow (\neg r \vee q)$ (if $p$ and $q$ then not $r$ or $q$)

$\neg$ binds more tightly than $\vee$ and $\wedge$, and the latter two bind more tightly than $\rightarrow$. Implication is right-associative: expressions of the form $p \rightarrow q \rightarrow r$ denote $p \rightarrow (q \rightarrow r)$.


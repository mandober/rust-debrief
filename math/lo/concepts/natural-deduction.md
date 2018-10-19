# Natural deduction

https://en.wikipedia.org/wiki/Natural_deduction
https://en.wikipedia.org/wiki/Deductive_reasoning
https://en.wikipedia.org/wiki/Category:Deductive_reasoning
https://ncatlab.org/nlab/show/natural+deduction


In logic and proof theory, natural deduction is a kind of proof calculus in which logical reasoning is expressed by inference rules closely related to the "natural" way of reasoning.


For constructing a calculus for reasoning about propositions so we can establish the validity we need a set of rules each of which allows us to draw a conclusion given a certain arrangement of premises.

Natural deduction has a collection of _proof rules_, which allow us to infer formulas from other formulas. By applying these rules in succession, we may infer a conclusion from a set of premises.

Given a set of _premises_, such as formulas $$\phi_1, \phi_2, \dots, \phi_n$$, and a _conclusion_ formula, $$\psi$$: we continually apply the proof rules to the premises, working our way through possibly more formulas, until we eventually arrive at the conclusion, which is denoted by $$\phi_1, \phi_2, \dots, \phi_n \vdash \psi$$.

This expression is called a __sequent__; it is valid if its proof exists. (the turnstile symbol, $$\vdash$$, is usually read "yields" or "proves").


<details>
<summary>Example</summary

Example 1.   
If the train arrives late and there are no taxis at the station, then John is late for his meeting. John is not late for his meeting. The train did arrive late. Therefore, there were taxis at the station.

Example 2.    
If it is raining and Jane does not have her umbrella with her, then she will get wet. Jane is not wet. It is raining. Therefore, Jane has her umbrella with her.

```
If it's raining and Jane doesn't have her umbrella, then she'll get wet.
It's raining.
Jane is not wet.
Therefore, Jane has her umbrella.
```

which could be simplified into:

```
If p and not q, then r.
Not r.
p.
Therefore, q.
```

and converted into formal presentation:

$${p}\wedge\neg{q}\rightarrow{r}$$
$$\neg{r}$$
$${p}$$
$$\therefore{q}$$

The sequent for these examples:    
$${p}\wedge\neg{q}\rightarrow{r}, \neg{r}, p \vdash q$$

</details><br>


It is not necessarily obvious which rules to apply, and in what order, to obtain the desired conclusion. Additionally, our proof rules should be carefully chosen; otherwise, we might be able to "prove" invalid patterns of argumentation. For example, we won't be able to prove the sequent: $$p, q \vdash p \land \lnot q$$.

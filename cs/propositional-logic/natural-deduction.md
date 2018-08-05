# Natural deduction

Natural deduction has a collection of _proof rules_, which allow us to _infer_ formulas from other formulas. By applying these rules in succession, we may infer a conclusion from a set of premises.

Given a set of premises, such as formulas $\phi_1, \phi_2, \dots, \phi_n$, and a conclusion formula, $\psi$: we continually apply the proof rules to the premises, working our way through possibly more formulas, until we eventually arrive at the conclusion, which is denoted as: $\phi_1, \phi_2, \dots, \phi_n \vdash \psi$. This expression is called a __sequent__; it is valid if its proof exists. (The symbol $\vdash$ is usually read "entails)".

> Example 1.   
> If the train arrives late and there are no taxis at the station, then John is late for his meeting. John is not late for his meeting. The train did arrive late. Therefore, there were taxis at the station.

> Example 2.    
> If it is raining and Jane does not have her umbrella with her, then she will get wet. Jane is not wet. It is raining. Therefore, Jane has her umbrella with her.

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

${p}\wedge\neg{q}\rightarrow{r}$
$\neg{r}$
${p}$
$\therefore{q}$

The sequent for these examples: ${p}\wedge\neg{q}\rightarrow{r}, \neg{r}, p \vdash q$

It is not necessarily obvious which rules to apply, and in what order, to obtain the desired conclusion. Additionally, our proof rules should be carefully chosen; otherwise, we might be able to "prove" invalid patterns of argumentation. 

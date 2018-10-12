# Rules of Inference for Natural Deduction

Rules for natural deduction are about introduction and elimination of logical connectives: negation, conjunction, disjunction, implication and  bijunction.


For each connective, there are rules to introduce it and eliminate it
- Negation
  - negation introduction
  - negation elimination
  - double negation introduction
  - double negation elimination
- Conjunction
  - and-introduction
  - and-elimination (conjunction elimination, simplification)
- Disjunction
  - disjunction introduction
  - disjunction elimination
- Implication
  - implication introduction
  - implication elimination (modus ponens)
  - conditional proof (conditional introduction) 
  - modus tollens
- Biconditional
  - biconditional introduction
  - biconditional elimination



## Conjunction

* $$\land$$-introduction, $$\land_{i}$$
* $$\land$$-elimination (one for each atom)
  - $$\land_{e_1}$$
  - $$\land_{e_2}$$


Conjunction introduction (1 rule)

$$\land$$-introduction (and-introduction, conjunction introduction, adjunction): $$\land_{i}$$

It allows us to conclude $$\phi \land \psi$$, given that we have already concluded $$\phi$$ and $$\psi$$ separately.

$$$
\frac
{\phi \quad \psi}
{\phi \land \psi}
{\, \land_i}
$$$

Above the line are the two _premises_ of the rule, and below is the conclusion. We have introduced a conjunction (in the conclusion) where there was none before (in the premises).


Conjunction elimination (2 rules)

$$\land$$-elimination has two rules, one for each assumption: $$\wedge_{e_1}$$ and $$\wedge_{e_2}$$

The rule $$\land_{e}$$ says that if you have a proof of $$\phi \land \psi$$ then
- by applying the rule $$\land_{e_1}$$, you can get a proof of $$\phi$$, or
- by applying the rule $$\land_{e_2}$$, you can get a proof of $$\psi$$.

$$$
\frac
{\phi \land \psi}
{\phi \quad}
{\, \land_{e_1}}
$$$

Applying these rules is a kind of pattern matching:
- in the first rule, the conclusion $$\phi$$ has to match the first __conjunct__ of the premise, the second conjunct is irrelevant.
- In the second rule, the conclusion $$\psi$$ has to match the second conjunct, the first conjucnt can be any formula.

$$$
\frac
{\phi \land \psi}
{\quad \psi}
{\, \land_{e_2}}
$$$


<details>
<summary>Example</summary>

Using this rules to prove that $$p\land q, r \vdash q\land r$$ is valid.

$$$
\begin{array}{lll}
1 \quad   & p\land q   & premise           \\
2 \quad   & r          & premise           \\
3 \quad   & q          & \land_{e_2} \ 1   \\
4 \quad   & q\land r   & \land_{i} \ \ 3,2 \\
\end{array}
$$$

</details><br>



## Negation

- These rules involve the notion of contradiction.
- Contradictions are expressions of the form $$\lnot \phi \land \phi$$, $$\phi \land \lnot \phi$$, where $$\phi$$ is any formula.

Contradictions can be derived from contradictions, in fact, any formula can be derived from a contradiction, making this argument valid: $$p\land \lnot p \vdash q$$.

The reason natural deduction takes this stance is that $$\vdash$$ tells us all the things we may infer, provided that we can assume the formulas to the left of it. This process does not care whether such premises make any sense. This has at least the advantage that we can match $$\vdash$$ to checks based on semantic intuitions: if all the premises compute to true, then the conclusion must compute true as well. In particular, this is not a constraint in the case that one of the premises is (always) false.

The fact that $$\bot$$ can prove anything is encoded in the calculus by the proof rule "bottom-elimination":

$$$
\frac
{\bot}
{\phi}
{\, \bot_{e}}
$$$


The fact that $$\bot$$ itself represents a contradiction is encoded by the proof rule "not-elimination":

$$$
\frac
{\phi \quad \lnot \phi}
{\bot}
{\, \lnot_{e}}
$$$


We can show that $$\lnot p \lor q \vdash p \to q$$ is valid.



## Double negation

Double negation introduction: $$\neg\neg{i}$$

$$$
\frac
{\phi}
{\lnot \lnot \phi}
{\, \lnot \lnot_{i}}
$$$

Double negation elimination: $$\neg\neg{e}$$

$$$
\frac
{\lnot \lnot \phi}
{\quad \phi}
{\, \lnot \lnot_{e}}
$$$




## Implication

There is one rule to introduce implication and one to eliminate it.


The rules for implication elimination

There are 2 rules to eliminate implication: modus ponens and modus tollens.

Modus ponens
- _modus ponens_ (MP), _modus ponendo ponens_, arrow-elimination, implies-elimination: $$\to_{e}$$
- MP states that, given $$\phi$$ and knowing that $$\phi\to \psi$$, we may rightfully conclude $$\psi$$
- $$\phi \to \psi,\phi \vdash \psi$$

$$$
\frac
{\phi \quad \phi \to \psi}
{\quad \psi}
{\, \to_{e}}
$$$


Modus tollens
- _modus tollens_ (MT), _modus tolendo tollens_
- $$\phi \to \psi, \neg \psi \vdash \neg \phi$$

$$$
\frac
{\phi \to \psi \quad \lnot \psi}
{\quad \lnot \phi}
{\, _{MT}}
$$$


<details>
<summary>Examples</summary>

Prove that $$p\to (q\to r), p, \lnot r \vdash \lnot q$$ holds:

$$$
\begin{array}{lll}
1 \quad   & p\to (q\to r)  & premise       \\
2 \quad   & p              & premise       \\
3 \quad   & \lnot r        & premise       \\
4 \quad   & q\to r         & \to_{e} \ 1,2 \\
5 \quad   & \lnot q        & MT      \ 4,3 \\
\end{array}
$$$


Prove that $$\lnot p\to q, \lnot q \vdash p$$ holds:

$$$
\begin{array}{lll}
1 \quad   & \lnot p\to q   & premise       \\
2 \quad   & \lnot q        & premise       \\
3 \quad   & \lnot \lnot p  & MT              \ 1,2 \\
4 \quad   & p              & \lnot{\lnot_{e}} \ 3 \\
\end{array}
$$$


Prove that $$\lnot p\to \lnot q, q \vdash \lnot p$$ holds:

$$$
\begin{array}{lll}
1 \quad   & p\to \lnot q   & premise       \\
2 \quad   & q              & premise       \\
4 \quad   & \lnot \lnot q  & \lnot{\lnot_{i}} \ 2 \\
3 \quad   & \lnot p        & MT               \ 1,3 \\
\end{array}
$$$

Note that the order of applying double negation rules and MT is different in these examples; this order is driven by the structure of the particular sequent whose validity one is trying to show.

</details><br>



The rule for implication introduction

MT makes it possible to show that $$p \to q, \lnot q \vdash \lnot p$$ is valid, so it seems the validity of the sequent $$p \to q \vdash \lnot q \to \lnot p$$ should also be valid. But to show it holds we have to make temporary assumtions. If we assume that $$\lnot q$$ holds, we can use $$MT$$ to infer $$\lnot p$$. So, assuming $$p \to q$$ we can show that $$\lnot q \to \lnot p$$.

$$$
\begin{array}{lll}
1 \quad   & p\to q               & premise       \\
\hline
2 \quad   & \lnot q              & assumption    \\
3 \quad   & \lnot p              & MT      \ 1,2 \\
\hline
4 \quad   & \lnot q \to \lnot p  & \to_i   \ 2-3 \\
\end{array}
$$$



## Disjunction

The rules for disjunction introduction

From the premise $$\phi$$ we can infer that $$\phi \lor \psi$$ holds, for we already know that $$\phi$$ holds; this inference is valid for any choice of $$\psi$$.

$$$
\frac
{\phi}
{\phi \lor \psi}
{\, \lor_{i_1}}
$$$

Similarly, we may conclude $$\phi \lor \psi$$ if we already have $$\psi$$, which is valid for any choice of $$\phi$$.

$$$
\frac
{\psi}
{\phi \lor \psi}
{\, \lor_{i_2}}
$$$


The rules for disjunction elimination

To use a formula $$\phi \lor \psi$$ in a proof, we need to disassemble assumptions into their basic constituents so that the latter may be used in our argumentation such that they render our desired conclusion.

Let us imagine that we want to show some proposition $$\chi$$ by assuming $$\phi \lor \psi$$. Since we don't know which of $$\phi$$ and $$\psi$$ is true, we have to give two separate proofs which we need to combine into one argument:
- First, we assume $$\phi$$ is true and have to come up with a proof of $$\chi$$.
- Next, we assume $$\psi$$ is true and need to give a proof of $$\chi$$ as well.
- Given these two proofs, we can infer $$\chi$$ from the truth of $$\phi \lor \psi$$, since our case analysis above is exhaustive.



## The Kreisel–Putnam rule

https://en.wikipedia.org/wiki/Admissible_rule

The Kreisel–Putnam rule or Independence of Premise

$$
\underline{\quad\quad\neg p\to q\lor r\quad\quad}\\
{(\neg p\to q) \lor (\neg p\to r)}
\quad (\mathit{KPR})
$$

is admissible in the Intuitionistic Propositional Calculus (IPC).   
In fact, it is admissible in every superintuitionistic logic.

On the other hand, the formula:    
$$(\neg p\to q\lor r) \to ((\neg p\to q) \lor (\neg p\to r))$$

is not an intuitionistic tautology, hence KPR is not derivable in IPC.   
In particular, IPC is not structurally complete.


## Defeasibility
it refers to the possibility of a particular principle, rule or understanding being overridden in appropriate circumstances.

In modern argumentation theories, arguments are regarded as defeasible passages from premises to a conclusion. Defeasibility means that when additional information (new evidence or contrary arguments) is provided, the premises may be no longer lead to the conclusion (non-monotonic reasoning). This type of reasoning is referred to as defeasible reasoning. For instance we consider the famous Tweedy example:


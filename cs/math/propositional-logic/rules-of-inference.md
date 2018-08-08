# Rules of inference

Connectives introduction and elimination:
- negation
- conjunction
- disjunction
- implication
- bijunction


- Negation
  - negation introduction
  - negation elimination
  - double negation introduction
  - double negation elimination
- Conjunction
  - and-introduction (conjunction introduction, adjunction)
  - and-elimination (conjunction elimination, simplification)
- Disjunction
  - disjunction introduction
  - disjunction elimination
- Implication
  - implication introduction
  - implication elimination (modus ponens, implies-elimination, conditional elimination) 
  - conditional proof (conditional introduction) 
  - modus tollens
- Biconditional
  - biconditional introduction
  - biconditional elimination



## Negation

negation introduction (Reductio ad absurdum)


$$
{\phi \vdash \psi }\\
{\underline {\phi \vdash \lnot \psi}}\\
{\lnot \phi}
$$


Reductio ad absurdum (related to the law of excluded middle)


Noncontradiction (or Negation Elimination)

Double negation elimination

Double negation introduction



## Conjunction


and-introduction, conjunction introduction, adjunction, $\wedge{i}$

and-elimination

two rules, one for each assumption: $\wedge{e_1}$ and $\wedge{e_2}$


The rules of double negation:
- double negation introduction: $\neg\neg{i}$
- double negation elimination: $\neg\neg{e}$

The rules for implication:
- implication introduction: $\rightarrow{i}$
- implication elimination: $\rightarrow{e}$    
  (_modus ponens_,  implies-elimination, arrow-elimination)
- _modus tollens_: $MT$
$\phi \rightarrow \psi, \neg \psi \vdash \neg \phi$


---

https://www.wikiwand.com/en/Propositional_calculus


# Rules of propositional calculus


Negation introduction
From (p\to q) and (p\to \neg q), infer \neg p.
That is, \{(p\to q),(p\to \neg q)\}\vdash \neg p.

Negation elimination
From \neg p, infer (p\to r).
That is, \{\neg p\}\vdash (p\to r).

Double negative elimination
From \neg \neg p, infer p.
That is, \neg \neg p\vdash p.

Conjunction introduction
From p and q, infer (p\land q).
That is, \{p,q\}\vdash (p\land q).

Conjunction elimination
From (p\land q), infer p.
From (p\land q), infer q.
That is, (p\land q)\vdash p and (p\land q)\vdash q.

Disjunction introduction
From p, infer (p\lor q).
From q, infer (p\lor q).
That is, p\vdash (p\lor q) and q\vdash (p\lor q).

Disjunction elimination
From (p\lor q) and (p\to r) and (q\to r), infer r.
That is, \{p\lor q,p\to r,q\to r\}\vdash r.

Biconditional introduction
From (p\to q) and (q\to p), infer (p\leftrightarrow q).
That is, \{p\to q,q\to p\}\vdash (p\leftrightarrow q).

Biconditional elimination
From (p\leftrightarrow q), infer (p\to q).
From (p\leftrightarrow q), infer (q\to p).
That is, (p\leftrightarrow q)\vdash (p\to q) and (p\leftrightarrow q)\vdash (q\to p).

Modus ponens (conditional elimination) 
From p and (p\to q), infer q.
That is, \{p,p\to q\}\vdash q.

Conditional proof (conditional introduction) 
From [accepting p allows a proof of q], infer (p\to q).
That is, (p\vdash q)\vdash (p\to q).


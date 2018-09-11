# Rules of inference


## Negation

negation introduction (Reductio ad absurdum)

$$$
{\phi \vdash \psi }\\
{\underline {\phi \vdash \lnot \psi}}\\
{\lnot \phi}
$$$

Reductio ad absurdum (related to the law of excluded middle)
Noncontradiction (or Negation Elimination)
Double negation elimination
Double negation introduction


## Conjunction
and-introduction, conjunction introduction, adjunction, $$\wedge{i}$$
and-elimination
two rules, one for each assumption: $$\wedge{e_1}$$ and $$\wedge{e_2}$$


##  The rules of double negation:
- double negation introduction: $$\neg\neg{i}$$
- double negation elimination: $$\neg\neg{e}$$


##  The rules for implication:
- implication introduction: $$\rightarrow{i}$$
- implication elimination: $$\rightarrow{e}$$    
  (_modus ponens_,  implies-elimination, arrow-elimination)
- _modus tollens_: $$MT$$
$$\phi \rightarrow \psi, \neg \psi \vdash \neg \phi$$


## Implication
- implication introduction
- implication elimination (modus ponens, implies-elimination, conditional elimination) 
- conditional proof (conditional introduction) 
- modus tollens

### Modus ponens (MP)
Modus ponens (modus ponendo ponens i.e. "mode that affirms by affirming", or implication elimination, or $$\rightarrow$$ elimination) is a rule of inference in propositional logic that can be summarized as: "if p then q; there is a p, therefore there is a q".

$$$
p\to{q}\\
\underline{p\quad\quad}\\
\therefore{q\quad}
$$$

As a sequent: $$p\to{q}, \; p\;\; \vdash\;\; q$$
where $$\vdash$$ is a metalogical symbol meaning that q is a syntactic consequence of $$p\to{q}$$ and $$p$$ in some logical system.

As the statement of a truth-functional tautology or theorem of propositional logic: $$((p \to q) \land p) \to q$$, where $$p$$, and $$q$$ are propositions expressed in some formal system.


## Rules of propositional calculus
https://www.wikiwand.com/en/Propositional_calculus


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


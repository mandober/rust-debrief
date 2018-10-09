# Rules of inference

<!-- TOC -->

- [Negation](#negation)
- [Conjunction](#conjunction)
- [double negation](#double-negation)
- [implication](#implication)
- [Modus ponens](#modus-ponens)
- [De Morgan's laws](#de-morgans-laws)
- [Rules of propositional calculus](#rules-of-propositional-calculus)

<!-- /TOC -->


## Negation

Negation introduction (Reductio ad absurdum)

$$
{\phi \vdash \psi }\\
{\underline {\phi \vdash \lnot \psi}}\\
{\lnot \phi}
$$

$$p\to q, p\to\neg q \vdash\neg p$$


## Conjunction

- and-introduction
- $$\land_{i}$$
- adjunction
- conjunction introduction
- and-elimination

two rules, one for each assumption: $$\wedge{e_1}$$ and $$\wedge{e_2}$$


## double negation
- double negation introduction: $$\neg\neg{i}$$
- double negation elimination: $$\neg\neg{e}$$


## implication
- implication introduction: $$\rightarrow{i}$$
- implication elimination: $$\rightarrow{e}$$    
  (_modus ponens_,  implies-elimination, arrow-elimination)
- _modus tollens_: $$MT$$
$$\phi \rightarrow \psi, \neg \psi \vdash \neg \phi$$


- implication introduction
- implication elimination (modus ponens, implies-elimination, conditional elimination) 
- conditional proof (conditional introduction) 
- modus tollens

## Modus ponens
Modus ponens (modus ponendo ponens i.e. "mode that affirms by affirming", or implication elimination, or $$\rightarrow$$ elimination) is a rule of inference in propositional logic that can be summarized as: "if p then q; there is a p, therefore there is a q".

$$$
p\to{q}\\
\underline{p\quad\quad}\\
\therefore{q\quad}
$$$

As a sequent: $$p\to{q}, \; p\;\; \vdash\;\; q$$
where $$\vdash$$ is a metalogical symbol meaning that q is a syntactic consequence of $$p\to{q}$$ and $$p$$ in some logical system.

As the statement of a truth-functional tautology or theorem of propositional logic: $$((p \to q) \land p) \to q$$, where $$p$$, and $$q$$ are propositions expressed in some formal system.


## De Morgan's laws
The negation of conjunction rule may be written in sequent notation:

$$\neg (P\land Q) \vdash (\neg P \lor \neg Q)$$

The negation of disjunction rule may be written as:

$$\neg (P\lor Q)\vdash (\neg P\land \neg Q)$$


De Morgan's duality can be generalised to quantifiers, the universal quantifier and existential quantifier are duals:

$$\forall x P(x) \equiv \neg (\exists x\,\neg P(x))$$

$$\exists x P(x)\equiv \neg (\forall x\,\neg P(x))$$



## Rules of propositional calculus

https://en.wikipedia.org/wiki/Propositional_calculus


**Negation introduction**

$$p\to q, p\to\neg q \vdash\neg p$$


**Negation elimination**

$$\neg p \vdash p\to r$$

**Double negative elimination**

$$\neg \neg p\vdash p$$

**Conjunction introduction**

$$p,q \vdash p\land q$$

**Conjunction elimination**

$$p\land q \vdash p$$ and $$p\land q \vdash q$$

**Disjunction introduction**

$$p\vdash p\lor q$$ and $$q\vdash p\lor q$$

**Disjunction elimination**

$$p\lor q,p\to r,q\to r \vdash r$$


**Biconditional introduction**

From $$p\to q$$ and $$q\to p$$ infer $$p\leftrightarrow q$$

$$p\to q, q\to p \vdash p\leftrightarrow q$$


**Biconditional elimination**

$$p\leftrightarrow q \vdash (p\to q)$$ 

and

$$(p\leftrightarrow q)\vdash (q\to p)$$


**Modus ponens** (conditional elimination) 
From p and (p\to q), infer q.

$$p, p\to q \vdash q$$


**Conditional proof** (conditional introduction) 

From, assumption that $$p$$ allows a proof of $$q$$, infer $$p\to q$$

$$(p\vdash q)\vdash (p\to q)$$


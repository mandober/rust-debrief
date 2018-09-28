# Propositional Logic


**Propositional logic** (propositional calculus, sentential logic, zeroth-order logic, statement logic) is based on **propositions**, which are declarative sentences that have a **truth value** - they can be evaluated to either true or false, not both, not neither.

Compound propositions are formed by connecting propositions by **logical connectives**.

The propositions without logical connectives are called **atomic propositions**.


The argument is **valid** if the conclusion logically follows from the propositions - whether the propositions are actually true is not important, only the form.

We are only concerned with the logical structure of statements, not with their semantics for we want to transform declarative sentences into formulas in order to make arguments rigorous.

A symbolic language is employed to express sentences in a way that brings out their logical structure in order to showcase the argumentation.

Similarly to transforming a subset of a natural language, we can transform a subset of a programming language, opening up the possibility for automatic program verification based on the rules of logic.

By identifying atomic (indecomposable) declarative statements we can assign symbols, e.g. $$p,q,r$$, to each atom, allowing us to make additional, more complex statements by composing the atoms with logical connectives according to the rules.



## Syntactic rules

**Definition of propositional logic**    
1. Every atomic formula φ is a formula of sentential logic.
2. If φ is a formula of sentential logic, then so is ¬φ.
3. If φ and ψ are formulae of sentential logic, 
   then so are each of the following: (φ & ψ), (φ ∨ ψ), (φ → ψ)
4. An expression φ of sentential logic is a formula only if it can
   be constructed by one or more applications of the first 3 rules.

This is an inductive definition, giving rules for generating or constructing the objects falling under the definition.


Given a set $$A=\{p,q,r,\dots\}$$ of atomic propositions, the language of propositional logic is constructed according to the following rules:   

$$\phi ::= p \in A |\ p\ | \lnot \phi\ |\ \phi \lor \phi\ |\ \phi \land \phi\ |\ \phi \to \phi\ |\ \phi \iff \phi$$

We can now construct propositions such as: $$(p\land q) \to (\lnot r \lor q)$$ (if $$p$$ and $$q$$ then not $$r$$ or $$q$$).


## Syntax and Formation Rules of PL

A statement letter of PL is defined as any uppercase letter written with or without a numerical subscript.

Note: According to this definition, 'A', 'B', 'B2', 'C3', and 'P14' are examples of statement letters. The numerical subscripts are used just in case we need to deal with more than 26 simple statements: in that case, we can use 'P1' to mean something different than 'P2', and so forth.

Definition:
A connective or operator of PL is any of the signs '¬', '∧', 'v', '→', and '↔'.

Definition:
A well-formed formula (hereafter abbrevated as wff) of PL is defined recursively as follows:

Any statement letter is a well-formed formula.
If α is a well-formed formula, then so is '¬α'.
If α and β are well-formed formulas, then so is '(α ∧ β)'.
If α and β are well-formed formulas, then so is '(α v β)'.
If α and β are well-formed formulas, then so is '(α → β)'.
If α and β are well-formed formulas, then so is '(α ↔ β)'.
Nothing that cannot be constructed by successive steps of (1)-(6) is a well-formed formula.

Note: According to part (1) of this definition, the statement letters 'C', 'P' and 'M' are wffs. Because 'C' and 'P' are wffs, by part (3), "(C ∧ P)" is a wff. Because it is a wff, and 'M' is also a wff, by part (6), "(M ↔ (C ∧ P))" is a wff. It is conventional to regard the outermost parentheses on a wff as optional, so that "M ↔ (C ∧ P)" is treated as an abbreviated form of "(M ↔ (C ∧ P))". However, whenever a shorter wff is used in constructing a more complicated wff, the parentheses on the shorter wff are necessary.

The notion of a well-formed formula should be understood as corresponding to the notion of a grammatically correct or properly constructed statement of language PL. This definition tells us, for example, that "¬(Q v ¬R)" is grammatical for PL because it is a well-formed formula, whereas the string of symbols, ")¬Q¬v(↔P∧", while consisting entirely of symbols used in PL, is not grammatical because it is not well-formed.



## Truth Functions and Truth Tables

So far we have in effect described the grammar of language PL. When setting up a language fully, however, it is necessary not only to establish rules of grammar, but also describe the meanings of the symbols used in the language. We have already suggested that uppercase letters are used as complete simple statements. Because truth-functional propositional logic does not analyze the parts of simple statements, and only considers those ways of combining them to form more complicated statements that make the truth or falsity of the whole dependent entirely on the truth or falsity of the parts, in effect, it does not matter what meaning we assign to the individual statement letters like 'P', 'Q' and 'R', etc., provided that each is taken as either true or false (and not both).


XOR: ¬(α ↔ β) ≡ (α ∧ ¬β) v (¬α ∧ β)

```
         ¬(α ↔ β)
¬( ( α →  β) ∧  (β   →  α)  )
¬( (¬α v  β) ∧  (¬β  v  α) )
  ¬(¬α v  β) v ¬(¬β  v  α)
  (¬¬α ∧ ¬β) v  (¬¬β ∧ ¬α)
    (α ∧ ¬β) v  (¬α ∧ β)
```

Also some of the usual operators can as well be defined in terms of the others.

It is possible to get by using only the signs '¬' and anothor one among '→', '∧', 'v' and define all other possible truth-functions in virtue of them.


## In terms of '¬' and '→'
```
α ∧ β ≡ ¬(α → ¬β)
α v β ≡ ¬α → β
α ↔ β ≡ ¬((α → β) → ¬(β → α))
```

## In terms of '¬' and 'v'
```
α ∧ β ≡ ¬(¬α v ¬β)
α → β ≡ (¬α v β)
α ↔ β ≡ ¬(¬(¬α v β) v ¬(¬β v α))
```

## In terms of '¬' and '∧'
```
α v β ≡ ¬(¬α ∧ ¬β)
α → β ≡ ¬(α ∧ ¬β)
α ↔ β ≡ (¬(α ∧ ¬β) ∧ ¬(β ∧ ¬α)
```


## In terms of a single operator

There are also 2 ways of reducing all truth-functional operators down to a single primitive operator, but they require using an operator that is not included so far.

### Sheffer's stroke
In first approach, we utilize an operator NAND (not AND) or Sheffer's stroke, written '↑', cooresponding to '¬(α ∧ β)'

Sheffer's stroke:
```
α ↑ β ≡ ¬(α ∧ β)
```

Defining all operators in term of Sheffer's stroke:

```
α ∧ β ≡ (α↑β) ↑ (α↑β)
α v β ≡ (α↑α) ↑ (β↑β)
α → β ≡ α ↑ (β↑β)
α ↔ β ≡ ((α↑α) ↑ (β↑β)) ↑ (α↑β)
   ¬α ≡ α↑α
```

### Peirce's arrow
In second approach, we utilize an operator NOR (not OR), written '↓', cooresponding to '¬(α v β)'. The sign '↓' is called the Peirce's arrow.

Peirce's arrow:
```
α ↓ β ≡ ¬(α ∧ β)
```

Defining all operators in term of Peirce's arrow:

```
¬α ≡ α↓α
α v β ≡ (α↓β)   ↓ (α↓β)
α ∧ β ≡ (α↓α)   ↓ (β↓β)
α → β ≡ (α↓α)↓β ↓ (α↓α)↓β
```


Parallel review:
```
op    ≡ Sheffer stroke          ≡ Peirce arrow
   ¬α ≡   α↑α                   ≡  α↓α
α ∧ β ≡  (α↑β) ↑ (α↑β)          ≡ (α↓α)   ↓ (β↓β)
α v β ≡  (α↑α) ↑ (β↑β)          ≡ (α↓β)   ↓ (α↓β)
α → β ≡     α  ↑ (β↑β)          ≡ (α↓α)↓β ↓ (α↓α)↓β
α ↔ β ≡ ((α↓α) ↓ (β↓β)) ↓ (α↓β) ≡
```


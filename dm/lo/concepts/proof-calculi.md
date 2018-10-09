# Proof calculi

https://en.wikipedia.org/wiki/Proof_calculus


A __proof calculus__ or a __proof system__ is built to prove statements.

A proof system encompasses:
- **Language**: the set of formulas admitted by the proof system.
- **Rules of inference**: list of rules to prove theorems from axioms.
- **Axioms**: formulas in the language assumed to be valid.

Usually a given proof calculus encompasses more than a single particular formal system, since many proof calculi are under-determined and can be used for radically different logics.

For example, a paradigmatic case is the sequent calculus, which can be used to express the consequence relations of both intuitionistic logic and relevance logic. Thus, loosely speaking, a proof calculus is a template or design pattern, characterized by a certain style of formal inference, that may be specialized to produce specific formal systems, namely by specifying the actual inference rules for such a system. There is no consensus among logicians on how best to define the term.


## Examples of proof calculi
The most widely known proof calculi are those classical calculi that are still in widespread use:
- The class of Hilbert systems, of which the most famous example is the 1928 Hilbert-Ackermann system of first-order logic;
- Gerhard Gentzen's calculus of natural deduction, which is the first formalism of structural proof theory, and which is the cornerstone of the formulae-as-types correspondence relating logic to functional programming;
- Gentzen's sequent calculus, which is the most studied formalism of structural proof theory.

Many other proof calculi were, or might have been, seminal, but are not widely used today.
- Aristotle's syllogistic calculus, presented in the Organon, readily admits formalisation. There is still some modern interest in syllogistic, carried out under the aegis of term logic.
- Gottlob Frege's two-dimensional notation of the Begriffsschrift (1879) is usually regarded as introducing the modern concept of quantifier to logic.
- C.S. Peirce's existential graph might easily have been seminal, had history worked out differently.

Modern research in logic teems with rival proof calculi:
- Several systems have been proposed which replace the usual textual syntax with some graphical syntax. Proof nets and cirquent calculus are among such systems.
- Recently, many logicians interested in structural proof theory have proposed calculi with deep inference, for instance display logic, hypersequents, the calculus of structures, and bunched implication.


## Natural deduction
In logic and proof theory, **natural deduction** is a kind of **proof calculus** in which logical reasoning is expressed by inference rules closely related to the "natural" way of reasoning. This contrasts with **Hilbert-style systems**, which instead use axioms as much as possible to express the logical laws of deductive reasoning.


## Sequent calculus
Sequent calculus is one of several extant styles of _proof calculus_ for expressing line-by-line logical arguments.

Proof calculi:
* __Hilbert style__: every line is an unconditional tautology (or theorem).
* __Gentzen style__: every line is a conditional tautology (or theorem) with zero or more conditions on the left.
* __Natural deduction__: every conditional line has exactly one asserted proposition on the right.
* __Sequent calculus__: every conditional line has zero or more asserted propositions on the right.

_Natural deduction_ and _sequent calculus_ systems are particular distinct kinds of Gentzen-style systems. _Gentzen-style_ systems typically have very few axioms, if any, relying more on the inference rules. _Hilbert-style_ systems typically have a very small number of inference rules, relying more on the sets of axioms.

_Gentzen-style_ systems have significant practical and theoretical advantages compared to Hilbert-style systems.


For example, both natural deduction and sequent calculus systems facilitate the elimination and introduction of universal and existential quantifiers so that unquantified logical expressions can be manipulated according to the much simpler rules of propositional calculus.

In a typical argument, 
- quantifiers are eliminated, 
- then propositional calculus is applied to unquantified expressions (which typically contain free variables), 
- and then the quantifiers are reintroduced, much like the way in which mathematical proofs are carried out in practice by mathematicians.

Predicate calculus proofs are generally much easier to discover with this approach, and are often shorter.
- Natural deduction systems are more suited to practical theorem-proving.
- Sequent calculus systems are more suited to theoretical analysis.

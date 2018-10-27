# Lectures in Logic and Set Theory

Lectures in Logic and Set Theory Volume 1: Mathematical Logic
Lectures in Logic and Set Theory Volume 2: Set Theory


## Mathematical Logic
Logic is the science of reasoning.

Mathematical logic applies to mathematical reasoning - deduction.

The modern use of the term mathematical logic includes the proof theory, study of the structure, properties and limitations of proofs; model theory, study of the interplay between syntax and semantics, by looking at the algebraic structures where formal languages are interpreted; computability theory, study of the properties and limitations of algorithmic processes; set theory.

By formalization we understand the faithful representation or simulation of the reasoning processes of mathematics in general (pure logic), or of a particular mathematical theory (applied logic), within an activity that is, in principle, driven exclusively by the form or syntax of mathematical statements, ignoring their meaning.

Formalization turns mathematical theories into mathematical objects that we can study. This is analogous to building a model airplane with intention of studying through it the properties of a real airplane. One can also use the formal theory to generate theorems, i.e. discover truths in the real domain by simply running the simulation that this theory replica is.


## First Order Languages

A formalized **mathematical theory** consists of:
- a set of primitive symbols, V, used to build symbol sequences (also called strings, or expressions, or words) over V.
- a set of well-formed formulas (wff), over V, called the formulas of the theory.
- a subset of wff that is the set of theorems (thm) of the theory.

This is the _extension_ of a theory i.e. the explicit set of objects in it.

In most cases of interest to the mathematician a theory is given by V and two
sets of simple rules: formula-building rules and theorem-building rules.

Rules from the first set allow us to build, or generate, Wff from V.
The rules of the second set generate Thm from Wff.

In short, a theory consists of
- an alphabet of primitive symbols
- rules to generate the language of the theory (meaning, essentially, Wff) from these symbols
- rules of inference to generate the theorems


1. First off, the first order formal language, L, where the theory is discussed is a triple (V, Term, Wff), that is, it has three important components, each of them a set.
- V is the alphabet or vocabulary of the language. It is the collection of the basic syntactic symbols that we use to form expressions that are terms (members of Term) or formulas (members of Wff). We will ensure that the processes that build terms or formulas, using the basic building blocks in V, are algorithmic.
- Terms will formally codify objects
- formulas will formally codify statements about objects.

2. Reasoning in the theory will be the process of discovering true statements
about objects - theorems. This discovery begins with formulas which codify statements that we accept without proof, called axioms. There are two types of axioms:
- Special or nonlogical axioms describe specific aspects of any specific theory that we build. For example, $$x + 1 \neq 0$$ is a special axiom that contributes towards the characterization of number theory over the natural numbers.
- Logical axioms is a universally valid, theory-independent truth, found in all theories. For example, x = x.

3. The last component are the rules for reasoning, called rules of inference. These are the rules that allow us to deduce a true statement from other statements that we have already established as being true. These rules will be chosen to be oblivious to meaning, being only concerned with form. They will apply to statement forms, deriving new statements.

We may think of axioms of either logical or nonlogical type as special cases of rules, that is, rules that receive no input in order to produce an output. In this manner item (2) above is subsumed by item (3).


There are two parts in each first order alphabet. The first, the collection of the logical symbols, is common to all first order languages regardless of which theory is "spoken" in them.

**Logical Symbols** will have a fixed interpretation:
1. Object or individual variables.   
  An object variable is any one symbol out of the infinite sequence $$v_0, v_1, v_2, \dots$$. In practice, whether we are using logic as a tool or as an object of study, we agree to be sloppy with notation and use, generically, x, y, z, u, v, w with or without subscripts as names of object variables. This is just a matter of notational convenience. Object variables (intuitively) range over the objects that the theory studies (numbers, sets, atoms, lines, points, etc.).
2. The Boolean (or propositional) connectives; the symbols: ¬ ∨
3. The existential quantifier, ∃
4. The equality predicate, symbol =, to indicate equality
5. Parenthesis



## First-order logic
FOL uses quantified variables over non-logical objects and allows the use of sentences that contain variables. This distinguishes it from propositional logic, which does not use quantifiers or relations.

FOL without variables or quantifiers is sometimes called **zeroth-order logic**. Some authors use this phrase as a synonym for the propositional calculus, but others define it as an extended propositional logic by adding constants, operations and relations on non-Boolean values. Every zeroth-order language in this broader sense is complete and compact.

A theory about a topic is usually a FOL together with a specified **domain of discourse** over which the quantified variables range, finitely many functions from that domain to itself, finitely many predicates defined on that domain, and a set of axioms believed to hold for those things. Sometimes "theory" is understood in a more formal sense, which is just a set of sentences in FOL.




# First-order logic

- Atomic sentence is an indecomposable unit of language carrying a truth value.
- Individual constant is an expressions that refers to specific distinct object.
- Every individual constant must refer to exactly one actual object.
- An object can have any number of names: none, one, or more.
- Predicates are symbols used to express a property or relations between objects.
- Arity is the number of arguments needed for formation of an atomic sentence.
- Argument is a constant symbol used to fill in one of the slots associated with a predicate symbol.
- Predicate symbols in FOL have fixed arity.
- Each predicate is interpreted by the determinate property.
- Determinate property is matter of fact if an object posses it or not.


Components of predicate logic
- Atomic sentences (terms)
- Constants
- Variables, free and bound
- Predicates, predicate symbols
- Quantifiers
- Universe of discourse


**First-order logic** (FOL) is a family of languages, all having a similar grammar and sharing certain important vocabulary items, known as the connectives and quantifiers. Languages in this family can differ in the specific vocabulary used to form their most basic sentences, the so-called atomic sentences.

**Atomic sentence** is the most basic sentences in the language that can be used to make a claim about the world. That is, it's the smallest unit that can be said to have a truth value i.e. to be either true or false (not both, not neither).

Individual **constants** are symbols that refer to some distinct object and they function in FOL much the same way as the names do in English.

Constants are usually represented with a single lower case letter, picked from the front of the English alphabet, with or without the subscript and superscript.

To avoid ambiguity, each constant denotes an actual object in the world; it always has a concrete referent. Some objects are referred to by more than one constant and some (nameless generic objects) by no constant at all.

Aside: Names in English that do not refer to an actual object (Pegasus, Zeus, etc.) are disallowed in FOL, but a variant of FOL called **free logic** accepts referentless constants, providing a language suitable for formalizing, e.g. mythology and fiction.


**Predicate symbols** are symbols used to express some property of object or a relation between objects; in latter case they are sometimes called relation symbols.

As in English, **predicates** are expressions that when combined with names (acting as arguments) form atomic sentences.

An **argument** is a constant symbol when used to fill in one of the places associated with a predicate symbol to make an atomic sentence.

**Arity** specifies the number of individual constants the predicate symbol needs in order to form a sentence. Predicates in FOL have fixed arity.

Arity of a predicate symbol is the number of argument places associated with that symbol; this tells us the number of names required to make an atomic sentence from that predicate.

_Unary_ predicates require 1, _binary_ 2, _ternary_ 3 arguments; in general, an _n-ary_ (`n`-place) predicate requires `n` arguments. _Nullary_ predicate takes no arguments.

Unary predicate is used to represent some property of an object (e.g. `H(x)` for "is tall"). A relations between objects (e.g. `T(xy)` for "is taller than") is modelled using a binary predicate. Predicates with higher arity are also used to represent relations (e.g. `P(xyz)` for "x gave y to z"). Predicates with arity 1, 2 and 3 are the most common.

In FOL every predicate is interpreted by a **determinate property** (or a relation), which is a property of an object for which there is a factual matter whether or not the object posseses such property.

**Atomic sentence** is a predicate symbol applied to the appropriate number of names (arguments), making a claim about the world. The order of the names is crucial in forming atomic sentences.

Since predicates express determinate properties and names denote definite individuals, each atomic sentence of FOL must express a claim that is either true or false.

Since FOL is a general purpose language, we can use different individual constants and predicate symbols to talk about any **subject matter**.

The identity predicate is a binary predicate, symbolized with an infix operator. An atomic sentence containing an occurence of the identity predicate is true when the predicate's two arguments refer to the very same object.


# Logic glossary


## Abstract machine
An abstract machine is a theoretical model of a computer hardware or software system used in automata theory.

## Affine type system
Affine type systems allow exchange and weakening, but not contraction: every (occurrence of a) variable is used at most once. Affine types are a version of linear types allowing a resource to be unused. An affine resource can only be used once, while a linear resource must be used once.

## Axiom
An axiom (postulate) is a statement that is taken to be true, to serve as a premise or starting point for further reasoning and arguments. In classic logic, an axiom is an evident and well-established statement that needs no further proof. In modern logic, an axiom is a premise for reasoning. In brief, an axiom is a generally accepted statement.

## Church–Turing thesis
In computability theory, the Church–Turing thesis is a hypothesis about the nature of computable functions. It states that a function on the natural numbers is computable by a man following an algorithm, ignoring resource limitations, if and only if it is computable by a Turing machine.

## Conjecture
An unproved statement that is believed true is called a conjecture. To be considered a conjecture, a statement must usually be proposed publicly, at which point the name of the proponent may be attached to the conjecture, as with Goldbach's conjecture. Other famous conjectures include the Collatz conjecture and the Riemann hypothesis. On the other hand, Fermat's Last Theorem has always been known by that name, even before it was proved; it was never known as "Fermat's conjecture".

## Entscheidungsproblem
(german, "decision problem", [ɛntʃaɪ̯dʊŋspʁoblem]) is a challenge posed by David Hilbert in 1928, that asks an ant, named Chai Dung, what is his problem. More preciselly, he asks for an algorithm that takes as input a statement of a first-order logic and determines whether the statement is universally valid (valid in every structure satisfying the axioms). By the completeness theorem of first-order logic, a statement is universally valid if and only if it can be deduced from the axioms, so the decision problem can also be viewed as asking for an algorithm to decide whether a given statement is provable from the axioms using the rules of logic. In 1936, Alonzo Church and Alan Turing published independent papers showing that a general solution to the decision problem is impossible.

## First-order logic
While propositional logic deals with simple declarative propositions, first-order logic additionally covers predicates and quantification. Predicate logic uses quantified variables over objects and allows the use of sentences that contain variables, so rather than propositions such as "Socrates is a man" one can have expressions in the form "there exists X such that X is Socrates and X is a man" and "there exists" is a quantifier while "X" is a variable. This distinguishes it from propositional logic, which does not use quantifiers or relations. In first-order theories, predicates are often associated with sets.

## Formal logic
Formal logic is the study of inference with purely formal content. An inference possesses a purely formal content if it can be expressed as a particular application of a wholly abstract rule i.e. a rule that is not about any particular thing or property. 

From the second half of the XIX century, formal and rigorous mathematical methods have started to be used in the study of logic, which has introduced a shift towards a symbolic notation and artificial languages. In 1933, with Tarski, it culminated with the withdrawal from the absolute notion of truth and instead focused on the particular truths of concrete structures and models.

## Fuzzy logic
Fuzzy logic is a form of many-valued logic in which the truth values of variables may be any real number between 0 and 1. It is employed to handle the concept of partial truth, where the truth value may range between completely true and completely false. By contrast, in Boolean logic, the truth values of variables may only be the integer values 0 or 1.

## Linear type system
Linear type systems allow exchange, but neither weakening nor contraction: every variable is used exactly once. Linear types ensures that objects are used exactly once, allowing the system to safely deallocate an object after its use. Linear types can be used to model heap-based memory allocation.

## Logical form
A logical form of a syntactic expression is a precisely-specified semantic version of that expression in a formal system. Informally, the logical form attempts to formalize a possibly ambiguous statement into a statement with a precise, unambiguous logical interpretation with respect to a formal system.

## Informal logic
Informal logic is the study of natural language arguments. The study of fallacies is an important branch of informal logic. Since much informal argument is not strictly speaking deductive, on some conceptions of logic, informal logic is not logic at all.

### Mathematical logic
Mathematical logic is an extension of symbolic logic into other areas, in particular to the study of model theory, proof theory, set theory, and computability (recursion) theory.

## Mathematical object
An abstract object that can be formally defined, and which one may do deductive reasoning and mathematical proofs with.

## Ordered type system
Ordered type systems discard exchange, contraction, and weakening: every variable is used exactly once in the order it was introduced. Ordered types can be used to model stack-based memory allocation (contrast with linear types which can be used to model heap-based memory allocation). Without the exchange property, an object may only be used when at the top of the modelled stack, after which it is popped off resulting in every variable being used exactly once in the order it was introduced.

## Postulate
see Axiom.

## Predicate
A predicate takes an entity or entities in the domain of discourse as input while outputs are either True or False. Informally, a predicate is a statement that may be true or false depending on the values of its variables. For example, predicates can be used in set builder notation to indicate set membership: a predicate P(x) will be true or false, depending on whether x belongs to a set.

## Premise
A premise is a statement that an argument claims will induce or justify a conclusion. A premise is an assumption that something is true.

# Propositions
Propositions are declarative sentences that have a truth value.

## Propositional logic
Propositional (sentential) logic is based on propositions and argument flow. Compound propositions are formed by connecting propositions by logical connectives. The propositions without logical connectives are called atomic propositions. Unlike first-order logic, propositional logic does not deal with non-logical objects, predicates about them, or quantifiers. However, all the machinery of propositional logic is included in first-order logic and higher-order logics. In this sense, propositional logic is the foundation of first-order logic and higher-order logic.

## Logical connectives
Logical connectives (logical operators) are symbols or words used to connect sentences in a grammatically valid way, such that the value of the compound sentence produced depends only on that of the original sentences and on the meaning of the connective. Common logical connectives are negation, conjunction, disjunction, biconditional, implication.

## Relevant type system
Relevant type systems allow exchange and contraction, but not weakening: every variable is used at least once.

## Second-order logic
Ssecond-order logic is an extension of first-order logic, which itself is an extension of propositional logic. Second-order logic is in turn extended by higher-order logic and type theory. First-order logic quantifies only variables that range over individuals (elements of the domain of discourse), second-order logic additionally quantifies over relations.

## Substructural type system
Substructural type systems are a family of type systems analogous to substructural logics where one or more of the structural rules are absent or only allowed under controlled circumstances. Such systems are useful for constraining access to system resources such as files, locks and memory by keeping track of changes of state that occur and preventing invalid states. Substructural type systems: linear, affine, relevant, ordered.

## Syllogisms
A syllogism is a kind of logical argument where a quantified statement of a specific form (the conclusion) is inferred from two other quantified statements (the premises). It is a logical argument where one starts with premises and reaches a conclusion.

A syllogism is a kind of logical argument that applies deductive reasoning to arrive at a conclusion based on two or more propositions that are asserted or assumed to be true. In its earliest form, defined by Aristotle, from the combination of a general statement (the major premise) and a specific statement (the minor premise), a conclusion is deduced.

## Symbolic logic
Symbolic logic is the study of symbolic abstractions that capture the formal features of logical inference. It is often divided into two main branches, propositional and predicate logic.

## Theorem
A theorem is a statement that has been proven on the basis of previously established statements (other theorems) and generally accepted statements (axioms). It is a logical consequence of the axioms.

## Truth value
Truth or logical value is a value indicating the relation of a proposition to truth. A proposition has a truth value if it can be evaluated to true or false.

## Turing machine
A Turing machine (TM) is a mathematical model of computation that defines an abstract machine, which manipulates symbols on a strip of tape according to a table of rules. Given any algorithm, a TM capable of simulating that algorithm's logic can be constructed. The Turing machine was "invented" in 1936 by Alan Turing.

## Turing completeness
Turing completeness is the ability for a system of instructions to simulate a Turing machine. A programming language that is Turing complete is theoretically capable of expressing all tasks accomplishable by computers. Almmost all programming languages are Turing complete.

## Universal Turing machine (UTM)
A Turing machine that is able to simulate any other TM is called a universal Turing machine.

## Validity
In logic, an argument is valid if and only if it takes a form that makes it impossible for the premises to be true and the conclusion nevertheless to be false. It is not required that a valid argument have premises that are actually true, but to have premises that, if they were true, would guarantee the truth of the argument's conclusion.

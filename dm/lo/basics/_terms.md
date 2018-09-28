<!-- TOC -->

- [Logic Concepts](#logic-concepts)
- [Logical Form](#logical-form)
- [Validity](#validity)
- [Validity and deductive logic](#validity-and-deductive-logic)
- [Metatheorem](#metatheorem)
- [Logical Framework](#logical-framework)
- [Extensionality](#extensionality)

<!-- /TOC -->


## Logic Concepts
Logic arose from a concern with correctness of argumentation. Modern logicians usually wish to ensure that logic studies just those arguments that arise from appropriately general forms of inference.

However, logic does not cover good reasoning as a whole - that is the job of the theory of rationality.

Logic deals with inferences whose validity can be traced back to the formal features of the representations that are involved in that inference, be they linguistic, mental, or other representations".

Logic has also been defined as the study of arguments correct in virtue of their form.

The idea that logic treats special forms of argument, deductive argument, rather than argument in general, has a history in logic that dates back at least to logicism in mathematics (19th and 20th centuries) and the advent of the influence of mathematical logic on philosophy.

A consequence of taking logic to treat special kinds of argument is that it leads to identification of special kinds of truth, the logical truths (with logic equivalently being the study of logical truth), and excludes many of the original objects of study of logic that are treated as informal logic.

Robert Brandom has argued against the idea that logic is the study of a special kind of logical truth, arguing that instead one can talk of the logic of material inference (in the terminology of Wilfred Sellars), with logic making explicit the commitments that were originally implicit in informal inference.


## Logical Form
The concept of logical form is central to logic. The validity of an argument is determined by its logical form, not by its content.

A **logical form** of a syntactic expression is a precisely specified semantic version of that expression in a formal system.

Informally, the logical form attempts to formalize a possibly ambiguous statement into a statement with a precise, unambiguous logical interpretation with respect to a formal system.


# Satisfiability and validity

Satisfiability and validity are elementary concepts of semantics.

A formula is **satisfiable** if it is possible to find an interpretation (model) that makes the formula true.

A formula is **valid** if all interpretations make the formula true.

The opposites of these concepts are unsatisfiability and invalidity.

A formula is **unsatisfiable** if none of the interpretations make the formula true.

A formula is **invalid** if some such interpretation makes the formula false.

These 4 concepts are related to each other in a manner exactly analogous to Aristotle's square of opposition.


## Validity
In logic, an argument is valid iff it takes a form that makes it impossible for the premises to be true and the conclusion nevertheless to be false.

It is not required that a valid argument have premises that are actually true, but to have premises that, if they were true, would guarantee the truth of the argument's conclusion.

A formula is valid iff it is true under every interpretation.

An argument form (or schema) is valid iff every argument of that logical form is valid.


## Validity and deductive logic
Arguments in which the truth of the premises guarantees the truth of the conclusion are known as `valid`, while those where the truth of the premises makes the truth of the conclusion likely, but not certain, are called `inductively strong`. These two properties, `validity` and `inductive strength`, have given rise to __deductive__ and _inductive_ logic, respectively.

The combination of true premises and validity actually has its own name: arguments that are valid and have true premises are called __sound arguments__. Not all valid arguments are sound (since some of their premises could be false), but any sound argument is necessarily valid.





## Metatheorem
In logic, a metatheorem is a statement about a formal system proven in a metalanguage. Unlike theorems proved within a given formal system, a metatheorem is proved within a metatheory, and may reference concepts that are present in the metatheory but not the object theory.


## Logical Framework
provides a means to define a logic as a signature in a higher-order type theory in such a way that provability of a formula in the original logic reduces to a type inhabitation problem in the framework type theory.


## Extensionality
In any of several studies that treat the use of signs (e.g. linguistics, logic, mathematics, semantics, semiotics) the **extension** of a sign (concept, idea) consists of the things to which it applies, in contrast with its **intension** (comprehension) which consists, very roughly, of the ideas, properties, or corresponding signs that are implied or suggested by the concept in question.

In logic, __extensionality__, or _extensional equality_, refers to principles that judge objects to be equal if they have the same external properties. It stands in contrast to the concept of __intensionality__, which is concerned with whether the internal definitions of objects are the same.

For example, consider the two functions f and g mapping from and to natural numbers, defined as follows:
- To find f(n), first add 5 to n, then double it: λn.2(n+5)
- To find g(n), first double n, then add 10:      λn.2n+10

These functions are _extensionally equal_ ("from outside"), but the definitions of the functions are not equal, and in that intensional ("internal") sense the functions are not the same.

There are many predicates (relations) that are intensionally different but extensionally identical.

In logic, the __comprehension__ of an object is the __totality of intensions__, that is, attributes, characters, marks, properties, or qualities, that the object possesses, or else the totality of intensions that are pertinent to the context of a given discussion. This is the correct technical term for the whole collection of intensions of an object, but it is common in less technical usage to see 'intension' used for both the composite and the primitive ideas.


# Types of Logic

- Traditional logic (Syllogistic, Term logic)
- Informal logic
- Formal logic
- Symbolic logic
- Symbolic logic
- Propositional logic

Predicate logic

**Term logic**, also known as **traditional** logic, **syllogistic** logic or **Aristotelian** logic, is a loose name for an approach to logic that began with Aristotle and that was dominant until the advent of modern logic in the late XIX century.

__Informal logic__ is the study of natural language arguments. The study of fallacies is an important branch of informal logic.

**Formal logic** is the study of inference with purely formal content. An inference possesses a purely formal content if it can be expressed as a particular application of a wholly abstract rule i.e. a rule that is not about any particular thing or property. In many definitions of logic, logical inference and inference with purely formal content are the same. Formal logic has started to take shape from the second half of the XIX century, with the use of formal and rigorous mathematical methods in the study of logic, which has introduced a shift towards a symbolic notation and language. The works of Aristotle contain the earliest known formal study of logic; modern formal logic follows and expands on Aristotle.

**Symbolic logic** is the study of symbolic abstractions that capture the formal features of logical inference. It is often divided into two main branches: propositional and predicate logic.

**Mathematical logic** is a subfield of mathematics exploring the applications of formal logic to mathematics. The unifying themes in mathematical logic include the study of the expressive power of formal systems and the deductive power of formal proof systems. Math logic is often divided into the fields of set theory, model theory, computability theory, proof theory, constructive mathematics. These areas share basic results on logic, particularly first-order logic, and definability.


**Propositional logic** deals with propositions and argument flow. Compound propositions are formed by connecting propositions by logical connectives. The propositions without logical connectives are called atomic propositions. Unlike first-order logic, propositional logic does not deal with non-logical objects, predicates about them, or quantifiers. However, all the machinery of propositional logic is included in first-order logic and higher-order logics. In this sense, propositional logic is the foundation of first-order logic and higher-order logic.

**Zeroth-order logic** is, under one definition, first-order logic without variables or quantifiers. Another definition extends propositional logic by adding constants, operations, and relations on non-Boolean values. Every zeroth-order language in this broader sense is complete and compact.

**Predicate logic** is in principle sufficient for math as it has a sound and complete deduction system and satisfies important semantic results. Lindstrom's theorems show that there can be no logical system with more expressive power than predicate logic and with the same good semantic properties.



---

CS has seen a multitude of logics suited for different purposes.

__Modal logic__ is used for reasoning about concepts like possibility or necessity; it extends classical logic with non-truth-functional, "modal", operators. __Probabilistic logic__ has the truth values of formulas as probabilities.

**Temporal logic** is any system of rules and symbolism for representing, and reasoning about, propositions qualified in terms of time. Temporal logic has found an important application in formal verification, where it is used to state requirements of hardware or software systems. For instance, one may wish to say that whenever a request is made, access to a resource is eventually granted, but it is never granted to two requestors simultaneously.

Moreover, these logics come in different flavors, usually admitting propositional, first-order, higher-order, and intuitionistic presentations, as well as combinations of these and many ad hoc variants. 

**Non-classical** logics are formal systems that significantly differ from standard logical systems such as propositional and predicate logic. There are several ways in which this is done, including by way of extensions, deviations and variations. The aim of these departures is to make it possible to construct different models of logical consequence and logical truth.

__Computability logic__ is a semantically constructed formal theory of computability (as opposed to classical logic, which is a formal theory of truth); it integrates and extends classical, linear and intuitionistic logics.


__Many-valued logics__ rejects bivalence, allowing for truth values other than just binary _true_ and _false_. The most popular forms are __three-valued logic__, as initially developed by Jan Łukasiewicz, and __infinitely-valued logics__ such as __fuzzy logic__, which deals with approximate, vague concepts such as the distinction between "warm" and "hot", so it permits any real number between 0 and 1 as a truth value.

__Intuitionistic logic__, sometimes more generally called __constructive logic__, refers to systems of symbolic logic that differ from the systems used for classical logic by more closely mirroring the notion of `constructive proof`. In particular, systems of intuitionistic logic do not include `the law of the excluded middle` and `double negation elimination`, which are fundamental inference rules in classical logic.

__Paraconsistent logic__ rejects the `principle of explosion`, while __linear logic__ rejects `idempotency of entailment` as well. Linear logic, __relevance logic__ and non-monotonic logic reject `monotonicity of entailment`. __Nonmonotonic logic__ takes the stand that an established piece of knowledge may have to be retracted if additional facts are later known. __Non-reflexive logic__, also known as "Schrödinger logics", rejects or restricts `the law of identity`.

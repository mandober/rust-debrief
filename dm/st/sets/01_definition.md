# Introduction to sets

## Terms
- Set
- Set element
- Comprehension principle
- Mathematical object
- Primitive notion
- Russell's paradox
- Set theory
- Naïve set theory
- Axiomatic set theory


## Definition
Sets and set theory were invented, almost single-handedly, by the German mathematician Georg Cantor, at the end of the XIX century. The word for "set" in German is "_Menge_".

Initially developed theory of sets that treats sets just as collections of things is called **naive set theory** (here, _naive_ means non-axiomatic).

Broadly, a set is an unordered collection of distinct objects.

A set may be intuitively defined as a well-defined collection of objects that share a common property.

Anything you care to think of can be a set. This concept is known as the **comprehension principle**, which states that given any property, there is a set which consists of all objects having that property.

The most broad defnition is that a set is a collection of objects. Since a notion of set lacks a formal definition, the views about what can constitute a set vary.

> A set is an unordered collection of distinct objects.

The difficulty in composing a suitable formal definition may be the fact that plethora of math objects resemble a set of some kind, making it hard to come up with a strict, precise definition that will demark the appropriate boundaries.

Nevertheless, since sets are so ubiquitous and unavoidable, we must accept the term "set", and related terms, such as "element", as a primitive notion, and with math as our subject, it wouldn't hurt to strenghten the requirements and allow only mathematical objects that share a common property to form a set.

> A set is an unordered collection of distinct mathematical objects that share a common property.

However, the assumption that any property may be used to form a set leads to paradoxes, with the **Russell's paradox** being the most famous.

Thus a consistent system of naïve (non-axiomatic) set theory must impose restrictions on the principles which can be used to form sets.

Russell's paradox is about self-reference ("whenever there's self-referencing, a paradox lurks near by", as was said) and affects the theories that grant set membership without restriction.

In the Russell's paradox, the set $$R$$ is a set of sets - its members are themselves sets. The common property all the members share is "not being a member of self". Usually, the sets don't contain themselves (this would lead to infinite recursion) and overwhelming number of sets does qualify. So, all the sets (now in the role of element) with this property **are** (it is not a choice) members of the set $$R$$. Finally, the question becomes, whether the set $$R$$ contains itself.

If $$R$$ is not a member of itself, then its definition dictates that it must contain itself (because it does have the required property). But if it contains itself, then it contradicts its own definition (because it doesn't have the required property).

> Let R be the set of all sets that are not members of themselves. If R is not a member of itself, then by definition it must contain itself. And if it does contains itself, then it contradicts its own definition.

Resolving the Russell's and related paradoxes has yield a number of set theories.

**Naïve set theory** is an approach to set theory which assumes the existence of a universal set, despite the fact that such an assumption leads to paradoxes. It is founded upon the rigid set of axioms of propositional and predicate logic.

**Axiomatic set theory** is a system that constrains the sets, which are allowed to be generated, by axioms. The best known axiomatic system is Zermelo-Fraenkel set theory.

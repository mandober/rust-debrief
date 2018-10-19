# Set Definition


Sets and set theory were invented, almost single-handedly, by the German mathematician Georg Cantor, at the end of the XIX century. The word for "set" in German is "_Menge_". Initially, **naive set theory** (here, "naive" means non-axiomatic) was developed, which treated sets as collections of objects.

> A set is an unordered collection of distinct objects.

A set may be defined as a well-defined collection of objects that share a common property. Here, "well-defined" is taken to mean that the principle by which elements are included in a set is completely unambiguous.

**Comprehension principle** states that given any property, there is a set consisting of all the objects having that property. This is a position that allows anything and everything to constitute a set.

The opionions about what can constitute a set vary in largly because the term "set" lacks a formal definition. The difficulty in composing a suitable formal definition may be the fact that plethora of math objects resemble a set of some kind, making it hard to come up with a strict, precise definition that will demark the appropriate boundaries.

Nevertheless, since sets are so ubiquitous and unavoidable, we must accept it as a primitive notion. With math as subject, the requirements may be strenghtened so only mathematical objects that share a common property are allowed as members of a set.

> A set is an unordered collection of distinct mathematical objects that share a common property.

However, the assumption that any property may be used to form a set leads to paradoxes, with the **Russell's paradox** being the most famous. A consistent system of naïve set theory must impose restrictions on the set inclusion principles.

Russell's paradox is about self-reference ("whenever there's self-referencing, a paradox lurks near by", as they say) and affects the theories that grant set membership without restriction.

In the Russell's paradox, the set $$R$$ is a set of sets - its members are themselves sets. The common property all the members share is "not being a member of self". Usually, the sets don't contain themselves (this would lead to infinite recursion) and overwhelming number of sets does qualify. So, all the sets (now in the role of element) with this property **are** (it is not a choice) members of the set $$R$$. Finally, the question becomes, whether the set $$R$$ contains itself.

If $$R$$ is not a member of itself, then its definition dictates that it must contain itself (because it does have the required property). But if it contains itself, then it contradicts its own definition (because it doesn't have the required property).

> Let R be the set of all sets that are not members of themselves. If R is not a member of itself, then by definition it must contain itself. And if it does contains itself, then it contradicts its own definition.

Resolving the Russell's and related paradoxes has yield a number of set theories.

**Naïve set theory** is an approach to set theory which assumes the existence of a universal set, despite the fact that such an assumption leads to paradoxes. It is founded upon the rigid set of axioms of propositional and predicate logic.

**Axiomatic set theory** is a system that constrains the sets, which are allowed to be generated, by axioms. The best known axiomatic system is Zermelo-Fraenkel set theory.

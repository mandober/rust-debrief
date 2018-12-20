# Set Fundamentals

> Sets are considered as foundation of all mathematics. They are extensively used in logic, and, by extension, in CS.


Sets were introduced by the German mathematician Georg Cantor, at the end of the XIX century. _"A set is a gathering together into a whole of definite, distinct objects of our perception or of our thought - which are called elements of the set."_ -- Georg Cantor

Initially, **naive set theory** (meaning non-axiomatic) was developed, which treated sets as collections of objects and any kind of objects could be gather together to form a set.

> A set is an unordered collection of distinct objects.

A set may, slightly better, be defined as a well-defined collection of objects that share a common property, but the term "well-defined" cannot, by itself, guarantee the consistency and unambiguity of what exactly constitutes a set; in the best case, it may be taken to mean that the principle by which elements are included in a set is clear and unambiguous.

**Comprehension principle** states that given any property, there is a set consisting of all the objects having that property. This is a position that allows anything and everything to constitute a set.

The problem is that set does not have a formal definition, but being so ubiquitous and unavoidable in math, with many mathematical objects resembing a set of some kind, we must accept the term "set" as a primitive, sparing sets from rigorous scrutiny.

> A set is an unordered collection of distinct mathematical objects that share a common property.

The lack of formal definition lets any set-forming property be used, which quickly leads the paradoxes, among which, the **Russell's paradox**, as the best-known one.

Russell's paradox is about self-reference ("whenever there's self-referencing, a paradox lurks near by") and affects the set theories that grant set membership to object without restriction.

> Russell's paradox: Let R be the set of all sets that are not members of themselves. If R is not a member of itself, then by definition it must contain itself. And if it does contains itself, then it contradicts its own definition.

In the Russell's paradox, the set $$R$$ is a set of sets - its members are themselves sets. The common property all the members share is "not being a member of self". Usually, the sets don't contain themselves (this would lead to infinite recursion) and overwhelming number of sets does qualify. So, all the sets (now in the role of element) with this property **are** (it is not a choice) members of the set $$R$$. Finally, the question becomes, whether the set $$R$$ contains itself. If $$R$$ is not a member of itself, then its definition dictates that it must contain itself (because it does have the required property). But if it contains itself, then it contradicts its own definition (because it doesn't have the required property).

A consistent system of naïve set theory must impose restrictions on the set inclusion principles. Resolving Russell's and related paradoxes has yield a number of set theories, which tried to restrict properties used to form a set, bringing this problem towards axiomatic set or class theory.

**Naïve set theory** is an approach to set theory which assumes the existence of a universal set, despite the fact that such an assumption leads to paradoxes. It is founded upon the set of axioms of propositional and predicate logic. It is still regarded as a useful introduction to sets.

**Axiomatic set theory** is a system that constrains the sets, which are allowed to be generated, by axioms. The best known axiomatic system is Zermelo-Fraenkel set theory, which is regarded as a possible foundation of all mathematics.

---

> "Unter einer 'Menge' verstehen wir jede Zusammenfassung M von bestimmten wohlunterschiedenen Objekten m unserer Anschauung oder unseres Denkens (welche die 'Elemente' von M genannt werden) zu einem Ganzen." -- Georg Cantor

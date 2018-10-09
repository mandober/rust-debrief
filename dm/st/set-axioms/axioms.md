# Axioms


An **axiom schema** is a formula in the metalanguage of an axiomatic system, in which one or more schematic variables appear. These variables, which are metalinguistic constructs, stand for any term or subformula of the system, which may or may not be required to satisfy certain conditions. Often, such conditions require that certain variables be free, or that certain variables not appear in the subformula or term

A **schema** is a linguistic "template" together with a rule for using it to specify a potentially infinite multitude of phrases, sentences, or arguments, which are called instances of the schema. Schemas are used in logic to specify rules of inference, in mathematics to describe theories with infinitely many axioms, and in semantics to give adequacy conditions for definitions of truth

A **class** is a collection of sets (or sometimes other mathematical objects) that can be unambiguously defined by a property that all its members share.

The precise definition of "class" depends on foundational context. In ZF the notion of class is informal, whereas NBG axiomatizes the notion of "proper class" e.g. as entities that are not members of another entity.

In ZF, a class that is not a set is called a **proper class**, and a class that is a set is sometimes called a **small class**. For instance, the class of all ordinal numbers, and the class of all sets, are proper classes in many formal systems.

A **subclass** is a class contained in some other class in the same way that a subset is a set contained in some other set.

That is, given classes A and B, A is a subclass of B iff every member of A is also a member of B. If A and B are sets, then of course A is also a subset of B. In fact, when using a definition of classes that requires them to be first-order definable, it's enough that B be a set; the axiom of specification essentially says that A must then also be a set.

As with subsets, the empty set is a subclass of every class, and any class is a subclass of itself. But additionally, every class is a subclass of the class of all sets.

Accordingly, the subclass relation makes the collection of all classes into a Boolean lattice, which the subset relation does not do for the collection of all sets. Instead, the collection of all sets is an ideal in the collection of all classes. Of course, the collection of all classes is something larger than even a class!

**Extensions by definitions** formalize the introduction of new symbols by means of a definition. For example, it is common in naive set theory to introduce a symbol $$\varnothing$$ for the set which has no member i.e. empty set. In the formal setting of first-order theories, this can be done by adding to the theory a new constant $$\varnothing$$ and the new axiom $$\forall x(x\notin \varnothing)$$, meaning "for all x, x is not a member of $$\varnothing$$". Doing this adds essentially nothing to the old theory, as should be expected from a definition - more precisely, the new theory is a conservative extension of the old one.






---

- Axiom of Extension: 
  two sets are equal iff they contain the same elements.
- Axiom of Regularity: 
- Axiom Schema of Specification: 
- Axiom of Pairing: 
  for any two sets, there exists a set to which only those two sets belong.
- Axiom of Union:  
  for every collection of sets, there exists a set that contains all the elements that belong to at least one of the sets in the collection.
- Axiom of Replacement: 
  for any set $$S$$, there exists a set $$x$$ such that, for any element $$y$$ of $$S$$, if there exists an element $$z$$ satisfying the condition $$P(y,z)$$, where $$P(y,z)$$ is a propositional function, then such $$z$$ appear in $$x$$.
- Axiom of the Empty Set: 
  there exists a set that has no elements.
- Axiom of Subsets: 
  for every set and every condition, there corresponds a set whose elements are exactly the same as those elements of the original set for which the condition is true.
- Axiom of Powerset: 
  for each set there exists a collection of sets that contains amongst its elements all the subsets of the given set.
- Axiom of Infinity: 
  there exists a set containing a set with no elements and the successor of each of its elements.
- Axiom of Foundation: 
  for all non-null sets, there is an element of the set that shares no member with the set.
- Axiom of Choice: 
  for every set, we can provide a mechanism for choosing one element of any non-empty subset of the set.



---

[Choice](https://www.wikiwand.com/en/Axiom_of_choice)
[countable](https://www.wikiwand.com/en/Axiom_of_countable_choice)
[dependent](https://www.wikiwand.com/en/Axiom_of_dependent_choice)
[Constructibility (V=L)](https://www.wikiwand.com/en/Axiom_of_constructibility)
[Determinacy](https://www.wikiwand.com/en/Axiom_of_determinacy)
[Extensionality](https://www.wikiwand.com/en/Axiom_of_extensionality)
[Infinity](https://www.wikiwand.com/en/Axiom_of_infinity)
[Limitation of size](https://www.wikiwand.com/en/Axiom_of_limitation_of_size)
[Pairing](https://www.wikiwand.com/en/Axiom_of_pairing)
[Power set](https://www.wikiwand.com/en/Axiom_of_power_set)
[Regularity](https://www.wikiwand.com/en/Axiom_of_regularity)
[Union](https://www.wikiwand.com/en/Axiom_of_union)
[Martin's axiom](https://www.wikiwand.com/en/Martin%27s_axiom)
[Axiom schema](https://www.wikiwand.com/en/Axiom_schema)
[replacement](https://www.wikiwand.com/en/Axiom_schema_of_replacement)
[specification](https://www.wikiwand.com/en/Axiom_schema_of_specification)


https://proofwiki.org/wiki/Axiom:Axiom_of_Extension
https://proofwiki.org/wiki/Axiom:Zermelo-Fraenkel_Axioms


# Development

The fundamental idea behind type theory is being able to distinguish between different types (classes) of objects.

Before the introduction of formalization to math and logic in the end of XIX, the types were present in mathematics and logic but implicit, given only informally or taken for granted, since no formal representations of types existed. Discovery of new paradoxes inspired the development of type theory, especially the paradox that Bernard Russell found in relation to Cantor's set theory, in Frege's _Begriffsschrift_, which became known as the Russell's paradox. Although paradoxes have always existed, the importance of Russell's paradox is that it was applicable to many theories that were forming at the time, like Cantor's, Frege's and Peano's formal systems. Being at the very basis of logic, Russell's paradox couldn't be disregarded and a solution had to be found.


## Ramified Type Theory
Russell introduced types, in 1908 edition of his and Whitehead's _Principia Metematica_, in hope of avoiding paradoxes, especially the Russell paradox that occurs in face of impredicativity.

Something that is **impredicative** contains reserences to itself, it is a self-referencing definition. Roughly speaking, a definition is impredicative if it mentions (or quantifies over) the set being defined, or, more commonly, another set that contains the thing being defined.

Russell's paradox is an example of an impredicative construction - namely the set of all sets that do not contain themselves. The paradox is that such a set cannot exist, because if it did, the question becomes whether it contains itself or not: if it does, then by definition it should not, and if it doesn't, then by definition it should.

On the other hand, **predicativity** entails building **stratified** (or ramified) theories with quantification over lower levels of entities that results in variables of some new type, distinguished from the lower types that the variable ranges over. An example is intuitionistic type theory, which retains stratification (ramification) in order to avoid impredicativity.

Therefore, Russell's plan was to avoid impredicativity by applying the vicious circle principle: "_Whatever involves all of a collection must not be one of the collection_". Types in **Ramified Type Theory** (RTT) of _Principia_ have a double hierarchy: one of simple types and one of orders. The notion of a general set was replaced with the hierarchy of sets of different types - a set of a certain type is only allowed to contain sets of strictly lower types. The primary objects are assigned to type 0, the properties of individuals to type 1, properties of properties of individuals to type 2, and so on, and no properties which do not fall into one of these logical types are allowed.

In _Principia_, Whitehead and Russell attempted to found mathematics on logic, the approach that is known as **logicism**. The result was a very formal and accurate build-up of mathematics, avoiding the logical paradoxes, however, the description of RTT, though extensive, was still informal.


## Simple type theory
In 1920s, Chwistek and Ramsey noticed that, if one is willing to give up the vicious circle principle, the hierarchy of levels of types in the RTT can be collapsed. 

The **vicious circle principle** states that no object or property may be introduced by a definition that depends on that object or property itself.

This resulted in the theory of simple types, commonly called, simple type theory (STT). In 1940 Alonzo Church reformulated it as **simply typed lambda calculus**.

The adjective "simple" in STT doesn't mean the theory is simple, but restricted: types of different orders are not allowed to mix. Mixed types, such as classes containing individuals and classes as elements, and also transfinite types, such as the class of all classes of finite types, are excluded.

In 1944, in his book _Russell's mathematical logic_, Kurt Gödel mentioned the STT writing that it does avoid all known paradoxes thus enabling the derivation of all mathematics, but that it needs further development and elucidation.


## Curry-Howard correspondence
The Curry-Howard correspondence is the interpretation of proofs-as-programs and formulae-as-types. The idea starting in 1934 with Haskell Curry and finalized in 1969 with William Alvin Howard. It connected the "computational component" of many type theories to the derivations in logics.

Howard showed that the typed lambda calculus corresponded to intuitionistic natural deduction (natural deduction without the Law of excluded middle).

The connection between types and logic lead to a lot of subsequent research to find new type theories for existing logics and new logics for existing type theories.

---

in 1967, Nicolaas Govert de Bruijn created the **Automath type theory** as a mathematical foundation for the Automath system which could verify the correctness of proofs. It allowed expressing complete mathematical theories in such a way that an included automated proof checker can verify their correctness. The system developed and added features over time as type theory developed.

In 1972, Per Martin-Löf founded a type theory that corresponded to predicate logic by introducing dependent types, which became known as **intuitionistic type theory** (ITT) or **Martin-Löf type theory** (MTT). One of its key features is that it unifies set theory and logic.

In 1991, Barendregt introduced the **lambda cube**, which wasn't a type theory but a categorization of existing type theories. The eight corners of the cube included some existing theories with simply typed lambda calculus at the lowest corner and the calculus of constructions at the highest.

In 2006, Awodey and Warren, and Voevodsky, discovered that intuitionistic type theory has homotopical models which led to the development of **category type theory**.

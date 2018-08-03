# Set theory

- The theory of sets, initiated by Georg Cantor (1845–1918), constitutes the
basis of almost all modern mathematics.
  - The set concept itself cannot be defined in simpler terms.
  - A set is often described as a collection (aggregate, class, totality, family) of objects of any specified kind.
  - The term "set" must be accepted as a primitive notion, without definition. 
- Sets are usually denoted with capital letters: $A, B, \ldots, Y, Z$
  - $A=\{a,b,c\}$
  - Infinite sets: $\{1,2,3,\ldots\}$, or $\{\ldots,1,2,3,\ldots\}$
  - with terminal member: $\{1,2,3, \ldots,100\}$, or $\{1,3,5, \ldots,2n - 1\}$
  - $\{a,b\}=\{b, a\}$, also $\{a\}=\{a, a\}$, also $\{\{\}\}=\{\{\}\{\}\}$
  - set of one element is not the same as that element alone: $p \neq  \{p\}$
  - $\varnothing=\{\}$, and $\{\{\}\}=\{\varnothing\}$, so: $\varnothing \neq \{\varnothing\}$
- The objects belonging to a set A are called its __elements__ or __members__:
  - if x is an element of the set A: $x \in A$, if not:  $x \notin A$
- A set that contains no elements is __empty set__: $\varnothing$ or $\{\}$
  - instead of saying there are no objects of some specific kind, we say the set of these elements is empty
  - the empty set itself is regarded as an existing object.
  - $\varnothing$ is considered a subset of any set
  - $\varnothing$ is a proper subset of any nonempty set
- A set $\{a,b\}$ is distinguished from the ordered pair $(a,b)$
  - n element set is different from ordered n-tuple: $\{x_1, x_2,\dots,x_n\} \neq  (x_1,\dots, x_n)$
  - Two ordered pairs are equal iff: $(a,b) = (x,y) \iff a=x \wedge b = y$
- Once a set has been formed, it is a new entity, different from any of its elements.
  - in turn, this newly formed set may be an element of some other set
  - sets whose elements are other sets are collections of sets (families of sets, classes of sets)
  - but $x \in A \wedge A \in \mathscr{M} \not \rightarrow x \in A$
- If all elements of a set A are also elements of a set B:
  - A is a __subset__ of B: $A \subseteq B$, and B is a __superset__  of A: $B \supseteq A$
- The set B is __equal__ to A if the two sets consist of exactly the same elements
  - $A = B$ if $A \subseteq B \wedge B \subseteq A$
  - this statement shall be treated as an axiom, not a definition
- If $A \subseteq B$ but  $B \neq A$ then A is a __proper subset__ of B: $A \subset B$
- $\forall A, B, C$:
  1. $A \subseteq A$ (reflexivity)
  2. $A \subseteq B \wedge B \subseteq C \rightarrow A \subseteq C$  (transitivity)
  3. $A \subseteq B \wedge B \subseteq A \rightarrow A=B$ (anti-symmetry, axiom of extensionality)
  4. $\varnothing \subseteq A$
  5. $A\subseteq \varnothing \rightarrow A=\varnothing$
- If $P(x)$ is some proposition involving a variable $x$, we shall use the symbol  $\{x|P(x)\}$ to denote the set of all objects $x$ for which the formula $P(x)$ is true.
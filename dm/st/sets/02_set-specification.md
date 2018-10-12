# Set specification
math.dm.st.sets.spec


- Extensional (roster) definition
- Intensional (semantic) definition
- Set-builder notation (set comprehension)
- Infinite set
- Finite set
- Terminal set member


## Specifing a set
Standard notation for specifing a set is to list its elements (comma separated) between the curly braces: $$\{1,2,3\}$$

A set is an unordered collection of distinct elements - repetitions of elements or their order don't influence the set, i.e. the set $$\{1,2,3\}$$ is the same set as $$\{3,1,1,2,3\}$$.

A set is uniquely determined by its elements, which is known as the **uniqueness of (set) elements**. This means that the only thing that defines what a set *really is*, are its contents, its elements. However you choose to define the contents makes no difference to what the contents actually are.

Sets are usually denoted with an uppercase letter, e.g. the set $$\{1,2,3\}$$ may be declared as $$A=\{1,2,3\}$$, and then $$A$$ can be used to stand in for the set $$\{1,2,3\}$$.

There are two ways to specify a set: intensionally and extensionally.

Sets can be declared semantically using an **intensional** definition, as in: "A set containing odd natural numbers", denotable by $$\{x: x {\ is\ an\ odd\ natural\ number}\}$$.

An **extensional definition** is denoted by enclosing the list of members in curly brackets. **Roster**

Explicitly enumerating all elements of a set is known as **roster**, e.g. $$\{2,4,6\}$$. This allows for abbreviations in the form of ellipsis, e.g. $$\{1,2,3, \dots\}$$.


## Set-builder notation
Another form of extensional definition is **set-builder notation**, also called the **set comprehension**. It is a compact notation to describe a set, with the general sytax lookin like: $$X=\{exp:rule\}$$, with the colon, ":", (or sometimes, a pipe symbol, "|") standing for abstraction and being read as "_such that_".

For example: $$A=\{x:x \in \mathbb{Z}, 2^x<32\}$$    
is read as "_The set A contains all numbers of form two times x, such that x is an element of the integer set_". One of the common abbreviations is to immediately specify the "type" of $$x$$:     
$$A=\{x \in \mathbb{Z}: 2^x<32\}$$    

If the set comprehension contains more then one applicable rules, their relation can be spelled out in English or, more commonly, using logic symbols for negation (`¬`), conjunction (`∧`), disjunction (`∨` or `&`), implication (`→`) and bijection (`↔`). Generally, predicate logic is used in set theory.


## Definition by Predicate
Elements of a set can be specified by a predicate i.e. in terms of a property (or properties) they possess. Whether an object $$x$$ possesses a certain property $$P$$ is either true or false (in terms of classical logic), so it can be a subject of the propositional function $$P(x)$$.

A set can be specified by means of a propositional function, e.g. 
$$S=\{x:P(x)\}$$, meaning that $$S$$ is the set to whom each $$x$$ (object) belongs, if it has a certain property, i.e. for which $$P(x)$$ is true.


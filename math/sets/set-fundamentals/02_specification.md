# Set specification
math.st.sets.spec


- Extensional (roster) definition
- Intensional (semantic) definition
- Set-builder notation (set comprehension)
- Infinite set
- Finite set
- Terminal set member


## Specifing a set
The standard notation for specifing a set is to list the elements between the curlies: $$\{1,2,3\}$$

**A set is an unordered collection of distinct elements**     
so repetitions of elements or their order don't matter; e.g. the set $$\{1,2,3\}$$ is the same set as $$\{3,1,1,2,3\}$$.

**A set is defined by its elements**    
A set is uniquely determined by its elements, which is known as the **uniqueness of elements**, meaning that the only thing that defines *what a set really is* - are its elements.

By convention, sets are usually denoted by an uppercase letter, e.g. the set $$\{1,2,3\}$$ may be declared as $$A=\{1,2,3\}$$, and then $$A$$ can be used to stand in for the set $$\{1,2,3\}$$.


## Intensional and extensional specification
Sets can be declared semantically using an **intensional** definition, as in: "_A set containing odd natural numbers_", denotable by $$\{x: x {\ is\ an\ odd\ natural\ number}\}$$.

An **extensional definition** is denoted by enclosing the list of members in curly brackets.

Explicitly enumerating all elements of a set is also known as a **roster**, e.g. $$\{2,4,6\}$$. This allows for abbreviations in the form of ellipsis, e.g. $$\{1,2,3, \dots\}$$.


## Set-builder notation
Another form of extensional definition is **set-builder notation**, also called the **set comprehension**. It is a compact notation to describe a set, with the general sytax lookin like: $$X=\{exp:rule\}$$, with the colon, ":", (or sometimes, a pipe symbol, "|") standing for abstraction and being read as "_such that_".

For example: $$A=\{x:x \in \mathbb{Z}, 2^x<32\}$$    
is read as "_The set A contains all numbers of form two times x, such that x is an element of the integer set_". One of the common abbreviations is to immediately specify the "type" of $$x$$:     
$$A=\{x \in \mathbb{Z}: 2^x<32\}$$    

If the set comprehension contains more then one applicable rules, their relation can be spelled out in English or, more commonly, using logic symbols for negation (`¬`), conjunction (`∧`), disjunction (`∨` or `&`), implication (`→`) and bijection (`↔`). Generally, predicate logic is used as a language to describe sets.


## Definition by predicate

Elements of a set can be specified by a predicate i.e. in terms of a property (or properties) they possess.

Whether an object $$x$$ possesses a certain property $$P$$ is either true or false (in terms of classical logic), so it can be a subject of the propositional function $$P(x)$$.

A set can be specified by a predicate function; $$S=\{x:P(x)\}$$, means that $$S$$ is a set to whom each $$x$$, which possesses a certain property $$P$$, belongs; taht is, each $$x$$ for which $$P(x)$$ is true.

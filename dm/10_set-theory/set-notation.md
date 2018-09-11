Types of set definition (may be combined):
- intensional - semantic description
- extensional - element enumeration
- abbreviated - use of ellipsis
- set-builder notation


Sets are usually denoted with an uppercase letter, e.g. the set $$\{a,b,c\}$$ may be declared as $$A=\{a,b,c\}$$, and now we can use $$A$$ to stand in for the set $$\{a,b,c\}$$.

> Sets $$A$$ and $$B$$ are equal, $$A = B$$, if and only if they containt precisely the same elements.

**Intensional** is a semantic description of a set, e.g. "A set of odd positive integers".

**Extensional** definition is element enumeration i.e. listing all the elements of a set between the curly braces, e.g. $$\{2,4,6,8\}$$.

**Abbreviation** in the form of ellipsis can be used if a set contains a large number of elements, e.g. $$\{1,2,3,...,100\}$$.

__Set-builder notation__, also known as __set comprehension__, is a special notation used to describe sets that are too big or complex to list between braces. In general, the syntax is

$$$
X=\{exp:rule\}
$$$

For example, $$X=\{2n:n\in \mathbb{Z}\}$$ which builds the infinite set of even integers, which can be read as "_The set X is the set containing all numbers of form two times n, such that n is an element of the integer set_".

The domain of the elements can also be on the left side of colon, `:` which is read as "such that", e.g.

$$$
\{n \in \mathbb{N} : n^2 < 20\} = \{ 1, 2, 3, 4 \}
$$$

Here, we're descibing a set that contains numbers from the set of natural numbers _such that_ a number is in the set if it satisfies the condition that its square is less than 20.
# Set notation

Types of set definition (can be combined):
- intensional: semantic description
- extensional: element enumeration
- abbreviated: use of ellipsis
- set-builder notation

Sets are usually denoted with an uppercase letter, e.g. the set $$\{a,b,c\}$$ may be declared as $$A=\{a,b,c\}$$, and now we can use $$A$$ to stand in for the set $$\{a,b,c\}$$.

> Sets $$A$$ and $$B$$ are equal, $$A = B$$, iff they containt precisely the same elements.

**Intensional** is a semantic description of a set, e.g. "A set of odd positive integers".

**Extensional** definition is element enumeration i.e. listing all the elements of a set between the curly braces, e.g. $$\{2,4,6,8\}$$.

**Abbreviation** in the form of ellipsis can be used if a set contains a large number of elements, e.g. $$\{1,2,3,...,100\}$$.

__Set-builder notation__, also known as __set comprehension__, is a special notation used to describe sets that are too big or complex to list between braces. In general, the syntax is $$X=\{exp:rule\}$$


## Set-builder notation

Set-builder notation or __set comprehension__ is a compact notation used for describing sets.

The syntax looks the same, up to summary: $$\Omega=\{exp:rule\}$$, from there on, it's every author for himself. In fact, not even up to summary - the symbol for "_such that_" is randomly chosen between `:` and `|`.

Normal form is usually: $$X=\{x:x \in \mathbb{Z}, 2^x<32\}$$ i.e. "such that" x is an element of 
"such that x is", followed by the rules separated with a symbol for conjunction: commonly `,` or `∧`, `&` , or the word "_and_".

- $$X=\{x:x \in \mathbb{Z} ∧ 2^x<32\}$$
- $$X=\{2x:x \in \mathbb{Z}\}$$
- $$X=\{x \in \mathbb{Z}: 2^x<20\}$$


which can be read as "_The set X is the set containing all numbers of form two times x, such that x is an element of the integer set_".

The domain of the elements can also be on the left side of colon:
$$\{n \in \mathbb{N} : n^2 < 20\} = \{ 1, 2, 3, 4 \}$$
a set that contains numbers from the set of natural numbers _such that_ a number is in the set if it satisfies the condition that its square is less than 20.

The set of all even integers:
$$\{n\in \mathbb {Z} \mid (\exists k\in \mathbb {Z} )[n=2k]\ \}$$

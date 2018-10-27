# Boolean algebra

https://www.wikiwand.com/en/Boolean_algebra
https://www.wikiwand.com/en/Boolean_algebra_(structure)
https://www.wikiwand.com/en/Boolean_ring


Boolean algebra, introduced by George Boole in 1847, is a branch of algebra in which the values of the variables are the truth values true and false. It was fundamental in the development of digital electronics; it is available in all programming languages. It is also used in set theory and statistics.

The main operations of Boolean algebra:
- conjunction, $$\land$$ (analogous to )

Instead of algebra where the values of variables are numbers with basic operations of addition and multiplication, 
the main operations of Boolean algebra are the 
- conjunction and denoted as ∧, the 
- disjunction or denoted as ∨, and the 
- negation not denoted as ¬.

It is thus a formalism for describing logical relations in the same way that elementary algebra describes numeric relations.


Boolean algebra is a 6-tuple $$(A, \land, \lor, \lnot, \bot, \top)$$:
- set $$A$$ equipped with operations,
- **join** (and) binary operator, $$\land$$
- **meet** (or) binary operator, $$\lor$$
- **complement** (not) unary operator, $$\lnot$$
- **bottom** (least) element, $$\bot$$ (or $$0$$)
- **top** (greatest) element, $$\top$$ (or $$1$$)

such that $$\forall (a,b,c) \in A$$ these axioms hold:
- associativity : a ∨ (b ∨ c) = (a ∨ b) ∨ c, a ∧ (b ∧ c) = (a ∧ b) ∧ c
- commutativity : a ∨ b = b ∨ a, a ∧ b = b ∧ a
- distributivity: a ∨ (b ∧ c) = (a ∨ b) ∧ (a ∨ c), a ∧ (b∨c) = (a∧b) ∨ (a∧c)
- identity :      a ∨ 0 = a,  a ∧ 1 = a
- absorption:     a ∨ (a ∧ b) = a, a ∧ (a ∨ b) = a
- complement:     a ∨ ¬a = 1, a ∧ ¬a = 0


## NOT
¬¬a = a
¬0 = 1, ¬1 = 0

## AND
a ∧ a = a
a ∧¬a = 0
a ∧ 1 = a
a ∧ 0 = 0

# OR
a ∨ a = a
a ∨¬a = 1
a ∨ 1 = 1
a ∨ 0 = a


**and, or**    
a ∧ a = a ∨ a = a
a ∧ 1 = a ∨ 1 = a
a ∧ 0 = a ∨ 0 = 0

**de morgan**    
¬(a ∧ b) = ¬a ∨ ¬b
¬(a ∨ b) = ¬a ∧ ¬b


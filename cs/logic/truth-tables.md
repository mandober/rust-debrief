# Truth table

A truth table is a mathematical table used in logic to presents the values of logical expressions for each of their arguments. Truth tables can show whether a propositional expression is logically valid for all legitimate input values.

A truth table has one column for each input variable and one final column showing all of the possible results of the logical operation that the table represents, although is not uncommon to save space by merging more operations into a single truth table.

Each row of the truth table contains one possible configuration of the input variables and the result of the operation for those values.


## Unary operations
Unary operations have 2 rows and 4 configurations.

There are 4 unary operations:
- Logical false (unary falsum): the output value is always false
- Logical negation (unary negation): the output is the opposite of input.
- Logical identity (unary identity): the output value is the same as input.
- Logical true: the output value is always true

p  |⊥|¬|i|⊤
---|-|-|-|-
`1`|0|0|1|1
`0`|0|1|0|1


## Binary operations
Binary operations have 4 rows and 16 configurations

There are 16 possible truth functions of two binary variables, p and q.

p|q|⊥| ↓ |¬← |¬p|¬→ |¬q|^|↑  |∧  |↔|q|→  |p|←  |∨  |⊤
-|-|-|---|---|--|---|--|-|---|---|-|-|---|-|---|---|-
1|1|0| 0 |0  |0 |0  |0 |0|`0`|`1`|1|1|1  |1|1  |1  |1
1|0|0| 0 |0  |0 |`1`|1 |1|1  |0  |0|0|`0`|1|1  |1  |1
0|1|0| 0 |`1`|1 |0  |0 |1|1  |0  |0|1|1  |0|`0`|1  |1
0|0|0|`1`|0  |1 |0  |1 |0|1  |0  |1|0|1  |0|1  |`0`|1






---


0|1  |2  |3 |4  |5 |6|7  |8  |9|a|b  |c|d  |e  |f
-|---|---|--|---|--|-|---|---|-|-|---|-|---|---|-
0|0  |0  |0 |0  |0 |0|`0`|`1`|1|1|1  |1|1  |1  |1
0|0  |0  |0 |`1`|1 |1|1  |0  |0|0|`0`|1|1  |1  |1
0|0  |`1`|1 |0  |0 |1|1  |0  |0|1|1  |0|`0`|1  |1
0|`1`|0  |1 |0  |1 |0|1  |0  |1|0|1  |0|1  |`0`|1
⊥|↓  |¬← |¬p|¬→ |¬q|^|↑  |∧  |↔|q|→  |p|←  |∨  |⊤


- 1: XNOR ↓ : ¬(p ∨ q)
- 7: NAND ↑ : ¬(p ∨ q)


NOT   = ¬
AND   = ∧
NAND  = ↑ = ¬(p ∧ q)
OR    = ∨
NOR   = ↓
IMPLY = →
IFF   = ↔ =  (p → q) ∧ (q → p)
XOR   = ^ =  (p ∨ q) ∧ ¬(p ∧ q) = ⊕
XNOR  = 


∨ ∧ ¬ → ← ↔ 
⊕ ↓ ↑



p|q|r|p∧q|p∨q|p→q|
-|-|-|-|-|-|
1|1|1|1|0|0|
1|1|0| | |0|
1|0|1|0|1|0|
1|0|0| | |0|
0|1|1|0|1|0|
0|1|0| | |0|
0|0|1|0|1|0|
0|0|0| | |0|

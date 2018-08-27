> An equivalence relation is a binary relation that is at the same time reflexive, symmetric and transitive.

As a consequence of these properties an equivalence relation provides a _partition_ of a set into __equivalence classes__.

Notation: two elements `a` and `b` of a set are equivalent with respect to an equivalence relation `R`: `a ~ b` and `a ≡ b`, which are used when `R` is the obvious relation being referenced, and variations of `a ~R b`, `a ≡R b` or `a R b` otherwise.

Definition: A given binary relation `~` on a set `X` is said to be an equivalence relation if and only if it is reflexive, symmetric and transitive. 

That is, for all `a`, `b` and `c` in `X`:
- Reflexivity: `a ~ a`
- Symmetry: `a ~ b` if and only if `b ~ a`
- Transitivity: if `a ~ b` and `b ~ c` then `a ~ c`
`X` together with the relation `~` is called a setoid. The equivalence class of `a` under `~`, denoted `[a]`, is defined as: 
`[a] = {b ∈ X | a ~ b }`



For example:    
To see that the the "equals", `=`, operator is in equivalence relation with respect to the domain of natural numbers, we have to show that equals operator is reflexive, symmetric and transitive.

**Reflexivity**:
we have to show that $$x=x$$.
```
x = x
0 = 0
5 = 5
  T
```
this always holds, so the = operator is reflexive.


**Symmetry**:
we have to show that, if $$x=y$$ then $$y=x$$, or, formally $$(x=y) \to (y=x)$$
There are two cases here $$x\not = y$$ and $$x=y$$

In case $$x\not = y$$:
```
(x=y) → (y=x)
(3=5) → (5=3)
  F   →   F
      T
```

In case $$x\not = y$$:
```
(x=y) → (y=x)
(0=0) → (0=0)
  T   →   T
      T
```

since the overall truth value of both cases are true, the symmetry holds.


Note: the truth table of the implication operator, p → q:
p|q|p→q
-|-|---
1|1|1
__1__|__0__|__0__
0|1|1
0|0|1


**Transitivity**:
we have to show that, if $$x=y$$ and $$y=z$$ then $$x=z$$,
formally $$(x=y) \land (y=z) \to (x=z)$$


Case 1: x=1, y=3, z=5: $$x \not = y \not = z$$
```
(x=y) ∧ (y=z) → (x=z)
(1=3) ∧ (3=5) → (1=5)
( F   ∧   F ) →   F
      F       →   F
              T
```


Case 2: x=1, y=3, z=1: $$x = z$$
```
(x=y) ∧ (y=z) → (x=z)
(1=3) ∧ (3=1) → (1=1)
( F   ∧   F ) →   T
      F       →   T
              T
```


Case 3: x=1, y=1, z=5: $$x = y$$
```
(x=y) ∧ (y=z) → (x=z)
(1=1) ∧ (1=5) → (1=5)
( T   ∧   F ) →   F
      F       →   F
              T
```


Case 4: x=1, y=3, z=3: $$y = z$$
```
(x=y) ∧ (y=z) → (x=z)
(1=3) ∧ (3=3) → (1=3)
( F   ∧   T ) →   F
      F       →   F
              F
```


Case 5: x=1, y=1, z=1: $$x = y = z$$
```
(x=y) ∧ (y=z) → (x=z)
(1=1) ∧ (1=1) → (1=1)
( T   ∧   T ) →   T
      T       →   T
              T
```

in all cases, the overall truth value is true, so the transivity holds.


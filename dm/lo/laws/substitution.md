## Substitution

https://www.wikiwand.com/en/Substitution_(logic)

Substitution is a fundamental concept in logic.

A substitution is a syntactic transformation on formal expressions.

To apply a substitution to an expression means to consistently replace its variable, or placeholder, symbols by other expressions.

The resulting expression is called a substitution instance, or short instance, of the original expression.


## Propositional logic

Definition

Where ψ and φ represent formulas of propositional logic, ψ is a substitution instance of φ if and only if ψ may be obtained from φ by substituting formulas for symbols in φ, replacing each occurrence of the same symbol by an occurrence of the same formula. For example:

(R → S) & (T → S)
is a substitution instance of:

P & Q
and

(A ↔ A) ↔ (A ↔ A)
is a substitution instance of:

(A ↔ A)
In some deduction systems for propositional logic, a new expression (a proposition) may be entered on a line of a derivation if it is a substitution instance of a previous line of the derivation (Hunter 1971, p. 118). This is how new lines are introduced in some axiomatic systems. In systems that use rules of transformation, a rule may include the use of a substitution instance for the purpose of introducing certain variables into a derivation.

In first-order logic, every closed propositional formula that can be derived from an open propositional formula a by substitution is said to be a substitution instance of a. If a is a closed propositional formula we count a itself as its only substitution instance.

Tautologies
A propositional formula is a tautology if it is true under every valuation (or interpretation) of its predicate symbols. If Φ is a tautology, and Θ is a substitution instance of Φ, then Θ is again a tautology. This fact implies the soundness of the deduction rule described in the previous section.

First-order logic
In first-order logic, a substitution is a total mapping σ: V → T from variables to terms; many,[1]:73[2]:445 but not all[3]:250 authors additionally require σ(x) = x for all but finitely many variables x. The notation { x1 ↦ t1, ..., xk ↦ tk }[note 1] refers to a substitution mapping each variable xi to the corresponding term ti, for i=1,...,k, and every other variable to itself; the xi must be pairwise distinct. Applying that substitution to a term t is written in postfix notation as t { x1 ↦ t1, ..., xk ↦ tk }; it means to (simultaneously) replace every occurrence of each xi in t by ti.[note 2] The result tσ of applying a substitution σ to a term t is called an instance of that term t. For example, applying the substitution { x ↦ z, z ↦ h(a,y) } to the term

f(	z	, a, g(	x	), y)  	yields
f(	h(a,y)	, a, g(	z	), y)	.
The domain dom(σ) of a substitution σ is commonly defined as the set of variables actually replaced, i.e. dom(σ) = { x ∈ V | xσ ≠ x }. A substitution is called a ground substitution if it maps all variables of its domain to ground, i.e. variable-free, terms. The substitution instance tσ of a ground substitution is a ground term if all of t's variables are in σ's domain, i.e. if vars(t) ⊆ dom(σ). A substitution σ is called a linear substitution if tσ is a linear term for some (and hence every) linear term t containing precisely the variables of σ's domain, i.e. with vars(t) = dom(σ). A substitution σ is called a flat substitution if xσ is a variable for every variable x. A substitution σ is called a renaming substitution if it is a permutation on the set of all variables. Like every permutation, a renaming substitution σ always has an inverse substitution σ−1, such that tσσ−1 = t = tσ−1σ for every term t. However, it is not possible to define an inverse for an arbitrary substitution.

For example, { x ↦ 2, y ↦ 3+4 } is a ground substitution, { x ↦ x1, y ↦ y2+4 } is non-ground and non-flat, but linear, { x ↦ y2, y ↦ y2+4 } is non-linear and non-flat, { x ↦ y2, y ↦ y2 } is flat, but non-linear, { x ↦ x1, y ↦ y2 } is both linear and flat, but not a renaming, since is maps both y and y2 to y2; each of these substitutions has the set {x,y} as its domain. An example for a renaming substitution is { x ↦ x1, x1 ↦ y, y ↦ y2, y2 ↦ x }, it has the inverse { x ↦ y2, y2 ↦ y, y ↦ x1, x1 ↦ x }. The flat substitution { x ↦ z, y ↦ z } cannot have an inverse, since e.g. (x+y) { x ↦ z, y ↦ z } = z+z, and the latter term cannot be transformed back to x+y, as the information about the origin a z stems from is lost. The ground substitution { x ↦ 2 } cannot have an inverse due to a similar loss of origin information e.g. in (x+2) { x ↦ 2 } = 2+2, even if replacing constants by variables was allowed by some fictitious kind of "generalized substitutions".

Two substitutions are considered equal if they map each variable to structurally equal result terms, formally: σ = τ if xσ = xτ for each variable x ∈ V. The composition of two substitutions σ = { x1 ↦ t1, ..., xk ↦ tk } and τ = { y1 ↦ u1, ..., yl ↦ ul } is obtained by removing from the substitution { x1 ↦ t1τ, ..., xk ↦ tkτ, y1 ↦ u1, ..., yl ↦ ul } those pairs yi ↦ ui for which yi ∈ { x1, ..., xk }. The composition of σ and τ is denoted by στ. Composition is an associative operation, and is compatible with substitution application, i.e. (ρσ)τ = ρ(στ), and (tσ)τ = t(στ), respectively, for every substitutions ρ, σ, τ, and every term t. The identity substitution, which maps every variable to itself, is the neutral element of substitution composition. A substitution σ is called idempotent if σσ = σ, and hence tσσ = tσ for every term t. The substitution { x1 ↦ t1, ..., xk ↦ tk } is idempotent if and only if none of the variables xi occurs in any ti. Substitution composition is not commutative, that is, στ may be different from τσ, even if σ and τ are idempotent.[1]:73-74[2]:445-446

For example, { x ↦ 2, y ↦ 3+4 } is equal to { y ↦ 3+4, x ↦ 2 }, but different from { x ↦ 2, y ↦ 7 }. The substitution { x ↦ y+y } is idempotent, e.g. ((x+y) {x↦y+y}) {x↦y+y} = ((y+y)+y) {x↦y+y} = (y+y)+y, while the substitution { x ↦ x+y } is non-idempotent, e.g. ((x+y) {x↦x+y}) {x↦x+y} = ((x+y)+y) {x↦x+y} = ((x+y)+y)+y. An example for non-commuting substitutions is { x ↦ y } { y ↦ z } = { x ↦ z, y ↦ z }, but { y ↦ z} { x ↦ y} = { x ↦ y, y ↦ z }.

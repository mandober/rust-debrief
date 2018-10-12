# Predicate logic


**Syllogistic logic** deals with arguments whose fundamental component is a term; validity of an argument depends on the arrangement of terms within the propositions. **Propositional logic** deals with arguments whose fundamental component is a statement; validity of an argument depends on the arrangement of propositions - an argument is valid iff it takes a form which makes it impossible for the premises to be true and the conclusion, nevertheless, false.

However, there's another kind of argument, featuring aspects of both, syllogistic and sentential logic, but which neither can properly described. We need another kind of system, called **predicate logic**, to describe such arguments. The validity of such an argument depends on both, the arrangement of the terms as well as the propositions.

Predicate logic is an extension of propositional logic - a proposition is a nullary predicate. In a sense, predicate logic uses propositional logic as its foundation - everything from propositional is available in predicate logic, with addition of some new concepts, making the predicate logic capable of describing all of math.

Predicate logic does not designates any specific logic, but it is a general term for many logics that use predicates.


## Orders

There are predicate logics of first-order, second-order and higer-order.

**First-order logic (FOL)** has quantifiers ranging over individuals. FOL allows variables to range over atomic symbols in the domain. It doesn't allow variables to be bound to predicate symbols, however.

In FOL, variables can only appear inside a predicate i.e. you can quantify over variables, but not over the predicates themselves.

**Second-order logic** allows this, adding quantifiers that range over predicates as well, so one can write sentences such as: $$\forall P.P(s)$$. It also quantifies over sets. Second order logic cannot be reduced to first-order logic.

In second-order logic, you can also quantify over predicates, e.g. 
$$\forall P\ \forall x . (P(x)\lor\lnot P(x))$$ is true: "_for every predicate P, either P(x) or not P(x) is true, regardless of what x is_".

**Third-order logic** also quantifies over sets of sets, and so on. A higher order logic allows predicates to accept arguments which are themselves predicates.

**Higher-order logic (HOL)** is the union of all $$n$$-order logics i.e. it allows quantification over sets that are nested arbitrarily deeply. These logics have more expressive semantics, but their model-theoretic properties are less well-behaved than those of FOL.

HOL's predicates have other predicates or functions as arguments, and one or both of predicate quantifiers or function quantifiers are permitted. In FOLs, predicates are often associated with sets; in interpreted HOLs, predicates may be interpreted as sets of sets.

HOL commonly names higher-order simple predicate logic; with "simple" indicating that the underlying type theory is simple (and not, e.g. polymorphic or dependent).


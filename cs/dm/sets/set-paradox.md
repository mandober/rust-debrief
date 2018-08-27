
Sets may be elements of other sets, but can a set be a member of itself? For instance, since all sets form a collection of objects, can we collect them into a single set - the set of all sets? And it, being a set, would be an element of the set of all sets.

Russell's paradox arises when we consider the property of not having itself
as an element.

For example, set of natural numbers is not in the set of natural numbers, $$\mathbb{N}\notin \mathbb{N}$$, since it is a set, not a natural number. Powerset of the set of real numbers is not an element of itself, $$\mathfrak{P}(\mathbb{R})\notin \mathfrak{P}(\mathbb{R})$$, since it is a set of sets of real numbers, not a set of real numbers.

What if we suppose that there is a set of all sets that do not have themselves as an element? Does $$R=\{x : x\notin x\}$$ exist?

If R exists, it makes sense to ask if $$R\in R$$ or not - it must be either $$R\in R$$ or $$R\notin R$$.

Suppose the former is true, i.e. $$R\in R$$ was defined as the set of all sets that are not elements of themselves, and so if $$R\in R$$, then $$R$$ does not have this defining property of $$R$$. But only sets that have this property are in $$R$$, hence, $$R$$ cannot be an element of $$R$$, i.e. $$R\notin R$$. But $$R$$ can't both be and not be an element of $$R$$, so we have a contradiction.

Since the assumption that $$R\in R$$ leads to a contradiction, we have $$R\notin R$$. But this also leads to a contradiction! If $$R\notin R$$, it does have the defining property of $$R$$, and so it would be an element of $$R$$ just like all the other non-self-containing sets. And again, it can't both not be and be an element of $$R$$.

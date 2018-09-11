# Permutations

When we are selecting objects and **the order does matter**, we are dealing with permutations.

If we have a set of $$n$$ objects and we want to choose $$r$$ objects from the set in order, we write $$P(n,r)$$ or $$_nP_r$$.
​
To calculate $$P(n,r)$$, we find the factorial $$n!$$ i.e. the number of ways to line up all $$n$$ objects. Then we divide it by $$(n-r)!$$ to cancel out the $$(n−r)$$ items that we do not wish to line up.

Given $$n$$ distinct objects, the number of ways to select $$r$$ objects from the set in order is $$P(n,r) = \frac{n!}{(n-r)!}$$

Example: $$n=6,\ \ r=3$$

\[
\frac{n!}{(n-r)!} = \frac{6!}{3!} = \frac{6·5·4· \not 3!}{\not 3!} = 6·5·4 = 120
\]



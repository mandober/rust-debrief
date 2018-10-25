# Combinations

**Combinatorics** studies the problems concerning counting and ordering, such as partitioning and enumerations.

When we are selecting objects and **the order does not matter**, we are dealing with combinations.

If we have a set of $$n$$ objects and we want to choose $$r$$ objects from the set, and the order does not matter, we write $$C(n,r)$$ or $$_nC_r$$.
​
Given $$n$$ distinct objects, the number of ways to select $$r$$ objects from the set:

\[
C(n,r) = \frac{n!}{r!(n-r)!}
\]

For example: $$n=4,\ \ r=2$$

\[
\frac{n!}{r!(n-r)!} 
= \frac{4!}{2!(4-2)!} 
= \frac{4!}{2!·2!}
= \frac{4·3·\not 2!}{2!·\not 2!} 
= 6
\]

---

\[
{n \choose k} = {n \choose n - k}
\]

Example:
How may distinct pairs can be made with 4 elements?

\[
{n \choose k} = 
{4 \choose 2} = 
\frac{4!}{2!} = 
\frac{4·3·2!}{2!} = 
6
\]

Check:
X={a,b,c,d}     
n=4     
6 pairs: ab, ac, ad, bc, bd, cd

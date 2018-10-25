# Partitioning

Partition of a set, S, is a collection of n disjoint subsets, P1,P2,…Pn, that satisfies the following three conditions:

- Pi does not contain the empty set.
  `[Pi≠{∅} for all 0<i≤n]`
- The union of the subsets must equal the entire original set.
  `[P1∪P2∪⋯∪Pn=S]`
- The intersection of any two distinct sets is empty.
  `[Pa∩Pb={∅}, for a≠b where n≥a,b≥0]`

Example:
Let S={a,b,c,d,e,f,g,h}
One probable partitioning scheme: {a},{b,c,d},{e,f,g,h}
Another probable partitioning scheme: {a,b},{c,d},{e,f,g,h}

**Bell number** signifies the number of ways to partition a set; it is denoted by $$B_n$$, where $$n$$ is the cardinality of the set. For example, let $$S=\{1,2,3\},\ \ n=|S|=3$$.

The partitions:
1. $$\{\},\{1,2,3\}$$
2. $$\{1\},\{2,3\}$$
3. $$\{1,2\},\{3\}$$
4. $$\{1,3\},\{2\}$$
5. $$\{1\},\{2\},\{3\}$$

so, $$B_3=5$$




Set                     | Notation
------------------------|---------
universe                | $$\mathcal{U}$$
set                     | $$A$$
class                   | $$A$$
element                 | $$a$$
urelement               | $$a$$ (element that is not a set)
membership              | $$a\in A$$
empty set               | $$\varnothing$$
disjoint union          | $$A\cap B = \varnothing$$
powerset                | $$\mathcal{P}(A)$$
Cartesian product       | $$A\times B$$
set of pairs            | $$A\times B$$
set of functions        | $$A\to B$$
relation                | $$R \subseteq A\times B$$
relation                | $$R \in P(A\times B)$$
union                   | $$A \cup B$$
intersection            | $$A \cap B$$
complement              | $$\bar A=\mathcal{U}\cap A$$
family of sets          | $$B(x)$$
family of elements      | $$b(x):B(x)$$

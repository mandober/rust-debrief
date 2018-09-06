https://www.wikiwand.com/en/Set_(mathematics)
https://www.wikiwand.com/en/Portal:Set_theory
https://www.tutorialspoint.com/discrete_mathematics/discrete_mathematics_quick_guide.htm

---

$$\quad \mathbb{N}\subseteq \mathbb{Z}\subseteq \mathbb{Q}\subseteq \mathbb{R}$$

Even though they are all infinite sets, it turns out that $$\mathbb{N}, \mathbb{Z}, \mathbb{Q}$$ have the same number of elements, while $$\mathbb{R}$$ has a whole lotta more. The former three are __enumerable__ and infinite whereas the latter R is __non-enumerable__ and infinite.

---


## Types of Sets
- Universal set
- Singleton (unit) set
- Equal set
- Equivalent set
- Overlapping set
- Disjoint set
- Powerset


## Set Operations
- Union
- Intersection
- Difference
- Complement
- Cartesian (cross) product
- Partitioning
- Bell Numbers


## Partitioning

Partition of a set, say S, is a collection of n disjoint subsets, say P1,P2,…Pn that satisfies the following three conditions −

Pi does not contain the empty set.

[Pi≠{∅} for all 0<i≤n]
The union of the subsets must equal the entire original set.

[P1∪P2∪⋯∪Pn=S]
The intersection of any two distinct sets is empty.

[Pa∩Pb={∅}, for a≠b where n≥a,b≥0]
Example

Let S={a,b,c,d,e,f,g,h}
One probable partitioning is {a},{b,c,d},{e,f,g,h}
Another probable partitioning is {a,b},{c,d},{e,f,g,h}


**Bell number** signifies the number of ways to partition a set; it is denoted by $$Bn$$, where $$n$$ is the cardinality of the set. For example, let $$S=\{1,2,3\},\ \ n=|S|=3$$.

The partitions:

1. $$\{\},\{1,2,3\}$$
2. $$\{1\},\{2,3\}$$
3. $$\{1,2\},\{3\}$$
4. $$\{1,3\},\{2\}$$
5. $$\{1\},\{2\},\{3\}$$

so, $$B3=5$$



#### The Lorenz Equations

$$$
\begin{align}
  \dot{x}  & = \sigma(y-x)\\
  \dot{y}  & = \rho x - y - xz \\
  \dot{z}  & = -\beta z + xy
\end{align}
$$$


#### The Cauchy-Schwarz Inequality

\[
\left( \sum_{k=1}^n a_k b_k \right)^{\!\!2} \leq
\left( \sum_{k=1}^n a_k^2 \right) \left( \sum_{k=1}^n b_k^2 \right)
\]

#### A Cross Product Formula

\[
\mathbf{V}_1 \times \mathbf{V}_2 =
\begin{vmatrix}
  \mathbf{i}                    & \mathbf{j}                    & \mathbf{k} \\
  \frac{\partial X}{\partial u} & \frac{\partial Y}{\partial u} & 0 \\
  \frac{\partial X}{\partial v} & \frac{\partial Y}{\partial v} & 0 \\
\end{vmatrix}
\]


#### The probability of getting \(k\) heads when flipping \(n\) coins is

\[P(E) = {n \choose k} p^k (1-p)^{ n-k} \]


#### An Identity of Ramanujan

\[
   \frac{1}{(\sqrt{\phi \sqrt{5}}-\phi) e^{\frac25 \pi}} =
     1+\frac{e^{-2\pi}} {1+\frac{e^{-4\pi}} {1+\frac{e^{-6\pi}}
      {1+\frac{e^{-8\pi}} {1+\ldots} } } }
\]


#### A Rogers-Ramanujan Identity

<p>\[
  1 +  \frac{q^2}{(1-q)}+\frac{q^6}{(1-q)(1-q^2)}+\cdots =
    \prod_{j=0}^{\infty}\frac{1}{(1-q^{5j+2})(1-q^{5j+3})},
     \quad\quad \text{for $|q|&lt;1$}.
\]</p>


#### Maxwell's Equations

$$$
\begin{align}
  \nabla \times \vec{\mathbf{B}} -\, \frac1c\, \frac{\partial\vec{\mathbf{E}}}{\partial t} &amp; = \frac{4\pi}{c}\vec{\mathbf{j}} \\
  \nabla \cdot \vec{\mathbf{E}} &amp; = 4 \pi \rho \\
  \nabla \times \vec{\mathbf{E}}\, +\, \frac1c\, \frac{\partial\vec{\mathbf{B}}}{\partial t} &amp; = \vec{\mathbf{0}} \\
  \nabla \cdot \vec{\mathbf{B}} &amp; = 0
\end{align}
$$$

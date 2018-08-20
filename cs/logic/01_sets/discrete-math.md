# Discrete Mathematics

https://www.tutorialspoint.com/discrete_mathematics/discrete_mathematics_quick_guide.htm

Mathematics can be broadly classified into two categories:
1. Continuous Mathematics
2. Discrete Mathematics

__Continuous Mathematics__
is based upon continuous number line or the real numbers. It is characterized by the fact that between any two numbers, there is an infinite set of numbers. For example, a function in continuous mathematics can be plotted in a smooth curve without breaks.

__Discrete Mathematics__
involves itself with distinct values; i.e. between any two points, there is a countable number of points. For example, if we have a finite set of objects, a function can be defined as a list of ordered pairs having these objects, and it can be presented as a complete list of ordered pairs.

The topics of Discrete Mathematics:
- Sets, Relations and Functions
- Mathematical Logic
- Group theory
- Counting Theory
- Probability
- Mathematical Induction and Recurrence Relations
- Graph Theory
- Trees
- Boolean Algebra


## Sets

A set is a collection of definite and distinguishable objects, selected by a specific rule or by a desciption of their common property.

{% hint style='info' %}
A set is an unordered collection of distinct elements.
{% endhint %}


## Set Builder Notation

The set is defined by specifying a property that elements of the set have in common. The set is described as A={x:p(x)}

Example 1 − The set {a,e,i,o,u} is written as −
A={x:x is a vowel in English alphabet}

Example 2 − The set {1,3,5,7,9} is written as −
B={x:1≤x<10 and (x%2)≠0}

If an element x is a member of any set S, it is denoted by x∈S and if an element y is not a member of set S, it is denoted by y∉S.


## Cardinality

Cardinality of a set S, denoted by |S|, is the number of elements of the set. The number is also referred as the cardinal number. If a set has an infinite number of elements, its cardinality is ∞.
|{1,2,3,4,5,…}| = ∞

|X|=|Y| denotes two sets X and Y having same cardinality.
It occurs when the number of elements in X is exactly equal to the number of elements in Y. In this case, there exists a __bijective__ function `f` from X to Y.

|X|≤|Y| denotes that set X’s cardinality is less than or equal to set Y’s cardinality. It occurs when number of elements in X is less than or equal to that of Y. Here, there exists an __injective__ function ‘f’ from X to Y.

|X|<|Y| denotes that set X’s cardinality is less than set Y’s cardinality. It occurs when number of elements in X is less than that of Y. Here, the function ‘f’ from X to Y is _injective function but not bijective_.

If |X|≤|Y| and |X|≥|Y| then |X|=|Y|. The sets X and Y are commonly referred as _equivalent_ sets.


## Types of Sets

Sets can be classified into many types, including: finite, infinite, universal, singleton, empty set.

__Proper Subset__
A Set X is a proper subset of set Y (Written as X⊂Y) if every element of X is an element of set Y and |X|<|Y|.

X={1,2,3,4,5,6} and Y={1,2}.
Here set Y⊂X since all elements in Y are contained in X too and X has at least one element is more than set Y.

__Universal Set__
It is a collection of all elements in a particular context or application. All the sets in that context or application are essentially subsets of this universal set. Universal sets are represented as U.

Example − We may define U as the set of all animals on earth. In this case, set of all mammals is a subset of U, set of all fishes is a subset of U, set of all insects is a subset of U, and so on.

__Singleton Set__ or __Unit Set__
Singleton set or unit set contains only one element. A singleton set is denoted by {s}.
Example − S={x|x∈N, 7<x<9} = {8}

__Equal Set__
If two sets contain the same elements they are said to be equal.
Example − If A={1,2,6} and B={6,1,2}, they are equal as every element of set A is an element of set B and every element of set B is an element of set A.

__Equivalent Set__
If the cardinalities of two sets are same, they are called equivalent sets.
Example − If A={1,2,6} and B={16,17,22}, they are equivalent as cardinality of A is equal to the cardinality of B. i.e. |A|=|B|=3

__Overlapping Set__
Two sets that have at least one common element are called overlapping sets.
In case of overlapping sets −
n(A∪B)=n(A)+n(B)−n(A∩B)
n(A∪B)=n(A−B)+n(B−A)+n(A∩B)
n(A)=n(A−B)+n(A∩B)
n(B)=n(B−A)+n(A∩B)
Example − Let, A={1,2,6} and B={6,12,42}. There is a common element ‘6’, hence these sets are overlapping sets.

__Disjoint Set__
Two sets A and B are called disjoint sets if they do not have even one element in common. Therefore, disjoint sets have the following properties −
n(A∩B)=∅
n(A∪B)=n(A)+n(B)
Example − Let, A={1,2,6} and B={7,9,14}, there is not a single common element, hence these sets are overlapping sets.

__Power set__
of a set S is the set of all subsets of S including the empty set. The cardinality of a power set of a set S of cardinality n is 2^n


## Set Operations
- Union
- Intersection
- Difference
- Complement
- Cartesian (cross) product


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


## Bell Numbers
Bell numbers give the count of the number of ways to partition a set. They are denoted by Bn where n is the cardinality of the set.

Example −

Let S={1,2,3}, n=|S|=3
The alternate partitions are −

1. ∅,{1,2,3}

2. {1},{2,3}

3. {1,2},{3}

4. {1,3},{2}

5. {1},{2},{3}

Hence B3=5



---

# Relations

Relations may exist between objects of the same set or between objects of two or more sets.































































































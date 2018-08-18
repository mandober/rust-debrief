# Sets

- Sets are denoted with a capital letters: $$A=\{a,b,c\}$$
- If $$a$$ is an element of some set $$B$$, $$a\in B$$, if not $$a\notin B$$
- Order or repetition of elements is unimportant: $$\{\{a\},b,\varnothing\}=\{b, \{a\},\{\},b,\varnothing,\{\}\}$$
- $$x \in A \land A \in \mathscr{M} \not \rightarrow x \in A$$.
- An empty set, $$\varnothing$$ or $$\{\}$$, contains no elements

- Two sets are equal if they contain exactly the same elements.

- $$\mathbb{N}$$: The set of natural numbers (positive integers)



A set, $$\{a,b\}$$, is distinguished from the ordered pair, $$(a,b)$$

- $$n$$-element set is different from ordered $$n$$-tuple: $$\{x_1, x_2,\dots,x_n\} \neq (x_1,\dots, x_n)$$

- Two ordered pairs are equal __iff__: $$(a,b) = (x,y) \iff a=x \land b = y$$

The order of the elements or their repetition is of no consequence to sets, $$\{a,b,\{\}\}=\{b,\{\},b,\varnothing,a\}$$.


$$\{\varnothing\}=\{\{\}\}$$





Subset and powerset:
- if all elements of a set $$A$$ are also elements of a set $$B$$, then $$A$$ is a __subset__ of $$B$$: $$A \subseteq B$$ and $$B$$ is a __superset__ of $$A$$: $$B \supseteq A$$.
- a set $$B$$ is __equal__ to $$A$$ if the two sets consist of exactly the same elements: $$A = B \iff A \subseteq B \land B \subseteq A$$.
- if $$A \subseteq B$$ but $$B \neq A$$ then $$A$$ is a __proper subset__ of $$B$$: $$A \subset B$$.
- if a set $$P$$ contains all possible subsets of a set $$A$$, then $$P$$ is a __powerset__ of $$A$$, denoted as $$P(A)$$.
- e.g. if $$A=\{a,b\}$$, then $$P=\{\{a\},\{b\},\{ab\},\{\varnothing\}\}$$
- here, the cardinality (number of elements) of $$P$$ is 4, denoted as $$|P|=4$$


Properties of sets: $$\forall A,B,C$$
- $$A \subseteq A$$ (reflexivity)
- $$A \subseteq B \land B \subseteq C \rightarrow A \subseteq C$$  (transitivity)
- $$A \subseteq B \land B \subseteq A \rightarrow A=B$$ (anti-symmetry, axiom of extensionality)
- $$\varnothing \subseteq A$$
- $$A\subseteq \varnothing \rightarrow A=\varnothing$$


# Introduction to Sets

- sets
- set elements
- infinite and finite sets
- set equality
- set notation
- special sets


The theory of sets, initiated by Georg Cantor (1845–1918), constitutes the basis of almost all modern mathematics. A set is often described as a __collection__ (aggregate, class, totality, family) of objects of any specified kind, but the set concept is one of the basics so the term _set_ must be accepted as a primitive notion, without definition. The objects belonging to a set are called its __elements__ or __members__.

__Infinite__ set has infinitely many elements; otherwise it is a __finite__ set. Infinite sets: $$\{1,2,3,\dots\}$$, or $$\{\dots,1,2,3,\dots\}$$. A sets with terminal member: $$\{1,2,3, \dots,100\}$$, or $$\{1,2,\dots,n\}$$

Two sets are __equal__ if they contain exactly the same elements. The order of the elements or their repetition is of no consequence to sets, $$\{a,b,\{\}\}=\{b,\{\},b,\varnothing,a\}$$

Sets are usually denoted with uppercase letters: the set $$\{2,4,6,8\}$$ may be declared as $$A=\{2,4,6,8\}$$ and then use $$A$$ to stand for $$\{2,4,6,8\}$$. To express that $$2$$ is an element of the set $$A$$, we write $$2 \in A$$. Expressions like $$2,4,6 \in A$$ indicate that several things are in a set. If an element is not in a set: $$3 \notin A$$. A set of one element is not the same as that element alone: $$p \neq \{p\}$$.

Once a set has been formed, it is a __new entity__, different from any of its elements. A newly formed set may be an element of some other set. Sets whose elements are other sets are __collections of sets__. If an element $$a$$ is in the set $$A$$ and set $$A$$ is in the set $$\mathscr{M}$$, that doesn't imply the element $$a$$ is in the set $$\mathscr{M}$$: $$x \in A \land A \in \mathscr{M} \not \rightarrow x \in A$$.

We are mainly concerned with sets whose elements are mathematical entities, such as numbers, points, functions, etc., but sets need not have just numbers as elements. Sets can even have other sets as elements.

Some sets are so prevalent they have a __special symbol__ assigned for them:
- The set of boolean (truth) values: $$\mathbb{B}=\{\bot,\top\}=\{F,T\}=\{0,1\}$$
- The set of natural numbers, either $$\mathbb{N}=\{1,2,3,\dots\}$$ or $$\mathbb{N}=\{0,1,2,3,\dots\}$$
- The set of integers, $$\mathbb{Z}=\{-2,-1,0,1,2,\dots\}$$; $$\mathbb{Z^+}=\{1,2,3,\dots\}$$
- The set of rational numbers: $$\mathbb{Q}=\{p/q : p \in\mathbb{Z}, q\in \mathbb{N}\}$$
- The set of real numbers, $$\mathbb{R}$$
- The set of complex numbers: $$\mathbb{C}=\{a,b\,i:a,b\in\mathbb{R}\}$$

$$$$
\mathbb{N}\subseteq \mathbb{Z}\subseteq \mathbb{Q}\subseteq \mathbb{R}
$$$$

The __empty set__ is the set that has no elements. It is denoted as $$\varnothing$$ and it represents $$\{\}$$. The empty set is the only set whose cardinality is zero.


- $$\varnothing=\{\}\neq\{\{\}\}=\{\varnothing\} $$
- the empty set is regarded as an existing object.
- empty set is a subset of any set
- empty set is a proper subset of any non-empty set





A special notation called set-builder notation is used to describe sets that are too big or complex to list between braces X={exp,exp2,...|rule,rule2,...}

The expression |X| means absolute value if X is a number and cardinality if X is a set

It is an unfortunate notational accident that (a,b) can denote both an interval on the line and a point on the plane. The difference is usually clear from context.


## 2 The Cartesian Product

An ordered pair is a list (x, y) of two things x and y,
enclosed in parentheses and separated by a comma.  The things in an ordered
pair don’t have to be numbers

The Cartesian product of two sets A and B is another
set, denoted as A £B and defined as A £B ˘ '(a,b) : a 2 A,b 2 B“
Thus A £ B is a set of ordered pairs of elements from A and B
If A and B are finite sets, then |A|x|B| = |A|*|B|
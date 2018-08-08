# Sets



## Introduction to Sets

- sets
- set elements
- infinite and finite sets
- set equality
- set notation
- special sets

The theory of sets, initiated by Georg Cantor (1845–1918), constitutes the basis of almost all modern mathematics. A set is often described as a __collection__ (aggregate, class, totality, family) of objects of any specified kind, but the set concept is one of the basics so the term _set_ must be accepted as a primitive notion, without definition.

__Infinite__ set has infinitely many elements; otherwise it is a __finite__ set. Infinite sets: $\{1,2,3,\dots\}$, or $\{\dots,1,2,3,\dots\}$. A sets with terminal member: $\{1,2,3, \dots,100\}$, or $\{1,2,\dots,n\}$.

Two sets are __equal__ if they contain exactly the same elements. The order of the elements or their repetition is of no consequence to sets, $\{a,b,\{\}\}=\{b,\{\},b,\varnothing,a\}$.

Sets are usually denoted with uppercase letters: the set $\{2,4,6,8\}$ may be declared as $A=\{2,4,6,8\}$ and then use $A$ to stand for $\{2,4,6,8\}$.

The objects belonging to a set are called its __elements__ or __members__. To express that $2$ is an element of the set $A$, we write $2 \in A$. Expressions like $2,4,6 \in A$ indicate that several things are in a set. If an element is not in a set: $3 \notin A$. A set of one element is not the same as that element alone: $p \neq \{p\}$.

Once a set has been formed, it is a __new entity__, different from any of its elements. A newly formed set may be an element of some other set. Sets whose elements are other sets are __collections of sets__. If an element $a$ is in the set $A$ and set $A$ is in the set $\mathscr{M}$, that doesn't imply the element $a$ is in the set $\mathscr{M}$: $x \in A \land A \in \mathscr{M} \not \rightarrow x \in A$.

We are mainly concerned with sets whose elements are mathematical entities, such as numbers, points, functions, etc., but sets need not have just numbers as elements. Sets can even have other sets as elements.

Some sets are so prevalent they have a special symbol assigned for them. __Fundamental sets__:
- $\mathbb{B}$ is the set of boolean (truth) values: $\{\bot,\top\}$ or $\{F,T\}$ or $\{0,1\}$
- $\mathbb{N}$ is the set of natural numbers, either $\{1,2,3,\dots\}$ or $\{0,1,2,3,\dots\}$
- $\mathbb{Z}$ is the set of integers $\{-2,-1,0,1,2,\dots\}$ and $\mathbb{Z^+}=\{1,2,3,\dots\}$
- $\mathbb{Q}$ is the set of rational numbers: $\{p/q : p \in\mathbb{Z}, q\in \mathbb{N}\}$
- $\mathbb{R}$ is the set of real numbers. $\mathbb{N}\subseteq \mathbb{Z}\subseteq \mathbb{Q}\subseteq \mathbb{R}$
- $\mathbb{C}$ is the set of complex numbers: $\{a,b\,i:a,b\in\mathbb{R}\}$
- $\varnothing$ is the empty set.


If $X$ is a finite set, its __cardinality__ or size is the number of elements it has, and this number is denoted as $|X|$.

Another special set, regarded as an existing object, is the __empty set__, i.e. a set that has no elements: $\{\}$. It is denoted as $\varnothing$, so $\{\}=\varnothing$. The empty set is the only set whose cardinality is zero: $|\varnothing|=0$. Note: $\varnothing=\{\}$ and $\{\varnothing\}=\{\{\}\}$, so $\varnothing\neq \{\varnothing\} $.
- empty set is a subset of any set
- empty set is a proper subset of any non-empty set


A special notation called __set-builder notation__ is used to describe sets that are too big or complex to list between braces. Its general syntax is $X=\{exp:rule\}$. For example, $E=\{2n:n\in \mathbb{Z}\}$ which builds the infinite set of even integers, which can be read as "E equals the set of all things of form $2n$, such that $n$ is an element of $\mathbb{Z}$".


## The Cartesian Product

An ordered pair is a list (x, y) of two things x and y,
enclosed in parentheses and separated by a comma.  The things in an ordered
pair don’t have to be numbers

The Cartesian product of two sets A and B is another
set, denoted as A £B and defined as A £B ˘ '(a,b) : a 2 A,b 2 B“
Thus A £ B is a set of ordered pairs of elements from A and B
If A and B are finite sets, then |A|x|B| = |A|*|B|
# Set membership
math.dm.st.sets.memb

- Membership relation
- Set membership
- set elements
- set members
- subset relation
- set inclusion
- set containment
- subset
- superset
- set equality
- proper subset
- proper superset


## Set membership
An element, or member, of a set is any one of the distinct objects that make up that set. The elements of a set can be anything, including other sets, since a (newly defined) set is an object in its own right. The objects belonging to a set are called (its) **elements** or **members**.

Set theory builds upon a fundamental binary relation between an object and a set - **membership relation**. The relation "_is an element of_", also called **set membership**, is denoted by the symbol "$$\in$$" (modified Greek letter epsylon: $$\epsilon$$).

Notation that an object $$x$$ is a member of a set $$X$$ is written as $$x\in X$$ and read "_x is an element of A_", "_x belongs to A_", or similarly. If $$y$$ is not a member of $$X$$, this is denoted by $$y\in X$$, and read "_y does not belong to X_".

Once a set has been formed, it is an object in its own right, different from any of its elements. A set containing a single element is not the same as that element alone: $$x\neq \{x\}$$.


## Subset
**Set inclusion**, a binary relation derived from membership relation, is a binary relation between two sets. The relation "_is included in_", also called **subset relation**, is denoted by the symbol $$\subseteq$$.

If all members of set $$X$$ are also members of set $$Y$$, then $$X$$ is a **subset** of $$Y$$, denoted $$X\subseteq Y$$. Equivalently, $$Y$$ is a **superset** of $$X$$, denoted $$Y\supseteq X$$.

The relationship of one set being a subset of another is called **inclusion** or sometimes **containment**.

The subset relation defines a partial order on sets. The algebra of subsets forms a Boolean algebra in which the subset relation is called inclusion.

The definition for subset also means that any set is a subset (or superset) of itself.

> Any set, $$X$$, is a subset of itself, $$X\subseteq X$$

This also gives us the definition of **set equality**: two sets are equal if they containt precisely the same elements, that is, if they are each other's subsets:

> Sets $$X$$ and $$Y$$ are equal if $$X\subseteq Y$$ and $$Y\subseteq X$$.

If set $$Y$$ contains the same elements as set $$X$$, but also additional ones, then $$X$$ is a **proper subset** of $$Y$$, denoted as $$X\subset Y$$.

> $$X$$ is a proper subset of $$Y$$ if and only if $$X$$ is a subset of $$Y$$, but $$X\neq Y$$.


Set membership is not set inclusion: note that $$1,2,3$$ are elements of the set $$\{1,2,3\}$$, but not its subsets, i.e. $$1,2,3\in \{1,2,3\}$$.

On the other hand, subsets, such as $$\{1\},\{2\},\{3\}$$, are not elements of the set $$\{1,2,3\}$$, but (some of) its subsets, e.g. $$\{2\} \subseteq \{1,2,3\}$$.



## Notations

$$
let\ A=\{1,2,3\}  \\
then: \\
\quad 1\in A      \\
\quad A\ni 2      \\
\quad 1,2,3\in A  \\
\quad \{2\} \not\in A \\
\quad A\subseteq A \\
\quad 3 \not\subseteq A \\
\quad 1,2,3 \not\subseteq A \\
\quad \{2\}     \subseteq A \\
\quad \{1,2\}   \subseteq A \\
\quad \{1,2,3\} \subseteq A \\
$$


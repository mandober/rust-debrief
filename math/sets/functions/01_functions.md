# Functions


The concept of function was formalized at the end of the XIX century in terms of set theory.

Any subset of the Cartesian product between two non-empty sets, $$X\times Y$$ (possibly the same set, denoted by $$X^2$$), defines a **binary relation** between these two sets, denoted by $$R\subseteq (X\times Y)$$

## Function definition

> A function is a particular relation on sets, that associates each element $$x$$ of a set $$X$$, called a **domain** of the function (DOM), to a single element $$y$$ of a set $$Y$$, called a **codomain** of the function (COD).

A function is also defined by a set $$G$$ of ordered pairs $$(x,y)$$ such that $$x \in X$$ and $$y \in Y$$ and every element of $$X$$ is the first component of exactly one ordered pair in $$G$$.

A functions is a set of ordered pairs with unique first components. All functions are relations, but not vice versa.

Defining properties of a function:
1. **each** element of domain, must be associated, and
2. it must be associated to a **single** element in codomain

Additional properties:
- both domain and codomain are non-empty sets
- related element in domain is called **pre-image**
- all elements of domain are pre-images
- element that's associated to in codomain is called **image**
- not all elements of codomain are images
- the set of all images is called **range**
- an image may be associated to more then one pre-image
- function relation can be denoted by $$f (x)=y$$ (read as "f of x")
- pre-image $$x$$ is the input argument of the function
- image $$y$$ is the output value of the function
- a function can also be uniquely represented by its graph, which is a list of all the ordered pairs $$(x,y)\in R$$, where $$y=f(x)$$
- functions are also called maps, denoted by $$x\mapsto f(x)$$

If domain and codomain are comprised of numbers, then the ordered pairs, plotted as the points in the Cartesian coordinates system, form a curve that is also called the graph of the function. The related element $$y$$ in codomain is called _the image of $$x$$ under $$f$$_ or _the value of $$f$$ applied to the argument $$x$$_. In the context of numbers, it is common to say that $$y$$ is the value of $$f$$ of $$x$$, denoted as $$y = f(x)$$.


A **univalent** or **single-valued** relation is a relation where every element of domain is related to a single element of codomain:

$$\forall x\ \forall y_1\ \forall y_2\ \big[\ (x,y_1 \in R) \land (x,y_2 \in R) \to\ y_1=y_2\ \big]$$

Univalent relations may be identified to functions whose domain is a subset of X.


A **left-total relation** is a relation such that

$$\forall x\in X, \exists y\in Y:\;(x,y)\in R$$


Formal functions may be strictly identified to relations that are both univalent and left total. Violating the left-totality is similar to giving a convenient encompassing set instead of the true domain, as explained above.

Various properties of functions and also the functional composition may be reformulated in the language of relations. For example, a function is injective if the converse relation {\displaystyle R^{\text{T))\subseteq (Y\times X)} is univalent, where the converse relation is defined as {\displaystyle R^{\text{T))=\{(y,x):\;(x,y)\in R\}.}



## Notation
The most commonly used notation is **functional notation**, which defines the function using an equation that gives the names of the function and the argument explicitly.

Let $$f:\mathbb{R} \to \mathbb{R}$$ be the function defined by the equation $$f(x) = x^2$$.

The notation, $$y=f(x)$$, ("y equals f of x") means that the pair $$(x, y)$$ belongs to the set of pairs defining the function $$f$$. If $$X$$ is the domain of $$f$$, the set of pairs defining the function is: 
$$\{(x,f(x))\colon x\in X\}$$

To explicitly express domain $$X$$ and the codomain $$Y$$ of a function $$f$$, the **arrow notation** is often used:

$$f:X\to Y\quad$$

"_the function f from X to Y_". Or:

$$X \stackrel{f} \to Y$$

"_the function f maps x to f (x)_"

This is often used in relation with the arrow notation for elements, often stacked immediately below the arrow notation giving the function symbol, domain, and codomain:

$$x\mapsto f(x)$$

**Index notation** is often used instead of functional notation; instead of writing $$f (x)$$, one writes: $$f_{x}$$


## Image and preimage

Let $$f: X\to Y$$, then the **image** by $$f$$ of an element $$x$$ of the domain $$X$$ is $$f(x)$$. If $$A$$ is any subset of $$X$$, then the image of $$A$$ by $$f$$, denoted $$f(A)$$ is the subset of the codomain $$Y$$ consisting of all images of elements of $$A$$, that is, $$f(A)=\{f(x)\mid x\in A\}$$

The image of $$f$$ is the image of the whole domain, that is $$f(X)$$, also called the range of $$f$$.

The **inverse image** or **preimage** by $$f$$ of a subset $$B$$ of the codomain $$Y$$ is the subset of the domain $$X$$ consisting of all elements of $$X$$ whose images belong to $$B$$. It is denoted by $$f^{-1}(B)$$. That is,    

$$f^{-1}(B)=\{x\in X\mid f(x)\in B\}$$

For example, the preimage of $$\{4,9\}$$ under the square function is the set $$\{−3,−2,2,3\}$$.



## Injective

An **injective** or **one-to-one** function, or **injection**, is a function that preserves distinctness: it never maps distinct elements of its domain to the same element of its codomain; every element of the function's codomain is the image of at most one element of its domain.


A function $$f:X\to Y$$ is **injective** or **one-to-one** if for every $$x \in X$$, there exists **at most one** $$y \in Y$$ such that $$f(x)=y$$. A function $$f$$ is injective if $$x\neq y$$ implies $$f(x)\neq f(y)$$.

$$x\neq y \Rightarrow f(x)\neq f(y)$$.

The function $$f : X\to Y$$ is injective if $$f(x_1)\neq f(x_2)$$ for any two distinct elements, $$x_1, x_2 \in X$$.


## Surjective
A function f from a set X to a set Y is surjective (or onto), or a surjection, if for every element y in the codomain Y of f there is at least one element x in the domain X of f such that f(x) = y. The function f may map one or more elements of X to the same element of Y, so y need not be unique.


A function f:A→B is surjective (onto) if the image of f equals its range.

Equivalently, for every b∈B, there exists some a∈A such that f(a)=b. This means that for any y in B, there exists some x in A such that y=f(x).


## Bijective
A function is **bijective** or **one-to-one correspondent** iff f is both injective and surjective.

A **bijection** is a function between the elements of two sets, where each element of one set is paired with exactly one element of the other set, and each element of the other set is paired with exactly one element of the first set. There are no unpaired elements.

In mathematical terms, a bijective function f: X → Y is a one-to-one (injective) and onto (surjective) mapping of a set X to a set Y.


## Inverse
The inverse of a one-to-one corresponding function f:A→B, is the function g:B→A, holding the following property −

f(x)=y⇔g(y)=x
The function f is called invertible, if its inverse function g exists.

Example
A Function f:Z→Z,f(x)=x+5, is invertible since it has the inverse function g:Z→Z,g(x)=x−5.

A Function f:Z→Z,f(x)=x2 is not invertiable since this is not one-to-one as (−x)2=x2.


## Composition
Functions $$f:X\to Y$$ and $$g:Y\to Z$$ can be composed to give a composition $$g\circ f$$, equal to $$g(f(x))$$, only if the codomain of $$f$$ is defined as the domain of $$g$$.

Composition properties
- Composition is associative, $$h\circ g\circ f=(h\circ g)\circ f=h\circ (g\circ f)$$
- Composition is not commutative
- If $$f$$ and $$g$$ are one-to-one functions, their composition is as well.
- If $$f$$ and $$g$$ are onto functions, their composition is as well.


A **function space** is a set of functions between domain and codomain.

A **constant function** is a function whose output value is the same for every input value.

An **identity function** (also called an identity relation, identity map or identity transformation) is a function that always returns the same value that was used as its argument.

A **linear map** (also called a linear mapping, linear transformation, soemtimes also linear function) is a mapping $$V \to W$$ between two modules (including vector spaces) that preserves the operations of addition and scalar multiplication. An important special case is when $$V = W$$, in which case the map is called a linear operator, or an endomorphism of $$V$$. Sometimes the term linear function has the same meaning as linear map, while in analytic geometry it does not.

A **polynomial** is an expression consisting of variables (also called indeterminates) and coefficients, that involves only the operations of addition, subtraction, multiplication and non-negative integer exponents of variables. An example of a polynomial of a single indeterminate x is x2 − 4x + 7.

A **rational function** is any function which can be defined by a rational fraction, i.e. an algebraic fraction such that both the numerator and the denominator are polynomials.

An **algebraic function** is a function that can be defined as the root of a polynomial equation. Quite often algebraic functions are algebraic expressions using a finite number of terms, involving only the algebraic operations addition, subtraction, multiplication, division and raising to a fractional power.

An **analytic function** is a function that is locally given by a convergent power series. There exist both real analytic functions and complex analytic functions, categories that are similar in some ways, but different in others.

The smoothness of a function is a property measured by the number of derivatives it has that are continuous. A **smooth function** is a function that has derivatives of all orders everywhere in its domain.

A **continuous function** is a function for which sufficiently small changes in the input result in arbitrarily small changes in the output. Otherwise, a function is said to be a **discontinuous function**.

A **measurable function** is a function between two measurable spaces such that the preimage of any measurable set is measurable, analogously to the definition that a function between topological spaces is continuous if the preimage of each open set is open.

**Restriction** of a function $$f$$ is a new function $$f\vert _{A}$$ obtained by choosing a smaller domain $$A$$ for the original function $$f$$. The notation $$f{\upharpoonright _{A}}$$ is also used.





## Properties

Classes/properties
- constant: output is the same for every input.
- identity: output is the same as input.
- Linear
- Polynomial
- Rational

- Algebraic
- Analytic
- Smooth
- Continuous
- Measurable
- Injective
- Surjective
- Bijective


Constructions  
- Restriction
- Composition
- lambda, λ
- Inverse

Generalizations  
- Partial
- Multivalued
- Implicit
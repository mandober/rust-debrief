# Functions

Discrete Mathematics: Relations
https://www.tutorialspoint.com/discrete_mathematics/discrete_mathematics_quick_guide.htm


A function assigns to each element of a set, exactly one element of a related set. 

Functions find their application in various fields like representation of the computational complexity of algorithms, counting objects, study of sequences and strings, to name a few.

## Definition of a function
A function or mapping (Defined as f:X→Y) is a relationship from elements of one set X to elements of another set Y (X and Y are non-empty sets). X is called Domain and Y is called Codomain of function ‘f’.

Function ‘f’ is a relation on X and Y such that for each x∈X, there exists a unique y∈Y such that (x,y)∈R. ‘x’ is called pre-image and ‘y’ is called image of function f.

A function can be one to one or many to one but not one to many.

## Injective / One-to-one function
A function f:A→B is injective or one-to-one function if for every b∈B, there exists at most one a∈A such that f(s)=t.

This means a function f is injective if a1≠a2 implies f(a1)≠f(a2).

Example
f:N→N,f(x)=5x is injective.

f:N→N,f(x)=x2 is injective.

f:R→R,f(x)=x2 is not injective as (−x)2=x2

## Surjective / Onto function
A function f:A→B is surjective (onto) if the image of f equals its range. Equivalently, for every b∈B, there exists some a∈A such that f(a)=b. This means that for any y in B, there exists some x in A such that y=f(x).

Example
f:N→N,f(x)=x+2 is surjective.

f:R→R,f(x)=x2 is not surjective since we cannot find a real number whose square is negative.

## Bijective / One-to-one Correspondent
A function f:A→B is bijective or one-to-one correspondent if and only if f is both injective and surjective.

Problem
Prove that a function f:R→R defined by f(x)=2x–3 is a bijective function.

Explanation − We have to prove this function is both injective and surjective.

If f(x1)=f(x2), then 2x1–3=2x2–3 and it implies that x1=x2.

Hence, f is injective.

Here, 2x–3=y
So, x=(y+5)/3 which belongs to R and f(x)=y.

Hence, f is surjective.

Since f is both surjective and injective, we can say f is bijective.

## Inverse of a Function
The inverse of a one-to-one corresponding function f:A→B, is the function g:B→A, holding the following property −

f(x)=y⇔g(y)=x
The function f is called invertible, if its inverse function g exists.

Example
A Function f:Z→Z,f(x)=x+5, is invertible since it has the inverse function g:Z→Z,g(x)=x−5.

A Function f:Z→Z,f(x)=x2 is not invertiable since this is not one-to-one as (−x)2=x2.

## Composition of Functions
Two functions f:A→B and g:B→C can be composed to give a composition gof. This is a function from A to C defined by (gof)(x)=g(f(x))
Example
Let f(x)=x+2 and g(x)=2x+1, find (fog)(x) and (gof)(x).

Solution
(fog)(x)=f(g(x))=f(2x+1)=2x+1+2=2x+3
(gof)(x)=g(f(x))=g(x+2)=2(x+2)+1=2x+5
Hence, (fog)(x)≠(gof)(x)
Some Facts about Composition
If f and g are one-to-one then the function (gof) is also one-to-one.

If f and g are onto then the function (gof) is also onto.

Composition always holds associative property but does not hold commutative property.
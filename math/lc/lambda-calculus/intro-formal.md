# Introduction

Formal introduction:

Assume given an infinite set $$\mathcal{V}$$ of variables, denoted by $$x, y, z\dots$$     
The set of lambda terms is given by the following Backus-Naur Form    
($$M$$ and $$N$$ are lambda terms, $$x$$ is a variable):     

$$
M, N ::= x \ |\ (MN)\ |\ (\lambda{x}.M)
$$

Traditional definition:
- Assume given an infinite set $$\mathcal{V}$$ of variables
- Let $$A$$ be an alphabet consisting of the elements of $$\mathcal{V}$$ and the special symbols, $$\lambda$$, `.`, `(`, `)`
- Let $$A^∗$$ be the set of strings (finite sequences) over the alphabet $$A$$
- The set of lambda terms is the smallest subset $$\Lambda \subseteq A^∗$$ such that:
  - Whenever $$x\in \mathcal{V}$$ then $$x\in \Lambda$$
  - Whenever $$M,N\in \Lambda$$ then $$(MN)\in \Lambda$$
  - Whenever $$x\in \mathcal{V}$$ and $$M\in \Lambda$$ then $$\lambda{x}.M \in \Lambda$$


Lamda calculus has 3 forms:
1. Variables, $$x$$
2. Application, $$MN$$, associate to the left: $$fxyz$$ is $$((fx)y)z$$
3. Abstraction, $$\lambda x.M$$, associate to the right: $$\lambda f.xyz$$ $$\lambda f.(xyz)$$



Examples:
- Variables: `x`, `y`
- Applications: `xx`, `xy`, `(λx.(xx))(λy.(yy))`
- Abstractions: `λx.x`, `λf.ff`, `λf.(λx.(f(fx)))`
- Abstraction and application: `(λx.x)(2)`, `((λf.f(f))(λx.x+2))(3)`


This, `λx.x+1`, defines an anonymous function that takes a parameter `x` and evaluates (returns) it to `x+1`. This applies this function to the argument 5: `(λx.x+1)(5)`; the 5 replaces the `x` in the function's body, so this expression evaluates to 6.

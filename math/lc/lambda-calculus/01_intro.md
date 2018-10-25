# Lambda Calculus

The λ-calculus is a prototype programming languages invented by a logician, Alonzo Church, in the 1930's. It is a higher-order language i.e. it gives a systematic notation for operators whose input and output values may be other operators. It is also a functional language with notation for function application and abstraction.


## Informal introduction
Lamda calculus has only 3 kinds of expressions: variables, abstraction (i.e. function definition) and function application.

* **Variables**:   
  infinite set of vars (with sub/superscript if needed) e.g. $$a,f,x_2\dots$$
* **Abstraction**:   
  is (anonymous) function definition, e.g. $$\lambda x.x$$ defines a function that takes a parameter, $$x$$, and returns the evaluated body (the expression after the dot), in this case just $$x$$ (id function).
* **Application**: applying a function to an expression, e.g. $$(\lambda x.x)(y)$$ applies the id function to $$y$$.


$$fx$$, associate to the left: $$fxyz$$ is $$((fx)y)z$$



associate to the right: $$\lambda f.xyz$$ $$\lambda f.(xyz)$$



## Formal introduction

- Assume given an infinite set $$\mathcal{V}$$ of variables, denoted by $$x, y, z\dots$$
- The set of lambda terms is given by the following Backus-Naur Form    
  ($$M$$ and $$N$$ are lambda terms, $$x$$ is a variable)    
  $$M, N ::= x \ |\ (MN)\ |\ (\lambda{x}.M)$$

Traditional introduction:
- Assume given an infinite set $$\mathcal{V}$$ of variables
- Let $$A$$ be an alphabet consisting of the elements of $$\mathcal{V}$$ and the special symbols: `λ`, `.`, `(`, `)`
- Let $$A^∗$$ be the set of strings (finite sequences) over the alphabet $$A$$
- The set of lambda terms is the smallest subset $$\Lambda \subseteq A^∗$$ such that:
  - Whenever $$x\in \mathcal{V}$$ then $$x\in \Lambda$$
  - Whenever $$M,N\in \Lambda$$ then $$(MN)\in \Lambda$$
  - Whenever $$x\in \mathcal{V}$$ and $$M\in \Lambda$$ then $$\lambda{x}.M \in \Lambda$$


## Syntax

There are 3 kinds of **λ-expressions**:
1. **Variables**, $$v_0,v_1,v_2,\dots$$, e.g. $$x,y,z,\dots$$
2. **Application**, $$e_1e_2$$, which means $$e_1(e_2)$$ i.e. application of expression $$e1$$ to expression $$e2$$.
3. **Abstractions**, $$\lambda x.e$$, represent the anonymous function which evaluates to (returns) the value $$e$$ when given formal parameter $$x$$.


**Association**
- Application, $$abcd$$, associate to the left: $$((ab)c)d$$
- Abstraction, $$\lambda f.\lambda g.\lambda h.x$$, associate to the right: $$\lambda f.(\lambda g.(\lambda h.x))$$

**Convention**
- Juxtaposed abstraction, $$\lambda a.\lambda b.\lambda c.abc$$, can be shorthanded as $$\lambda abc.abc$$
- Parenthesis are used to disambiguate expressions or to enforce evaluation order: application $$abcd$$ is interpreted as $$((ab)c)d$$, to enforce right-associativity, write $$a(b(cd))$$



## Abstraction
A lambda function is an abstraction over a lambda expression:    
$$\quad \ \lambda x.e$$    
where $$x$$ is a formal parameter and $$e$$ is a (sub)expression.

Examples of abstractions:

$$
\quad \ \lambda x.x\\
\quad \ \lambda fs.f\\
\quad \ \lambda af.fa
$$

- Greek letter lambda introduces a name used for abstraction.
- taht name is function's bound variable (here $$x$$) and it represents a  formal parameter similar to function declarations in PL.
- The dot separates the name (formal parameter) from the function's body.
- function's body is an expression in which abstraction with that name (actual parameter) takes place.
- expression (in function's body) may be any lambda expression, including another function abstraction.


<details><summary>Similarity with programming languages</summary><br>

Lambda abstraction (assuming the function `+` was defined, as infix):   
$$\quad \ \lambda x.\lambda y.x+y$$


```js
// in js
let f = x => y => x + y;
```

```rust
// in rust
let f = move |x| move |y| x + y;
```

```ocaml
(* in ocaml *)
let f x y = x + y;;
```

</details><br>



## Variables
Variables that are named in abstraction are bound: $$\lambda x.xyx$$     
here, the $$x$$ is bound, while the $$y$$ is a free variable.

If a bound parameter does not appear in the body of a function, it just gets thrown away.

$$
(\lambda x.yz)a\\
yz
$$


- An occurrence of a variable `x` within a subexpression `e` in `λx.e` is a **bound variable**.
- An occurrence of an unbound variable is a **free variable**.
- The occurrence of `x` in the abstraction's head, `λx.`, is the **binding occurrence**, which introduces the variable `x`
- The occurrence of bound `x` in the abstraction's body is called **applied occurrences**.
- If some occurrence of `x` is free in the subexpression then that is a **free occurrence** of variable `x`.
- A variable `x` is bound by the syntactically _innermost enclosing_ `λ`, if one exists, just as in any block-structured programming language.
- **Closed expression** contains no free variables.
- **Open expression** contains free variables.
- **Combinators** are lambda abstractions without free variables.


The same variable may occur both bound and free in an expression. For example, the first applied occurrence of $$x$$ in $$(\lambda x.\lambda y.yx)((\lambda z.zx)x)$$ is bound, but the second and third applied occurrences are free.

We can index all occurances of x    
$$\quad \ (\lambda x_b.\lambda y.yx_1)((\lambda z.zx_2)x_3)$$    
to better see that the first occurance of $$x$$ i.e. $$x_1$$ is bound by $$x_b$$, while the two other occurances, $$x_2$$ and $$x_3$$, are free.


## Alpha-conversion

The substitution of $$f$$ for the free occurrences of $$x$$ in $$e$$, written $$e[f/x]$$, is defined:
- $$x[f/x] :=f$$    
  and for a variable $$y$$, if $$y\not \equiv x$$, $$y[f/x]:=y$$
- For applications, we substitute into the two parts:    
  $$\quad \ (e_1\ e_2)[t/x] := (e_1[t/x]\ e_2[t/x])$$
- If $$e\equiv \lambda x.g$$ then $$e[f/x] := e$$    
  If $$y$$ is a variable distinct from $$x$$, and $$e\equiv \lambda y.g$$ then
  - if $$y$$ does not appear free in $$f$$,     
    $$e[f/x]:=\lambda y.g[f/x]$$
  - if $$y$$ does appear free in $$f$$,    
    $$e[f/x] := \lambda z.(g[z/y][f/x])$$     
    where $$z$$ is a variable which does not appear in $$f$$ or $$g$$.
    Note that we have an infinite collection of variables,    
    so that we can always find such a "z".
- In general, it is easy to see that if $$x$$ is not free in $$e$$    
  then $$e[f/x]$$ is $$e$$



## Application

A function application specialises an abstraction by providing a value for the name (formal parameter). The function expression contains the abstraction to be specialised with the argument expression. In a function application, also known as a __bound pair__, _the function expression is said to be applied to the argument expression_.

When the function $$(\lambda x.xy)$$ is applied to argument $$3$$, $$(\lambda x.xy)3$$:
- the value of the argument ($$3$$) is assigned to the formal parameter $$x$$
- the head of the function is dropped ($$\lambda x.$$)
- the actual parameters in the function's body (bound params with the same name as the formal param) are replaced with the value $$3$$
- the expression evaluates to $$3y$$

The process of evaluation of a λ-expression is called _simplification_ or __β-reduction__ (beta-reduction).

In general, to evaluate application of a lambda abstraction, i.e.    
$$\quad \ (\lambda x .e1)e2$$
we replace the actual parameters in the function's body with the formal parameter - this **substitution** is denoted by $$e1[e2/x]$$.


## Lazy and strict evaluation

There are two approaches to evaluating a function applications:
- **strict** evaluation: **by value** i.e. applicative order
- **lazy** evaluation: **by reference** i.e. normal order

The former approach is called __applicative order__ and it is similar to "call by value" in PL in that the value of the argument is evaluated before being assigned as the value of the formal parameter.

The second approach is called __normal order__ and it is similar to "call by reference" in PL in that the value of the argument is assigned to the formal parameter unevaluated.

All occurences of the function's bound variable in the function's body expression are replaced by either the value of the argument expression or the unevaluated argument expression.

In most cases the evaluation order amounts to the same result, but sometimes the order does matter, to the point of evaluation decidibility.




<details><summary>For example</summary><br>

$$(\lambda x.x)(((\lambda xy.x)a)b)$$    
here the argument expression, $$(((\lambda xy.x)a)b)$$,     
is either evaluated first and only then applied to the abstraction, $$(\lambda x.x)$$, or it is applied immediately unevaluated:

by value (evaluate the arg before applyng it):
$$
\quad \ (\lambda x.x)\ (((\lambda xy.x)a)b)\\
\quad \ (\lambda x.x)\ ((\lambda y.a)b)\\
\quad \ (\lambda x.x)\ (a)\\
\quad \ a
$$

by reference (apply the arg unevaluated):
$$
\quad \ (\lambda x.x)\ (((\lambda xy.x)a)b)\\
\quad \ (((\lambda xy.x)a)b)\\
\quad \ ((\lambda y.a)b)\\
\quad \ a
$$

</details><br>



## Currying
Lambda functions take one parameter at the time. In order to supply more then one parameter, there can be one function per argument: the first function accepts the first parameter and returns the second function that accepts the second parameter (with the first parameter fixed), and so on.

The function $$\lambda xy.xy$$ is the same as $$\lambda x.\lambda y.xy$$, which is the same as $$\lambda x.(\lambda y.xy)$$.

When it is applied, $$((\ (\lambda x.(\lambda y.xy))\ a)\ b)$$, the argument $$a$$ is bound by the outer abstraction (parameter $$x$$), while the argument $$b$$ is bound by the inner (parameter $$y$$):

$$
(\ (\lambda x.(\lambda y.xy))\ a)\ b\\
(\lambda y.ay)\ b\\
ab
$$



## Examples

<details><summary>Examplary expressions</summary><br>


**Example**: Evaluating a lambda expression    
(assuming the function `+` was defined, as infix)

$$
((\lambda x . \lambda y .x+y)(5))(2)\\
(\lambda y.5+y)(2)\\
5 + 2\\7
$$


**Example**: Evaluation i.e. _beta reduction_
$$
(\ (\ \lambda f.ff\ )\ \ (\lambda x.x)\ )3\\
(\ (\lambda x.x)(\lambda x.x)\ )3\\
(\ \lambda x.x\ )3\\3
$$


**Example**: evaluation fork
$$
(\ (\lambda xy.xy)\ (\lambda f.f)\ )\ a\\
(\ (\lambda x.(\lambda y.xy))\ (\lambda f.f)\ )\ a\\
(\lambda y.(\lambda f.f)y)\ a \quad\\
(\lambda y.y)\ a \quad | \quad (\lambda f.f)a\\
a \quad \quad \quad \quad | \quad a
$$


**Example**: constant function (unused bound parameter)
$$
(\lambda x.\lambda y.z)\ (\lambda f.f)\\
\lambda y.z
$$


**Example expressions**:

$$
(\lambda x.y)z \neq \lambda x.yz
\\\quad\\
\lambda f.fxy = \lambda f.((fx)y)
$$

</details><br>

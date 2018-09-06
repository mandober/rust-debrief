# Introduction

- The untyped lambda calculus is a formal language.
- The expressions of the language are called lambda terms.
- Lambda terms (BNF): $$M, N ::= x \ |\ (MN)\ |\ (\lambda{x}.M)$$
  - that is, there are variables: $$x, f, a$$
  - application (function evaluation) is juxtaposition: $$fx$$ means $$f(x)$$
  - application (right-associative): $$xyzw$$ means $$((xy)z)w$$
  - abstraction (left-associative): $$\lambda x . \lambda y . xy$$ means $$\lambda x . (\lambda y . xy)$$
  - abbreviation: $$\lambda xyz.xyz$$ means $$\lambda x.\lambda y.\lambda z.xyz$$


Lambda abstraction (assuming the function `+` was defined, as infix):
\[
\lambda x . \lambda y .x+y
\]

Lambda abstraction is function definition e.g.

```js
// js
let f = x => y => x + y;
```

```rust
// rust
let f = move |x| move |y| x + y;
```

```ocaml
(* ocaml *)
let f x y = x + y;;
```

---

**Example**: Evaluating a lambda expression    
(assuming the function `+` was defined, as infix)

\[
((\lambda x . \lambda y .x+y)(5))(2)\\
(\lambda y.5+y)(2)\\
5 + 2\\
7
\]

**Example**: Evaluating a lambda expression
\[
(\ (\ \lambda f.ff\ )\ \ (\lambda x.x)\ )3\\
(\ (\lambda x.x)(\lambda x.x)\ )3\\
(\ \lambda x.x\ )3\\
3
\]

**Examples**:

\(
(\lambda x.y)z \neq \lambda x.yz
\\
\quad
\\
\lambda f.fxy = \lambda f.((fx)y)
\)


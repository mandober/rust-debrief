# Lambda Calculi

https://en.wikipedia.org/wiki/Lambda_cube

The λ-cube is a framework for exploring the axes of refinement in Thierry Coquand's _calculus of constructions_, starting from the _simply typed lambda calculus_ as the vertex of a cube placed at the origin, and the calculus of constructions (higher-order dependently typed polymorphic lambda calculus) as its diametrically opposite vertex. Each axis of the cube represents a new form of abstraction.

[![Lambda_cube][pic]][link]

[link]: https://en.wikipedia.org/wiki/Lambda_cube
[pic]: https://upload.wikimedia.org/wikipedia/commons/1/19/Lambda_cube.png  "Lambda cube"


λ cube

```
         λfΠω
         /|\
        / | \
       /  |  \
      /   |   \
     /    |    \
    /     |     \
   λfω    |     λΠω 
   | \   λfΠ   / |
   |  \.     ./  |
   | . \     / . |
  λf    \   /   λΠ
    \    \ /    /
     \    λω   /
      \   |   /
       \  |  /
        \ | /
         \|/
          λτ
```

- `λτ`  simply typed lambda calculus
- `λf`  second-order lambda calculus
- `λΠ`  
- `λω`  
- `λfΠ`  
- `λfω`  
- `λΠω`  
- `λfΠω` higher-order dependently typed polymorphic lambda calculus



$$\lambda \underline{\omega}$$

- Values depending on types, or polymorphism.   
  Second order lambda calculus (λf), System F, is obtained by imposing only this property.

- Types depending on types, or type operators.
  Simply typed lambda calculus with type operators
  (λω in the diagram) is obtained by imposing only this property.
  Combined with System F it yields System Fω (λω).

- Types depending on values, or dependent types.
  Imposing only this property yields λΠ 
  (written as λP in the diagram), 
  a type system closely related to LF.


---

Does types being terms imply your dependend theory is considered polymorphic?
https://cs.stackexchange.com/a/19054

Q: In the introduction of the book by B.Jacobs, "Categorical Logic and Type Theory" (it's online here), he classifies type systems into three general flavours: Simply typed ones, depended typed (term depended types) and polymorphic types (type depended types). He says also there are also mix types.

Now if you start out with a dependently types theory and introduce transitive universes, hence forcing types on the level of terms, are you automatically speaking of a polymorphic type system then?


A: These three “flavors” are usually presented as the lambda cube. They aren't really flavors but properties that can be combined independently, so there are not 3 but 23=8 flavors. In fact you could go as high as 4 properties yielding 16 flavors:

- terms depending on terms: t -> t
  functional languages
  $$f:x\mapsto x+2$$
- terms depending on types: T -> t
  polymorphism
  head:[X]→X
- types depending on types: T -> T
  type operators
  list:X↦[X]
- types depending on terms: t -> T
  dependent types
  array(int):n↦int n


If you introduce transitive universes, then having types depending on terms automatically gives you types depending on higher-order terms, i.e. types.
The Barendregt cube tends to collapse when terms and types are collapsed. However this doesn't necessarily give you all the polymorphism you want. The Barendregt cube is a classification; it doesn't reflect all the properties of the language.
In particular, polymorphism can be more or less explicit, requiring you to thread type arguments down to every function, or not.
I think the right way to put it is that you're automatically speaking of a type system that can encode a polymorphic type system, but not necessarily give it to you the way you want.

---

What terms type systems exclude?
https://cs.stackexchange.com/a/49381

In the simply-typed lambda calculus, the type of a function must be fully specified, even when it's irrelevant. A trivial example is the identity function: in the lambda calculus, it's λx.x; in the simply-typed lambda calculus, there isn't a single identity function but rather a family of identity functions λx:T.x for every type T. This means that the simply-typed lambda calculus has no polymorphism. If you think about data structures, more concrete examples come up — you can't write, say, a generic function like list reversal, you have to write it all over again for each type of list elements. You can't even write basic primitives like nil and cons, only typed-indexed primitives nilT and consT.



```
  λω ----------- λΠω
  /|            /|
 / |           / |
λf-|-------- λΠf |
|  |          |  |
|  |          |  |
|  |          |  |
| λϖ ---------|- λΠϖ
| /           | /
|/            |/
λ→ --------- λΠ

Π ϖ ω f λ τ
```

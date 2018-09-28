# Lambda Calculi

The lambda  cube is a framework for exploring the axes of refinement in Thierry Coquand's _calculus of constructions_, starting from the _simply typed lambda calculus_ as the vertex of a cube placed at the origin, and the calculus of constructions (higher-order dependently typed polymorphic lambda calculus) as its diametrically opposite vertex. Each axis of the cube represents a new form of abstraction.


```
  λω ----------- λΠω
  /|            /|
 / |           / |
λf-|-------- λΠf |
|  |          |  |
|  |          |  |
|  |          |  |
| λω ---------|- λΠϖ
| /           | /
|/            |/
λ→ --------- λΠ

Π ϖ ω f λ τ
```

- `λ→` simply typed lambda calculus
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


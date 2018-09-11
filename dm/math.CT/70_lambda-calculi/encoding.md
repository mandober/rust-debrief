# Church Encoding

Church encoding is a means of representing data and operators in the lambda calculus. The data and operators form a mathematical structure which is embedded in the lambda calculus. The Church numerals are a representation of the natural numbers using lambda notation.

## Church Booleans
Church Booleans are the Church encoding of the Boolean values true and false. Since Boolean logic may be considered as a choice, the Church encoding of true and false values are binary functions:
- true chooses the first parameter, `T := λxy.x`
- false chooses the second parameter, `F := λxy.y`
- `NOT := λx.xFT`
- `AND := λxy.xyF`

```
Tp_ -> p    always first (second ignored)
F_q -> q    always second (first ignored)
xTF -> x    always self
xFT -> !x   always the opposite
xTT -> T    always T
xFF -> F    always F
```

### NOT
Good encoding for NOT seems to be `(λx.xFT)` which always returns the opposite value:

```
(λx.xFT)T -> TFT -> F
(λx.xFT)F -> FFT -> T
```

### AND

`AND := λxy.xyF`

(λxy.xy`F`)TT |(λxy.xy`F`)TF |(λxy.xy`F`)FT |(λxy.xy`F`)FF
--------------|------------|------------|-----------
(λy.Ty`F`)T   | (λy.Ty`F`)F| (λy.Fy`F`)T| (λy.Fy`F`)F
TT`F`         | TF`F`      | FT`F`      | FF`F`
T             | F          | `F`        | `F`


### IMPLICATION

`-> := λxy.xyT`

x | y | xyF | xyT
--|---|-----|----
T | T |  T  |  T
T | F |  F  |  F
F | T |  F  |  T
F | F |  F  |  T
. |   | AND | IMPL


### OR

`OR := λxy.xTy`

# Church Encoding

$$
\begin{align}
& {true }  & {T    }  & {\ := \lambda xy.x  }\\
& {false}  & {F    }  & {\ := \lambda xy.y  }\\
& {not  }  & {\lnot}  & {\ := \lambda x.xFT }\\
& {imply}  & {\to  }  & {\ := \lambda xy.xyT}\\
& {and  }  & {\land}  & {\ := \lambda xy.xyF}\\
& {or   }  & {\lor }  & {\ := \lambda xy.xTy}\\
\end{align}
$$

Church encoding is a means of representing data and operators in the lambda calculus. The data and operators form a mathematical structure which is embedded in the lambda calculus. The Church numerals are a representation of the natural numbers using lambda notation.

Church Booleans are the Church encodings of the Boolean values true and false, and boolean operators.

## Boolean values
A boolean values true and false are used in programming to make decisions, e.g. if some condition is true do one thing, otherwise do another thing. This gives a way to encode `true` value as, somewhat arbitrary, function that always returns the first parameter when 2 parameters are supplied; the `false` value can be represented by a function that always returns the second parameter when 2 parameters are supplied.

$$
true\ \ T :=\ \lambda xy.x\\
false\ \ F :=\ \lambda xy.y\\
\ \\
(\lambda xy.x)\ 1\ 0\\
(\lambda y.1)\ 0\\
1
$$


## Boolean operators
To encode Boolean operators like NOT, AND, OR, IMPLIES, we can use the two predefined functions `T` and `F` and combine them with arguments.

## Unary operators
NOT is a unary operator that we can get by seting up the triplet of `T`, `F` and the argument in a certain way so that the value of input is flipped:

$$
not\ \ \ \lnot\ =: \lambda xFT
$$

This way, if argument is `T`, the output is `F` and vice versa.

We can get the other 3 unary operators by different combinations of `x` (argument) with `T` and `F`:

$$
\lambda xTT \quad always\ T\\
\lambda xTF \quad identity\\
\lambda xFT \quad not\\
\lambda xFF \quad always\ F
$$


## Binary operators

We can get AND, OR and IMPLY by combining the two arguments with either `T` or `F`

$$AND := \lambda xy.xyF$$


$$IMPLICATION := \lambda xy.x(yT)$$


The other all binary operators by combining the two arguments and two Boolean values (functions) `T` and `F`.

The form of lambda expression is $$\lambda xy.xy\phi$$, where $$x$$ and $$y$$ are the arguments and $$\phi$$ is a boolean value, $$T$$ or $$F$$.

The table displays just the body of the lambda abstraction, since the head is always the same, $$\lambda xy$$:


x | y | xyβ | xβy | Txy | Fxy
--|---|-----|-----|-----|----
T | T |  y  |  β  |x  T |y  T
T | F |  y  |  β  |x  T |y  F
F | T |  β  |  y  |x  F |y  T
F | F |  β  |  y  |x  F |y  F


x |y |xyT |xyF|xTy|xFy|Txy|Fxy
--|- |----|---|---|---|---|---
T |T | T  | T | T | F | T | T
T |F | F  | F | T | F | T | F
F |T | T  | F | T | T | F | T
F |F | T  | F | F | F | F | F
x |y | →  | ∨ | ∧ |   |   |




---

¬ ∨ ∧ → ⇔


(λxy.xy`F`)TT |(λxy.xy`F`)TF |(λxy.xy`F`)FT |(λxy.xy`F`)FF
--------------|------------|------------|-----------
(λy.Ty`F`)T   | (λy.Ty`F`)F| (λy.Fy`F`)T| (λy.Fy`F`)F
TT`F`         | TF`F`      | FT`F`      | FF`F`
T             | F          | `F`        | `F`


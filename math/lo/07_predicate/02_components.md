# Syntax

Components of predicate logic
- Atomic sentence, atomic formula, term, expression, wff
- Symbols
- Constants
- Variables, free and bound
- Predicates, predicate symbols
- Quantifiers
- Universe of discourse


There are 6 kinds of symbols in PL:
- constants:  $$a,b,c,\ldots$$
- variables:  $$x,y,z,\ldots$$
- predicates: $$A,B,C,\ldots$$
- connectives: ¬ ∧ ∨ → 
- quantifiers: ∃ ∀
- parentheses

(first 3: possibly with subscripts). An expression is any string of symbols; symbols in any order form a PL expression.


## Constant
A **logical constant** of a formal language is a symbol that has the same semantic value under every interpretation of that language.

Two important types of logical constants are logical connectives and quantifiers.

The equality predicate (usually as infix `=`) is also treated as a logical constant in many logic systems.



## Predicates
Fundamental component in predicate logic is a **predicate**, symbolized by an 
uppercase letter called **predicate symbol**, which is an expression that, combined with a name (variables and constants), produces an atomic sentence.

A predicate is an expression like "_is a man_", which is not a sentence on its own and which doesn't have a truth value. In order to get a truth value we need to specify an object as an argument of this predicate.

Predicates translate 3 kinds of statements: singular, universal and particular.


## Singular statements
A singular statement is an affirmative or negative statement that asserts something about a named object (person, place, time, etc.).

- Singular terms are constants and variables.
- **Constants** pick out specific individuals.
- **Variables** do not stand for any specific individual - they are needed for introduction of quantifiers.
- An individual variable differs from an individual constant in that it can stand for any item in the universe of discourse (UD).
- A **proper name** is a singular term that picks out an individual without describing it.
- A **definite description** picks out an individual by means of a unique description.
- A singular terms must refer to one specific thing in UD

The expression "a is P" is translated as $$P(a)$$, with $$a$$ denoting a constant. However, in the expression, $$P(x)$$, $$x$$ is a variable; because a variable ranges over all objects in UD, this means that all objects in UD have the property $$P$$.

Examples:
- "Anything is possible": $$\forall x Px$$
- "Unicorns are extinct": $$\lnot \exists x Ux$$ or $$\forall x \lnot Ux$$


## Universal statement
A universal statement is either affirmative or negative statement that makes an assertion about every member of its subject class.

- Universal statements are translated as _conditionals_.
- Variable are used to form a universal quantifier.
- e.g. "All $$S$$ are $$P$$" is translated as $$\forall x(Sx\to Px)$$.

For example, "_All bricks are thick_" can be symbolized as $$\forall x(Bx \to Tx)$$, meaning "_for all x: if x is a brick, then x is thick_".

A symbol that indicates that an assertion goes for all members is called **universal quantifier**, and it is introduced along with a variable, e.g. $$\forall x(Px \to Qx)$$



## Particular statement
A particular statement is a statement that makes an assertion about one or more unnamed members of the subject class.
- Particular statements are translated as _conjunctions_.
- e.g. "Some $$S$$ are $$P$$" is translated as $$\exists x(Sx\land Px)$$

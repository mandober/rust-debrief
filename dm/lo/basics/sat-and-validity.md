# Satisfiability and validity

Satisfiability and validity are related to each other in a manner exactly analogous to the _square of opposition_. A formula is:
- **satisfiable** if it is possible to find an interpretation that makes the formula true.
- **unsatisfiable** if none of the interpretations make the formula true.
- **valid** if it is true under every interpretation.
- **invalid** if some such interpretation makes the formula false.

https://www.wikiwand.com/en/Square_of_opposition

sat-and-validity.md

## Validity
In logic, an argument is valid iff it takes a form that makes it impossible for the premises to be true and the conclusion nevertheless to be false.
It is not required that a valid argument have premises that are actually true, but to have premises that, if they were true, would guarantee the truth of the argument's conclusion.
A formula is valid iff it is true under every interpretation.
An argument form (or schema) is valid iff every argument of that logical form is valid.

## Validity and deductive logic
Arguments in which the truth of the premises guarantees the truth of the conclusion are known as `valid`, 

while those where the truth of the premises makes the truth of the conclusion likely, but not certain, are called `inductively strong`.

These two properties, `validity` and `inductive strength`, have given rise to __deductive__ and _inductive_ logic, respectively.

The combination of true premises and validity actually has its own name: arguments that are valid and have true premises are called __sound arguments__.

Not all valid arguments are sound (since some of their premises could be false), but any sound argument is necessarily valid.

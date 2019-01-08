# Information Theory: Information Entropy

`L = {a,b,c,d}` alphabet of symbols

`m1`: probability of a symbol occuring = 0.25 for all symbols i.e.:
`m1 = {a:0.25, b:0.25, c:0.25, d:0.25}`

`m2`: probabilities of a symbol occuring
`m2 = {a:0.50, b:0.25, c:0.125, d:0.125}`

*Which machine is producing more information?*
i.e.
*What is te minimum number of yes/no question you have to ask to correctly predict the next symbol?*

For m1 it is 2 questions:
```
(symbol == AB)
  ? { (symbol == A) ? a : b }
  : { (symbol == C) ? c : d }
```
e.g.:
1. Is it A or B?
2. Is it C?

We always have to ask 2 questions to predict the symbol correctly.
**So, the uncertainty of m1 is 2 questions per symbol**. entropy H = 2q/s

---
For m1, we have to ask 1-3 questions.
```
1. (symbol == "A") ? a 
2.  : (symbol == "B") ? b
3.      : (symbol == "C") ? c : d
```
e.g.
1. Is it A? (A=50%)
2. Is it B? (B=25%)
3. Is it C? (C=D=12.5%)

So, *on average*, how many question do you have to ask to predict the next symbol of m2?

We can construct such machine by placing pegs that deflect a ball, with equal probability, either to left or to the right:

```
   o
A      o
    B      o
        C     D
```

* To get A, that m2 generates 50% of the time, we need 1 peg (1 question) that deflects the ball to the "A" box half of the time.
* To get B, we need 2 pegs/questions.
* To get C or D we need 3 pegs/questions.

So, taking a weighted average as:
```
m2: #bounces = Pa*1 + Pb*2 + Pc*3 + Pd*3
m2: #bounces = 0.5*1 + 0.25*2 + 0.125*3 + 0.125*3
m2: #bounces = 0.5 + 0.5 + 0.375 + 0.375
m2: #bounces = 1.75
```
The expected number of bounces is equal to the sum of weighted probabilities of each outcome.
The result is 1.75 (bounces/questions)!

So, *on average*, how many question do you have to ask to predict the next symbol of m2? **1.75**

As opposed to **2**, that is the average for m1.
```
m1: #bounces = Pa*2 + Pb*2 + Pc*2 + Pd*2
m1: #bounces = 0.25*2 + 0.25*2 + 0.25*2 + 0.25*2
m1: #bounces = 2
```

A machine requires N pegs to generate a symbol, while guessing an unkown symbol requires N questions.

Each question, like a peg, cuts in half the remaining number of possibilities.

So, if we need to guess 100 symbols, we'd ask 200 question on average for m1, and 175 question on average for m2.

> Therefore, m2 is producing **LESS INFORMATION**! m2 creates less uncertainty (or surprise) about its output.

---

C. Shannon calls the measure of average uncertainty **ENTROPY**, and donotes it as `H`. He chooses to call the unit of entropy **bit**.

`H = \sum^n_{i=1} p_i \times NumOfBounces`
Entropy, H, is the summation for each symbol of the probability of that symbol times the number of bounces.

`NumOfBounces = log_2{NumOfOutcomes}`
The number of bounces is logarithm, base two, of the number of oucomes at that level.

`NumOfOutcomes = 1 / p`
The number of outcomes at a level is 1 divided by the number of probability of that outcome occuring.

---

All together now:
`H = \sum^n_{i=1} p_i \times log_2{1/p_i}`

**Entropy**, `H`, is the summation, for each symbol, of the probability of that symbol times the logarithm base two of 1 over the probability of that symbol.


which C.Shannon writes (equaly) as:
`H = - \sum^n_{i=1} p_i \times log_2{p_i}`
(negates the summation to get rid of the fraction)

Entropy is maximum when all outcomes are equaly likely.

Any time you move away from equaly likely outcomes, or introduces predictability, the entropy must decrease.

When entropy of an information source declines, we can pose fewer questions to get the outcome.

The **bit** is a measure of information (or a measure of surprise).



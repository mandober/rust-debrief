# Mathematical induction


**Principle of Induction**:    
Let $$P(n)$$ be a statement about the positive integer $$n$$. 
In order to show that $$P(n)$$ is true for all positive integers $$n$$, it suffices to show that:
1. show that $$P(1)$$ is true
2. if $$P(n)$$ is true for an integer $$n$$, then show it is also true for the next integer $$P(n+1)$$, denoted by $$P(n) \to P(n+1)$$


**Example**:    
Prove that the sum of odd integers yields squares.

Conjecture: $$1+3+5+\dots + (2n−1) = n^2$$

Proof by induction:
1. $$P(1)$$ is true, $$1 = 1$$
2. Assume $$P(n)$$ is true and show that $$P(n+1)$$ is also true.    

The idea is to start with the formula $$P(n)$$. Add the next odd number 
$$2(n+1)−1$$ to both sides, and see to transform the equation into $$P(n+1)$$


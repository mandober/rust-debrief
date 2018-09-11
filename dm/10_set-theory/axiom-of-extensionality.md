# Axiom of extensionality


## Intro
https://www.wikiwand.com/en/Axiom_of_extensionality

In axiomatic set theory and the branches of logic, mathematics, and computer science that use it, the axiom of extensionality, or axiom of extension, is one of the axioms of Zermelo–Fraenkel set theory.

## Formal statement
In the formal language of the Zermelo–Fraenkel axioms, the axiom reads:

$$\forall A\,\forall B\,(\forall X\,(X\in A\iff X\in B)\to A=B)$$

- Given any set $$A$$ and any set $$B$$
- **if**
  - for every set $$X$$ holds that
  - $$X$$ is a member of $$A$$ **iff** $$X$$ is a member of $$B$$    
    (i.e. A and B have the same elements)
- **then** 
  - $$A$$ is equal to $$B$$


It is not really essential that X here be a set — but in ZF, everything is.

The converse of this axiom follows from the substitution property of equality:
$$\forall A\,\forall B\,(A=B\to \forall X\,(X\in A\iff X\in B))$$, 

- Given any set $$A$$ and any set $$B$$
- **if**
  - $$A$$ is equal to $$B$$
- **then** 
  - for every set $$X$$ holds that
  - $$X$$ is a member of $$A$$ **iff** $$X$$ is a member of $$B$$



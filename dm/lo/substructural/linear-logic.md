# Linear logic

https://en.wikipedia.org/wiki/Linear_logic
https://plato.stanford.edu/entries/logic-linear/
https://ncatlab.org/nlab/show/linear+logic



**Linear logic** is a substructural logic proposed by  **Jean-Yves Girard** as a refinement of classical and intuitionistic logic, joining the dualities of the former with many of the constructive properties of the latter.


Ideas from linear logic have been influential in design of programming languages because of its emphasis on resource-boundedness, duality, and interaction.


Linear logic lends itself to many different presentations, explanations and intuitions.

Proof-theoretically, it derives from an analysis of classical sequent calculus in which uses of (the structural rules) contraction and weakening are carefully controlled.

Operationally, this means that logical deduction is no longer merely about an ever-expanding collection of persistent "truths", but also a way of manipulating resources that cannot always be duplicated or thrown away at will.

In terms of simple denotational models, linear logic may be seen as refining the interpretation of intuitionistic logic by replacing cartesian closed categories by symmetric monoidal categories, or the interpretation of classical logic by replacing boolean algebras by C*-algebras.


---



Linear logic
- useful for reasoning about resources
- each hypothesis must be used exactly once
- very different from the normal understanding of logic
- classical and intuitionistic variant


**Syntax**:     
$$F := F \oplus F
  \mid 1 
  \mid F \& F 
  \mid T 
  \mid F \otimes F 
  \mid 0 
  \mid F \multimap F 
  \mid\ !F
$$


**Meaning**:    
$$\alpha \otimes \beta$$: $$\alpha$$ and $$\beta$$ hold simultaneously    
$$\quad \quad 1$$: nothing holds    
$$\alpha \& \beta$$: $$\alpha$$ and $$\beta$$ hold not necessarily simultaneously    
$$\quad \quad \top$$: tautology    
$$\alpha \oplus \beta$$: $$\alpha$$ and $$\beta$$ hold    
$$\quad \quad 0$$: absurdity    
$$\alpha \multimap \beta$$: if $$\alpha$$ holds, then $$\beta$$ holds    
$$\quad \quad !\alpha$$: $$\alpha$$ holds arbitrarily often    


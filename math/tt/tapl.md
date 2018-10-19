# TAPL

Types and programming languages, the book by B.Pierce


Syntax - BNF notation:
```
t ::=                   terms:
  true                    constant true
  false                   constant false
  0                       constant zero
  if t then t else t      conditional
  succ t                  successor
  pred t                  predecessor
  iszero t                zero test
```

Syntax - Terms, inductively:
The set of terms is the smallest set $$\mathcal{T}$$ such that:
1. $$\{true,false,0\}\subseteq\mathcal{T}$$;
2. $$if\ t_1\in \mathcal{T},\ then\ \{succ\ t_1,\ pred\ t_1,\ iszero\ t_1\}\subseteq\mathcal{T}$$;
3. $$if\ t_1\in \mathcal{T}, t_2\in\mathcal{T}, t_3\in\mathcal{T}\ then\ if\ t_1\ then\ t_2\ else\ t_3 \in \mathcal{T}$$

# Matrix

A matrix is a rectangular array of numbers (or symbols or expressions) arranged 
in rows and columns.

```
┌       ┐
│ 1 2 3 │
│ µ ∑ ∂ │
└       ┘
```

The dimensions of this matrix are 2x3 ("two by three"). In notations concerning matrices, the rows are always first, the columns last.

Individual items in an `n×m` matrix A, denoted `a[i,j]` (`i <= n, j <= m`) are called its elements or entries.

## Identity matrix

Identity matrix is a square matrix in which all the elements of the main (major, principal, leading) diagonal are ones and all other elements are zeros.

```
               ┌     ┐   ┌       ┐
I¹ = 1 = I² =  │ 1 0 │ = │ 1 0 0 │ = ...
               │ 0 1 │   │ 0 1 0 │
               └     ┘   │ 0 0 1 │
                         └       ┘

```

The effect of multiplying a given matrix by an identity matrix is to leave the given matrix unchanged.
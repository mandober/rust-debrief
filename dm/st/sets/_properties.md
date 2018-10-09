# Set properties

$$A$$ and $$B$$
A∪(B∪C)=(A∪B)∪C, A∩(B∩C)=(A∩B)∩C
A∪(B∩C)=(A∪B)∩(A∪C), A∩(B∪C)=(A∩B)∪(A∩C)

A={a,b,c,d}
B={c,d,e,f}
A ∪ B = {a,b,c,d,e,f}
A ∩ B = {c,d}
A - B = {a,b} ≠ B - A = {e,f}
C(A)={}

op = ∪, ∩ set operartion

## Set properties
- **Commutativity**: $$A\cup B \equiv B\cup A$$
- **Associativity**, e.g. $$(A\cup B)\cup C \equiv A\cup (B\cup C)$$
- **Distributivity**, e.g. $$A \cup (B \cap C) \equiv (A \cup B) \cap (A\cup C)$$
- **Idempotency**: $$\forall A: A\cup A \equiv A, A\cap A \equiv A$$
- **Identity**: $$\forall A:\ A\cup \varnothing = A, A\cap \varnothing = \varnothing, A\cup \mathcal{U}=\mathcal{U}, A\cap \mathcal{U}=A$$
- **Transitivity**: $$(A \le B \le C) \to (A \le C)$$
- **Involution**: $$\forall A:A \equiv \lnot \lnot A$$
- **De Morgan's Law**: supports in proving tautologies and contradiction.



# Closure property
A set has closure under an operation if performance of that operation on members of the set always produces a member of that, same, set. In this case we also say that the set is closed under that operation.

Similarly, a set is said to be closed under a collection of operations if it is closed under each of the operations individually.

For example, the positive integers are closed under addition, but not under subtraction; a set containing only zero is closed under addition, subtraction and multiplication.

A set that is closed under an operation or collection of operations is said to satisfy a _closure property_.

When a set S is not closed under some operations, one can usually find the smallest set containing S that is closed. This smallest closed set is called the closure of S (with respect to these operations).

For example, the closure under subtraction of the set of natural numbers, viewed as a subset of the real numbers, is the set of integers. The set S must be a subset of a closed set in order for the closure operator to be defined. In this example, it is important that the reals are closed under subtraction.

The two uses of the word "closure" should not be confused. The former usage refers to the property of being closed, and the latter refers to the smallest closed set containing one that may not be closed. In short, the closure of a set satisfies a closure property.

A set is closed under an operation if the operation returns a member of the set when evaluated on members of the set. Sometimes the requirement that the operation be valued in a set is explicitly stated, in which case it is known as the __axiom of closure__. For example, one may define a group as a set with a binary product operator obeying several axioms, including an axiom that the product of any two elements of the group is again an element.

Closure on a set does not necessarily imply closure on all subsets. Thus a subgroup of a group is a subset on which the binary product and the unary operation of inversion satisfy the closure axiom.


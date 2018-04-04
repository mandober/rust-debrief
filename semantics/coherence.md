# Coherence

Coherence guarantees that there is always a single, unambiguous implementation of a trait that is used for any given type.

Coherence rules:
1. The orphan rule - either trait definition or the implementing type has to be local to the crate.
2. The overlap rule - a trait cannot have two implementations that both apply to the same type (which would introduce ambiguity about which impl to use), unless one is a specialization of the other.

These rules work closely together; in particular, the orphan rule ensures that sibling crates can’t accidentally define overlapping impls for a parent trait, since their impls must each involve crate-local types.

Current design was established with the "Rebalancing coherence RFC", which was predicated on a core assumption: The problem is that due to coherence, the ability to define impls is a zero-sum game: every impl that is legal to add in a child crate is also an impl that a parent crate cannot add without fear of breaking downstream crates.

However, with specialization, it seems this assumption may not longer hold, The point of specialization is to allow for impls to overlap, and then to select the “most specific” impl.


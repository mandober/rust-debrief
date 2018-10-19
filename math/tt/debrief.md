# Type Theory: Debriefing


- in tt, every term has a type
- ops are restricted to terms of a certain type
- two well-known type theories (can serve as foundations for math):
  - _Typed Lambda Calculus_ by Alonzo Church
  - _Intuitionistic Type Theory_ by Per Martin-Löf
- in tt concepts like "and" and "or" can be encoded as types
- terms generally belong to only one type
- when subset is needed, tt creates a new _dependent sum type_ with new terms
- union is similarly achieved by a new sum type and new terms
- types that combine unrelated types do so by creating new terms

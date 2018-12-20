# PL Design

Design principles

- Abstraction
  Desirable to have constructs that factor out recurring patterns.
  e.g. subprocedures
- Automation
  Automate mechanical, tedious, or error-prone activities.
- Defense in Depth
  Have a series of defenses so that if an error is not caught by one, it will probably be caught by another.
- Information Hiding
  The language should permit modules to be designed so that the user has all the information needed to use the module correctly and nothing more, and the implementor has all the information needed to implement the module correctly and nothing more.
- Labeling
  Avoid arbitrary sequences more than a few items long; do not require the user to know the absolute position of an item in a list. Instead, associate a meaningful label with each item and allow the items to occur in any order.
- Localized cost
  Users should only pay for what they use; avoid distributed costs.
- Manifest Interface
  All interfaces should be apparent (manifest) in the syntax.
- Orthogonality
  Basic features should be separately understandable and free from unexpected interaction. e.g. Pascal val and var parameters combine the type of use of parameters (input/output) with the parameter-passing mechanism (call-by-value/call-by-reference).
- Portability
  Avoid features or facilities that are dependent on a particular machine or a small class of machines.
- Preservation of Information
  The language should allow the representation of information that the user might know and that the compiler might need.
- Simplicity
  The fewer concepts to understand in a language the better.
- Regularity
  The fewer there are exceptions to the rules the better.
- Security
  No program that violates the definition of the language, or its own intended structure, should escape detection.
- Structure
  The static structure of the program should correspond in a simple way with the dynamic structure of the corresponding computations.
- Syntactic Consistency
  Similar constructs should look similar, different constructs should appear differently.
- Translation
  Language translator must run quickly and produce efficient object code.
- Zero-One-Infinity
  The only reasonable numbers are zero, one, and infinity.


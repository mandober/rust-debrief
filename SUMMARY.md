# Summary

[Road to Rust](./README.md)


## Part I Mathematics

* [math](./math/README.md)
* [math.dm](./math/am/README.md)
  - [Mathematics](./math/am/01_math.md)
  - [Foundations of mathematics](./math/am/01_math.md)
  - [Discrete mathematics](./math/am/02_dm.md)
  - [Logic: Timeline](./math/am/math-timeline.md)
  - [Mathematical entities](./math/am/entities.md)
* [math.nt](./math/nt/README.md)
  - Number theory
    - Natural numbers
    - Integers
    - Rational numbers
    - Real numbers
  - Counting theory
    - Fundamental counting rules
    - Complex numbers
    - Combinatorics
    - Permutations
* [math.st](./math/st/README.md)
  - [Sets](./math/st/sets/README.md)
    - [Overview](./math/st/sets/01_overview.md)
    - [Specification](./math/st/sets/02_specification.md)
    - [Membership](./math/st/sets/03_membership.md)
    - [Cardinality](./math/st/sets/04_cardinality.md)
    - [Set operations](./math/st/sets/06_set-operations.md)
    - Subset, powerset, Cartesian product
    - tuples, pairs
    - Fundamental sets
  - Set theory
    - Naïve set theory
    - Axiomatic set theory
    - ZFC axioms
  - Relations
    - Order theory
  - Functions
    - Types of functions
* [math.lo](./math/lo/README.md)
  - Mathematical Logic
  - Term logic
  - Propositional logic
  - Predicate logic
* [math.pt](./math/pt/README.md)
* [math.tc](./math/tc/README.md)
* [math.lc](./math/lc/README.md)
* [math.at](./math/at/README.md)
* [math.tt](./math/tt/README.md)
* [math.ct](./math/ct/README.md)


## Part II CS

* [cs](./cs/README.md)
  * Computer architecture
  * Components
  * Memory
  * Data representation
  * Programming
    - Declaration
    - Definition
    - Assignment
    - Binding
  * Type Systems
    - Data
    - Types
    - Typing discipline
  * [Memory](./cs/memory/README.md)
    - [Memory Management](./cs/memory/memory-management-levels.md)
    - Memory Management at the hardware level
    - [Memory hierachy](cs/memory/memory-hierarchy.md)
    - [Memory types](cs/memory/memory-types.md)
    - [Cache](cs/memory/cache.md)
    - [Memory address register](cs/memory/mar.md)
    - [Registers](cs/memory/registers.md)
    - [Word](cs/memory/word.md)
    - Memory Management at the OS level
    - [Memory model](cs/memory/memory-model.md)
    - [Memory protection](cs/memory/memory-protection.md)
    - [Virtual memory](cs/memory/virtual-memory.md)
    - [Segmentation fault](cs/memory/segmentation-fault.md)
    - [Address space](cs/memory/address-space.md)
    - [Memory address](cs/memory/memory-address.md)
    - [Physical address](cs/memory/physical-address.md)
    - Memory Management at the program level
    - Manual memory management
    - Garbage collection
    - [Memory allocation](cs/memory/memory-allocation.md)
    - Stack based allocation
    - Dynamic memory allocation
    - [Memory safety](cs/memory/memory-safety.md)
    - [Memory leak](cs/memory/memory-leak.md)
    - [Program memory](cs/memory/program-memory.md)


## Part III Rust

* [rust](./rust/README.md)
  - Rust Syntax
  - [Attributes](./rust/syntax/attributes.md)
  - [Comments](./rust/syntax/comments.md)
  - [Conventions](./rust/syntax/conventions.md)
  - [Expressions](./rust/syntax/expressions.md)
  - [Control flow](./rust/syntax/control-flow.md)
  - [Formatting output](./rust/syntax/format.md)
  - [Fully Qualified Syntax](./rust/syntax/fully-qualified-syntax.md)
  - [Grammar](./rust/syntax/grammar.md)
  - [Keywords](./rust/syntax/keywords.md)
  - [Literals](./rust/syntax/literals.md)
  - [Operators](./rust/syntax/operators.md)
  - [Syntactic elements](./rust/syntax/syntactic-elements.md)
  - Concepts
  - Lifetimes
  - [Iterators](./rust/concepts/iterators/iterators.md)
  - Primitives
  - [Character](./rust/primitives/char/char.md)
  - [Slices](./rust/primitives/slice/slice.md)
  - [Unit](./rust/primitives/unit/unit.md)
  - [Never](./rust/primitives/never/never.md)
  - Rust Types
  - Classification
  - Annotation
  - Modules
  - [collections](./rust/modules/collections/README.md)
  - [option](./rust/modules/option/README.md)
  - [any](./rust/modules/any/any.md)
    - [any](./rust/modules/any/any-trait.md)
  - [iter](./rust/modules/iter/README.md)
    - [methods](./rust/modules/iter/methods-all.md)
  - [box](./rust/modules/boxed/box.md)
  - Traits
  - [Traits](./rust/traits/README.md)
  - Index by module
  - Derivable traits
  - Trait resolution
  - Items
  - [Lang Items](./rust/items/README.md)
  - Macros
  - [Macros](./rust/macros/macro.md)
  - [Declarative macros](./rust/macros/macro-declerative.md)
  - [Procedural macros](./rust/macros/macro-procedural.md)
  - [Index of std macros](./rust/macros/macro-index.md)
  - Rust Core
  - [The compiler](./rust/core/compiler.md)
  - [Trait resolution](./rust/core/rustc/trait-resolution.md)
  - [Type inference engine](./rust/core/rustc/type-inference-engine.md)
  - [Types and the Type Context](./rust/core/rustc/types-and-the-type-context.md)
  - [MIR](./rust/core/rustc/mir.md)
  - [HIR](./rust/core/rustc/hir.md)
  - [The Borrow Checker](./rust/core/rustc/borrow-checker.md)
  - [Variance of type and lifetime parameters](./rust/core/rustc/variance-of-type-and-lifetime-parameters.md)


## Part IV Appendix

* [Appendix](./apx/README.md)
  * Taxonomy
    - Math Taxonomy
    - Logic Taxonomy
    - CS Taxonomy
  * [Links](./apx/links/README.md)
    - Index of Links
    - DM Links
    - CS Links
    - Rust Links
  * [Index](./apx/index/README.md)
    - Index of topics
    - Index of terms
    - Index of figures
    - Index of symbols
      - Greek alphabet
      - Math symbols
      - Set theory symbols
      - Logic symbols
  - [Glossary](./apx/glossary.md)
    - [Logic glossary](./apx/glossary_dm.md)
    - [Math glossary](./apx/glossary_dm.md)
  - [Abbreviations](./apx/abbreviations.md)
  - [Bibliography](./apx/bibliography.md)
  - [History](./apx/history.md)
  - [People](./apx/people.md)
  - [Links](./apx/links/README.md)
  * [Glossary](./apx/glossary.md)
  * [Abbreviations](./apx/abbreviations.md)

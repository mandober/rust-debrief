# Grammar

<!-- TOC -->

- [Notation](#notation)
- [Grammar productions](#grammar-productions)

<!-- /TOC -->


## Notation
- Rust's grammar is defined over Unicode codepoints, each denoted `U+XXXX`. 
- Most of grammar is confined to the ASCII range of Unicode.
- Grammar is described by _Extended Backus-Naur Form (EBNF)_, i.e. by EBNF dialect supported by common automated LL(k) parsing tools such as `llgen`(rather than the dialect given in ISO 14977)


Unicode productions
A few productions in Rust's grammar permit Unicode code points outside the ASCII range. We define these productions in terms of character properties specified in the Unicode standard, rather than in terms of ASCII-range code points. The grammar has a Special Unicode Productions section that lists these productions.

String table productions
Some rules in the grammar — notably unary operators, binary operators, and keywords — are given in a simplified form: as a listing of printable strings. These cases form a subset of the rules regarding the token rule, and are assumed to be the result of a lexical-analysis phase feeding the parser, driven by a DFA, operating over the disjunction of all such string table entries.

When such a string in monospace font occurs inside the grammar, it is an implicit reference to a single member of such a string table production. See tokens for more information.


## Grammar productions
- Tokens are primitive productions defined by regular, non-recursive, language.
- literal is a compile-time evaluated expression that immediately and directly denotes the value it evaluates to.
- Symbols are printable structural tokens: `:: -> # [ ] ( ) { } , ;`
- An item is a component of a crate
- Items are organized within a crate by a nested set of modules.


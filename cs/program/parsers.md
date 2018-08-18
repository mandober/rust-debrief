# Parser

https://tomassetti.me/parsing-in-javascript/
http://matt.might.net/articles/grammars-bnf-ebnf/
https://github.com/antlr/grammars-v4
https://sap.github.io/chevrotain/playground/
https://www.codeproject.com/Articles/1165963/PEG-Parser-Combinators-Implemented-in-Rust
https://github.com/jbclements/rust-antlr/
https://news.ycombinator.com/item?id=13819830
https://tomassetti.me/antlr-mega-tutorial/
https://www.wikiwand.com/en/Comparison_of_parser_generators
https://github.com/pest-parser/pest
https://github.com/jbclements/rust-antlr/
https://tomassetti.me/category/language-engineering/parsing/


A parser takes a piece of text and transforms it in an organized structure, such as an Abstract Syntax Tree (AST). The AST can be thought of as a story describing the content of the code or as its logical representation, created by putting together the various pieces.

## ANTLR parser
To generate an AST:
- Define a lexer and parser grammar
- ANTLR will generate a lexer and a parser in the target language
- Feed the source text to lexer/parser for processing: they generate the AST
- The source can be the code of a programming language, but generally it can be any kind of structured text.

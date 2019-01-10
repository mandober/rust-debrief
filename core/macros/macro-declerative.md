# Declarative macros





## Definition

Declarative macros are also referred to as macros by example, `macro_rules!` macros or just macros. Currently, macros can expand to expressions, statements, items, or patterns.

```rust
// overview
macro_rules! NAME {
  ( PATTERN ) => { BLOCK };
};

// details
macro_rules! NAME {
  ( $(PATTERN),* ) => { $(BLOCK),* };
};

// valid macro
macro_rules! mac {
  ( $( $i:ident ),* ) => ( $( $i:ident ),* )
}

// invalid macro: the block doesn't contain repetition quantifier
macro_rules! mac {
  ( $( $i:ident ),* ) => ( $i )
}
```


- keyword `macro_rules!`
- NAME: name of macro (without`!`). `!` is used to invoke a macro 
- PATTERN: pattern to be matched
- BLOCK: if the pattern matches this block is evaluated and emitted




Here is a simplified definition of the `vec!` macro:

```rust
#[macro_export]
macro_rules! vec {
  ( $( $x:expr ),* ) => {
    { // inside block
    
      let mut temp_vec = Vec::new(); // evaluate once
      
      $( // repeat
          temp_vec.push($x); // this expression
      )* // zero or more times
      
      temp_vec // evaluate once and return
      
    } // end inside block
  }; // end emitted block
} // end macro body and definition
```

This macro definition has one arm with the pattern `( $( $x:expr ),* ),` followed by => and the block of code associated with this pattern. If this pattern matches, then the block of code will be emitted. Given that this is the only pattern in this macro, there’s only one valid way to match; any other will be an error. More complex macros will have more than one arm.





### Declarative
(from Rust Reference)

- `sep_token` is any token other than `*` and `+`
- `non_special_token` is any token other than a delimiter or `$`
- expander transcribes every token that doesn't begin with a `$`


The macro expander looks up macro invocations by name, and tries each macro rule in turn. It transcribes the first successful match. The macro expander matches and transcribes every token that does not begin with a `$`, including delimiters. Delimiters must be balanced, but they are otherwise not special.

In the matcher, `$` _name_ `:` _designator_ matches the nonterminal in the Rust syntax named by _designator_. Valid designators are:
- `item`: an item
- `block`: a block
- `stmt`: a statement
- `pat`: a pattern
- `expr`: an expression
- `ty`: a type
- `ident`: an identifier
- `path`: a path
- `tt`: a token tree (a single token by matching (), [], or {})
- `meta`: the contents of an attribute

In the transcriber, the designator is already known, and so only the name of a matched nonterminal comes after the dollar sign.

In both the matcher and transcriber, the Kleene star-like operator indicates repetition. The Kleene star operator consists of:
- `$` and parentheses
- optionally followed by a separator token
- followed by `*` or `+` (`*` zero or more, `+` zero or one repetition)
- The parentheses are not matched or transcribed. 

On the matcher side, a name is bound to all of the names it matches, in a structure that mimics the structure of the repetition encountered on a successful match. The job of the transcriber is to sort that structure out.

The rules for transcription of these repetitions are called "Macro By Example". Essentially, one "layer" of repetition is discharged at a time, and all of them must be discharged by the time a name is transcribed.

Therefore, `( $( $i:ident ),* ) => ( $i )` is an invalid macro, but 
`( $( $i:ident ),* ) => ( $( $i:ident ),* )` is acceptable.

When Macro By Example encounters a repetition, it examines all of the `$` names that occur in its body. At the "current layer", they all must repeat the same number of times, so `( $( $i:ident ),* ; $( $j:ident ),* ) => ( $( ($i,$j) ),* )` is valid if given the argument `(a,b,c ; d,e,f)`, but not `(a,b,c ; d,e)`.

The repetition walks through the choices at that layer in lockstep, so the former input transcribes to `(a,d), (b,e), (c,f)`. Nested repetitions are allowed.


The parser used by the macro system is reasonably powerful, but the parsing of Rust syntax is restricted in two ways:
1. Macro definitions are required to include suitable separators after parsing expressions and other bits of the Rust grammar. This implies that a macro definition like `$i:expr [ , ]` is not legal, because `[` could be part of an expression. A macro definition like `$i:expr,` or `$i:expr;` would be legal, however, because `,` and `;` are legal separators. See [RFC 550](https://github.com/rust-lang/rfcs/blob/master/text/0550-macro-future-proofing.md) for more information.
2. The parser must have eliminated all ambiguity by the time it reaches a 
`$ name : designator`. This requirement most often affects name-designator pairs when they occur at the beginning of, or immediately after, a `$(...)*;` requiring a distinctive token in front can solve the problem.



### Procedural Macros

Procedural macros allow creating syntax extensions as execution of a function. Procedural macros can be used to implement custom derive on your own types. See the book for a tutorial.

Procedural macros involve a few different parts of the language and its standard libraries. First is the proc_macro crate, included with Rust, that defines an interface for building a procedural macro. The #[proc_macro_derive(Foo)] attribute is used to mark the deriving function. This function must have the type signature:


use proc_macro::TokenStream;

#[proc_macro_derive(Hello)]
pub fn hello_world(input: TokenStream) -> TokenStream
Finally, procedural macros must be in their own crate, with the proc-macro crate type.
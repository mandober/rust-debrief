# Rust RFC

RFC 2000 "Const Generics"
https://github.com/rust-lang/rfcs/blob/master/text/2000-const-generics.md
Feature Name: const_generics
Start Date: 2017-05-01
RFC PR: rust-lang/rfcs#2000
Rust Issue: rust-lang/rust#44580

Summary
Allow types to be generic over constant values; among other things this will allow users to write impls which are abstract over all array types.

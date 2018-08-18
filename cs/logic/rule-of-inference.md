# Rule of inference

[Source](https://en.wikipedia.org/wiki/Rule_of_inference)



In [logic][1], 
a **rule of inference**,
**inference rule** or
**transformation rule** 
is a [logical form][2] consisting of 
a function which takes premises, 
analyzes their [syntax][3], 
and returns a conclusion (or [conclusions][4]).

For example, the rule of inference called [_modus ponens][5]_ 
takes two premises, 
one in the form 
  "If p then q" and another in the form "p", 
and returns the conclusion "q". 
The rule is valid with respect to the semantics of [classical logic][6] (as well as the semantics of many other [non-classical logics][7]), 
in the sense that if the premises are true (under an interpretation), 
then so is the conclusion. 

Typically, a rule of inference preserves truth, a semantic property. 
In [many-valued logic][8], it preserves a general designation. 
But a rule of inference's action is purely syntactic, 
and does not need to preserve any semantic property: 
any function from sets of formulae to formulae 
counts as a rule of inference. 
Usually only rules that are [recursive][9] are important;
  i.e. rules such that there is an [effective procedure][10] 
  for determining whether any given formula 
  is the conclusion of a given set of formulae 
  according to the rule. 
  An example of a rule that is not effective in this sense 
  is the infinitary [ω-rule][11].[[1]][12]

Popular rules of inference in [propositional logic][13] include
  [_modus ponens][5]_, 
  [_modus tollens][14]_, and 
  [contraposition][15]. 

First-order [predicate logic][16] uses rules of inference 
to deal with [logical quantifiers][17]. 


## The standard form of rules of inference[[edit][18]]

In [formal logic][19] (and many related areas), 
rules of inference are usually given in the following standard form:

```text
Premise 1
Premise 2
.
.
.
Premise n
Conclusion
```

This expression states that 
whenever in the course of some logical derivation
the given premises have been obtained, 
the specified conclusion can be taken for granted as well. 
The exact formal language that is used to describe 
both premises and conclusions 
depends on the actual context 
of the derivations.

In a simple case, one may use logical formulae, such as in: 

$$
{\displaystyle A\to B} A\to B
{\displaystyle {\underline {A\quad \quad \quad }}\,\!} \underline {A\quad \quad \quad }\,\!
{\displaystyle B\!} B\!
$$

: A→B{displaystyle Ato B}![Ato B][20]

: A_{displaystyle {underline {Aquad quad quad }},!}![underline {Aquad quad quad },!][21]

: B{displaystyle B!}![B!][22]

This is the [_modus ponens][5]_ rule of [propositional logic][13]. Rules of inference are often formulated as [schemata][23] employing [metavariables][24].[[2]][25] In the rule (schema) above, the metavariables A and B can be instantiated to any element of the universe (or sometimes, by convention, a restricted subset such as [propositions][26]) to form an [infinite set][27] of inference rules. 

A proof system is formed from a set of rules chained together to form proofs, also called _derivations_. Any derivation has only one final conclusion, which is the statement proved or derived. If premises are left unsatisfied in the derivation, then the derivation is a proof of a _hypothetical_ statement: "_if_ the premises hold, _then_ the conclusion holds." 

## Example: Hilbert systems for two propositional logics[[edit][28]]

In a [Hilbert system][29], the premises and conclusion of the inference rules are simply formulae of some language, usually employing metavariables. For graphical compactness of the presentation and to emphasize the distinction between axioms and rules of inference, this section uses the sequent notation (⊢) instead of a vertical presentation of rules. 

The formal language for classical [propositional logic][13] can be expressed using just negation (¬), implication (→) and propositional symbols. A well-known axiomatization, comprising three axiom schemata and one inference rule (_modus ponens_), is: 
    
    
    (CA1) ⊢ _A_ → (_B_ → _A_)  
    (CA2) ⊢ (_A_ → (_B_ → _C_)) → ((_A_ → _B_) → (_A_ → _C_))  
    (CA3) ⊢ (¬_A_ → ¬_B_) → (_B_ → _A_)  
    (MP)  _A_, _A_ → _B_ ⊢ _B_
    

It may seem redundant to have two notions of inference in this case, ⊢ and →. In classical propositional logic, they indeed coincide; the [deduction theorem][30] states that _A_ ⊢ _B_ if and only if ⊢ _A_ → _B_. There is however a distinction worth emphasizing even in this case: the first notation describes a [deduction][31], that is an activity of passing from sentences to sentences, whereas _A_ → _B_ is simply a formula made with a [logical connective][32], implication in this case. Without an inference rule (like _modus ponens_ in this case), there is no deduction or inference. This point is illustrated in [Lewis Carroll][33]'s dialogue called "[What the Tortoise Said to Achilles][34]" [[3]][35], as well as later attempts by [Bertrand Russel and Peter Winch][36] to resolve the paradox introduced in the dialogue. 

For some non-classical logics, the deduction theorem does not hold. For example, the [three-valued logic][37] of [Łukasiewicz][38] can be axiomatized as:[[4]][39]
    
    
    (CA1) ⊢ _A_ → (_B_ → _A_)  
    (LA2) ⊢ (_A_ → _B_) → ((_B_ → _C_) → (_A_ → _C_))  
    (CA3) ⊢ (¬_A_ → ¬_B_) → (_B_ → _A_)  
    (LA4) ⊢ ((_A_ → ¬_A_) → _A_) → _A_  
    (MP)  _A_, _A_ → _B_ ⊢ _B_
    

This sequence differs from classical logic by the change in axiom 2 and the addition of axiom 4. The classical deduction theorem does not hold for this logic, however a modified form does hold, namely _A_ ⊢ _B_ if and only if ⊢ _A_ → (_A_ → _B_).[[5]][40]

## Admissibility and derivability[[edit][41]]

In a set of rules, an inference rule could be redundant in the sense that it is _admissible_ or _derivable_. A derivable rule is one whose conclusion can be derived from its premises using the other rules. An admissible rule is one whose conclusion holds whenever the premises hold. All derivable rules are admissible. To appreciate the difference, consider the following set of rules for defining the [natural numbers][42] (the [judgment][43] nnat{displaystyle n,,{mathsf {nat}}}![n,,mathsf{nat}][44] asserts the fact that n{displaystyle n}![n][45] is a natural number): 

: 0natnnats(n)nat{displaystyle {begin{matrix}{frac {}{mathbf {0} ,,{mathsf {nat}}}}&amp;{frac {n,,{mathsf {nat}}}{mathbf {s(} nmathbf {)} ,,{mathsf {nat}}}}\end{matrix}}}![
begin{matrix}
frac{}{mathbf{0} ,,mathsf{nat}} &amp;
frac{n ,,mathsf{nat}}{mathbf{s\(}nmathbf{\)} ,,mathsf{nat}} \
end{matrix}
][46]

The first rule states that **0** is a natural number, and the second states that **s(n)** is a natural number if _n_ is. In this proof system, the following rule, demonstrating that the second successor of a natural number is also a natural number, is derivable: 

: nnats(s(n))nat{displaystyle {frac {n,,{mathsf {nat}}}{mathbf {s(s(} nmathbf {))} ,,{mathsf {nat}}}}}![
frac{n ,,mathsf{nat}}{mathbf{s\(s\(}nmathbf{\)\)} ,,mathsf{nat}}
][47]

Its derivation is the composition of two uses of the successor rule above. The following rule for asserting the existence of a predecessor for any nonzero number is merely admissible: 

: s(n)natnnat{displaystyle {frac {mathbf {s(} nmathbf {)} ,,{mathsf {nat}}}{n,,{mathsf {nat}}}}}![
frac{mathbf{s\(}nmathbf{\)} ,,mathsf{nat}}{n ,,mathsf{nat}}
][48]

This is a true fact of natural numbers, as can be proven by [induction][49]. (To prove that this rule is admissible, assume a derivation of the premise and induct on it to produce a derivation of nnat{displaystyle n,,{mathsf {nat}}}![n ,,mathsf{nat}][44].) However, it is not derivable, because it depends on the structure of the derivation of the premise. Because of this, derivability is stable under additions to the proof system, whereas admissibility is not. To see the difference, suppose the following nonsense rule were added to the proof system: 

: s(−3)nat{displaystyle {frac {}{mathbf {s(-3)} ,,{mathsf {nat}}}}}![
frac{}{mathbf{s\(-3\)} ,,mathsf{nat}}
][50]

In this new system, the double-successor rule is still derivable. However, the rule for finding the predecessor is no longer admissible, because there is no way to derive −3nat{displaystyle mathbf {-3} ,,{mathsf {nat}}}![mathbf{-3} ,,mathsf{nat}][51]. The brittleness of admissibility comes from the way it is proved: since the proof can induct on the structure of the derivations of the premises, extensions to the system add new cases to this proof, which may no longer hold. 

Admissible rules can be thought of as [theorems][52] of a proof system. For instance, in a [sequent calculus][53] where [cut elimination][54] holds, the _cut_ rule is admissible. 


## References

1. [**^][57]** Boolos, George; Burgess, John; Jeffrey, Richard C. (2007). _Computability and logic_. Cambridge: Cambridge University Press. p.&nbsp;364. [ISBN][58]&nbsp;[0-521-87752-0][59].&nbsp;
2. [**^][60]** John C. Reynolds (2009) [1998]. [_Theories of Programming Languages_][61]. Cambridge University Press. p.&nbsp;12. [ISBN][58]&nbsp;[978-0-521-10697-9][62].&nbsp;
3. [**^][63]** Kosta Dosen (1996). "Logical consequence: a turn in style". In Maria Luisa Dalla Chiara; Kees Doets; Daniele Mundici; Johan van Benthem. [_Logic and Scientific Methods: Volume One of the Tenth International Congress of Logic, Methodology and Philosophy of Science, Florence, August 1995_][64]. Springer. p.&nbsp;290. [ISBN][58]&nbsp;[978-0-7923-4383-7][65].&nbsp; [preprint (with different pagination)][66]
4. [**^][67]** Bergmann, Merrie (2008). _An introduction to many-valued and fuzzy logic: semantics, algebras, and derivation systems_. Cambridge University Press. p.&nbsp;100. [ISBN][58]&nbsp;[978-0-521-88128-9][68].&nbsp;
5. [**^][69]** Bergmann, Merrie (2008). _An introduction to many-valued and fuzzy logic: semantics, algebras, and derivation systems_. Cambridge University Press. p.&nbsp;114. [ISBN][58]&nbsp;[978-0-521-88128-9][68].&nbsp;

[1]: https://en.wikipedia.org/wiki/Logic "Logic"
[2]: https://en.wikipedia.org/wiki/Logical_form "Logical form"
[3]: /wiki/Syntax_(logic) "Syntax (logic)"
[4]: https://en.wikipedia.org/wiki/Multiple-conclusion_logic "Multiple-conclusion logic"
[5]: https://en.wikipedia.org/wiki/Modus_ponens "Modus ponens"
[6]: https://en.wikipedia.org/wiki/Classical_logic "Classical logic"
[7]: https://en.wikipedia.org/wiki/Non-classical_logic "Non-classical logic"
[8]: https://en.wikipedia.org/wiki/Many-valued_logic "Many-valued logic"
[9]: https://en.wikipedia.org/wiki/Recursion "Recursion"
[10]: https://en.wikipedia.org/wiki/Effective_procedure "Effective procedure"
[11]: https://en.wikipedia.org/wiki/%CE%A9-consistent_theory "Ω-consistent theory"
[12]: https://en.wikipedia.org#cite_note-1
[13]: https://en.wikipedia.org/wiki/Propositional_logic "Propositional logic"
[14]: https://en.wikipedia.org/wiki/Modus_tollens "Modus tollens"
[15]: https://en.wikipedia.org/wiki/Contraposition "Contraposition"
[16]: https://en.wikipedia.org/wiki/Predicate_logic "Predicate logic"
[17]: https://en.wikipedia.org/wiki/Logical_quantifier "Logical quantifier"
[18]: /w/index.php?title=Rule_of_inference&amp;action=edit§ion=1 "Edit section: The standard form of rules of inference"
[19]: https://en.wikipedia.org/wiki/Formal_logic "Formal logic"
[20]: https://wikimedia.org/api/rest_v1/media/math/render/svg/d5b8dd84619daff17b52a08b77d15db2b9ad6c2a
[21]: https://wikimedia.org/api/rest_v1/media/math/render/svg/aaae14f3e7411747757d5ca86f6816926fd30500
[22]: https://wikimedia.org/api/rest_v1/media/math/render/svg/b0862a1de92638c6dbf56966deeb873becc27ec3
[23]: /wiki/Schema_(logic) "Schema (logic)"
[24]: https://en.wikipedia.org/wiki/Metavariable "Metavariable"
[25]: https://en.wikipedia.org#cite_note-Reynolds2009-2
[26]: https://en.wikipedia.org/wiki/Proposition "Proposition"
[27]: https://en.wikipedia.org/wiki/Infinite_set "Infinite set"
[28]: /w/index.php?title=Rule_of_inference&amp;action=edit§ion=2 "Edit section: Example: Hilbert systems for two propositional logics"
[29]: https://en.wikipedia.org/wiki/Hilbert_system "Hilbert system"
[30]: https://en.wikipedia.org/wiki/Deduction_theorem "Deduction theorem"
[31]: https://en.wikipedia.org/wiki/Deductive_reasoning "Deductive reasoning"
[32]: https://en.wikipedia.org/wiki/Logical_connective "Logical connective"
[33]: https://en.wikipedia.org/wiki/Lewis_Carroll "Lewis Carroll"
[34]: https://en.wikipedia.org/wiki/What_the_Tortoise_Said_to_Achilles "What the Tortoise Said to Achilles"
[35]: https://en.wikipedia.org#cite_note-ChiaraDoets1996-3
[36]: https://en.wikipedia.org/wiki/What_the_Tortoise_Said_to_Achilles#Discussion "What the Tortoise Said to Achilles"
[37]: https://en.wikipedia.org/wiki/Three-valued_logic "Three-valued logic"
[38]: https://en.wikipedia.org/wiki/%C5%81ukasiewicz "Łukasiewicz"
[39]: https://en.wikipedia.org#cite_note-4
[40]: https://en.wikipedia.org#cite_note-5
[41]: /w/index.php?title=Rule_of_inference&amp;action=edit§ion=3 "Edit section: Admissibility and derivability"
[42]: https://en.wikipedia.org/wiki/Natural_number "Natural number"
[43]: https://en.wikipedia.org/wiki/Natural_deduction "Natural deduction"
[44]: https://wikimedia.org/api/rest_v1/media/math/render/svg/56836978b9bf5e4a172bb62d6a808b7bdb9056de
[45]: https://wikimedia.org/api/rest_v1/media/math/render/svg/a601995d55609f2d9f5e233e36fbe9ea26011b3b
[46]: https://wikimedia.org/api/rest_v1/media/math/render/svg/9aeb98f879143f678e9639b4b3bb0f6e8417a9ba
[47]: https://wikimedia.org/api/rest_v1/media/math/render/svg/a7090b9bca2649623354dc0a01b94413c018c454
[48]: https://wikimedia.org/api/rest_v1/media/math/render/svg/603848489430ac52db7514d219a2f77a2db57548
[49]: https://en.wikipedia.org/wiki/Mathematical_induction "Mathematical induction"
[50]: https://wikimedia.org/api/rest_v1/media/math/render/svg/fce519cccc6aff38be1d3f5a08a962c01f94f502
[51]: https://wikimedia.org/api/rest_v1/media/math/render/svg/88aceb4e1d50e19e019223220992aa6ad01c8d86
[52]: https://en.wikipedia.org/wiki/Theorem "Theorem"
[53]: https://en.wikipedia.org/wiki/Sequent_calculus "Sequent calculus"
[54]: https://en.wikipedia.org/wiki/Cut_elimination "Cut elimination"
[55]: /w/index.php?title=Rule_of_inference&amp;action=edit§ion=4 "Edit section: See also"
[56]: /w/index.php?title=Rule_of_inference&amp;action=edit§ion=5 "Edit section: References"
[57]: https://en.wikipedia.org#cite_ref-1
[58]: https://en.wikipedia.org/wiki/International_Standard_Book_Number "International Standard Book Number"
[59]: https://en.wikipedia.org/wiki/Special%3ABookSources/0-521-87752-0 "Special:BookSources/0-521-87752-0"
[60]: https://en.wikipedia.org#cite_ref-Reynolds2009_2-0
[61]: https://books.google.com/books?id=2OwlTC4SOccC&amp;pg=PA12
[62]: https://en.wikipedia.org/wiki/Special%3ABookSources/978-0-521-10697-9 "Special:BookSources/978-0-521-10697-9"
[63]: https://en.wikipedia.org#cite_ref-ChiaraDoets1996_3-0
[64]: https://books.google.com/books?id=TCthvF8xLIAC&amp;pg=PA290
[65]: https://en.wikipedia.org/wiki/Special%3ABookSources/978-0-7923-4383-7 "Special:BookSources/978-0-7923-4383-7"
[66]: http://www.mi.sanu.ac.rs/~kosta/LOGCONS.pdf
[67]: https://en.wikipedia.org#cite_ref-4
[68]: https://en.wikipedia.org/wiki/Special%3ABookSources/978-0-521-88128-9 "Special:BookSources/978-0-521-88128-9"
[69]: https://en.wikipedia.org#cite_ref-5

  
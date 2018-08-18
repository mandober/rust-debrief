# Sequent calculus

[Source](https://en.wikipedia.org/wiki/Sequent_calculus "Permalink to Sequent calculus - Wikipedia")

**Sequent calculus** is, in essence, a style of formal logical [argumentation][1] where every line of a proof is a conditional [tautology][2] (called a [sequent][3] by [Gerhard Gentzen][4]) instead of an unconditional tautology. Each conditional tautology is inferred from other conditional tautologies on earlier lines in a formal argument according to rules and procedures of [inference][5], giving a better approximation to the style of natural deduction used by mathematicians than [David Hilbert's][6] earlier style of formal logic where every line was an unconditional tautology. There may be more subtle distinctions to be made; for example, there may be non-logical axioms upon which all propositions are implicitly dependent. Then sequents signify conditional [theorems][7] in a [first-order language][8] rather than conditional tautologies. 

Sequent calculus is one of several extant styles of [proof calculus][9] for expressing line-by-line logical arguments. 

* [Hilbert style][10]. Every line is an unconditional tautology (or theorem).
* Gentzen style. Every line is a conditional tautology (or theorem) with zero or more conditions on the left. 
    * [Natural deduction][11]. Every (conditional) line has exactly one asserted proposition on the right.
    * Sequent calculus. Every (conditional) line has zero or more asserted propositions on the right.

In other words, natural deduction and sequent calculus systems are particular distinct kinds of Gentzen-style systems. Hilbert-style systems typically have a very small number of inference rules, relying more on sets of axioms. Gentzen-style systems typically have very few axioms, if any, relying more on sets of rules. 

Gentzen-style systems have significant practical and theoretical advantages compared to Hilbert-style systems. For example, both natural deduction and sequent calculus systems facilitate the elimination and introduction of universal and existential [quantifiers][12] so that unquantified logical expressions can be manipulated according to the much simpler rules of [propositional calculus][13]. In a typical argument, quantifiers are eliminated, then propositional calculus is applied to unquantified expressions (which typically contain free variables), and then the quantifiers are reintroduced. This very much parallels the way in which mathematical proofs are carried out in practice by mathematicians. Predicate calculus proofs are generally much easier to discover with this approach, and are often shorter. Natural deduction systems are more suited to practical theorem-proving. Sequent calculus systems are more suited to theoretical analysis. 

## Overview[[edit][14]]

In [proof theory][15] and [mathematical logic][16], sequent calculus is a family of [formal systems][17] sharing a certain style of inference and certain formal properties. The first sequent calculi systems, **LK** and **LJ**, were introduced in 1934/1935 by Gerhard Gentzen[[1]][18] as a tool for studying [natural deduction][11] in [first-order logic][8] (in [classical][19] and [intuitionistic][20] versions, respectively). Gentzen's so-called "Main Theorem" (_Hauptsatz_) about LK and LJ was the [cut-elimination theorem][21],[[2]][22][[3]][23] a result with far-reaching [meta-theoretic][24] consequences, including [consistency][25]. Gentzen further demonstrated the power and flexibility of this technique a few years later, applying a cut-elimination argument to give a (transfinite) [proof of the consistency of Peano arithmetic][26], in surprising response to [Gödel's incompleteness theorems][27]. Since this early work, sequent calculi, also called **Gentzen systems**,[[4]][28][[5]][29][[6]][30][[7]][31] and the general concepts relating to them, have been widely applied in the fields of proof theory, mathematical logic, and [automated deduction][32]. 

### Hilbert-style deduction systems[[edit][33]]

One way to classify different styles of deduction systems is to look at the form of [_judgments][34]_ in the system, _i.e._, which things may appear as the conclusion of a (sub)proof. The simplest judgment form is used in [Hilbert-style deduction systems][35], where a judgment has the form 

: B{displaystyle B}![B][36]

where B{displaystyle B}![B][36] is any [formula][37] of first-order-logic (or whatever logic the deduction system applies to, _e.g._, propositional calculus or a [higher-order logic][38] or a [modal logic][39]). The theorems are those formulae that appear as the concluding judgment in a valid proof. A Hilbert-style system needs no distinction between formulae and judgments; we make one here solely for comparison with the cases that follow. 

The price paid for the simple syntax of a Hilbert-style system is that complete formal proofs tend to get extremely long. Concrete arguments about proofs in such a system almost always appeal to the [deduction theorem][40]. This leads to the idea of including the deduction theorem as a formal rule in the system, which happens in [natural deduction][11]. 

### Natural deduction systems[[edit][41]]

In natural deduction, judgments have the shape 

: A1,A2,…,An⊢B{displaystyle A_{1},A_{2},ldots ,A_{n}vdash B}![A_1, A_2, ldots, A_n vdash B][42]

where the Ai{displaystyle A_{i}}![A_{i}][43]'s and B{displaystyle B}![B][36] are again formulae and n≥0{displaystyle ngeq 0}![ngeq 0][44]. Permutations of the Ai{displaystyle A_{i}}![A_{i}][43]'s are immaterial. In other words, a judgment consists of a list (possibly empty) of formulae on the left-hand side of a [turnstile][45] symbol "⊢{displaystyle vdash }![vdash ][46]", with a single formula on the right-hand side.[[8]][47][[9]][48][[10]][49] The theorems are those formulae B{displaystyle B}![B][36] such that ⊢B{displaystyle vdash B}![vdash B][50] (with an empty left-hand side) is the conclusion of a valid proof. (In some presentations of natural deduction, the Ai{displaystyle A_{i}}![A_{i}][43]s and the turnstile are not written down explicitly; instead a two-dimensional notation from which they can be inferred is used.) 

The standard semantics of a judgment in natural deduction is that it asserts that whenever[[11]][51]A1{displaystyle A_{1}}![A_{1}][52], A2{displaystyle A_{2}}![A_{2}][53], etc., are all true, B{displaystyle B}![B][36] will also be true. The judgments 

: A1,…,An⊢B{displaystyle A_{1},ldots ,A_{n}vdash B}![A_1, ldots, A_n vdash B][54]

and 

: ⊢(A1∧⋯∧An)→B{displaystyle vdash (A_{1}land cdots land A_{n})rightarrow B}![vdash \(A_1 land cdots land A_n\) rightarrow B][55]

are equivalent in the strong sense that a proof of either one may be extended to a proof of the other. 

### Sequent calculus systems[[edit][56]]

Finally, sequent calculus generalizes the form of a natural deduction judgment to 

: A1,…,An⊢B1,…,Bk,{displaystyle A_{1},ldots ,A_{n}vdash B_{1},ldots ,B_{k},}![A_{1},ldots ,A_{n}vdash B_{1},ldots ,B_{k},][57]

a syntactic object called a sequent. The formulas on left-hand side of the [turnstile][45] are called the _antecedent_, and the formulas on right-hand side are called the _succedent_ or _consequent_; together they are called _cedents_ or _sequents_.[[12]][58] Again, Ai{displaystyle A_{i}}![A_{i}][43] and Bi{displaystyle B_{i}}![B_{i}][59] are formulae, and n{displaystyle n}![n][60] and k{displaystyle k}![k][61] are nonnegative integers, that is, the left-hand-side or the right-hand-side (or neither or both) may be empty. As in natural deduction, theorems are those B{displaystyle B}![B][36] where ⊢B{displaystyle vdash B}![vdash B][50] is the conclusion of a valid proof. 

The standard semantics of a sequent is an assertion that whenever _every_ Ai{displaystyle A_{i}}![ A_i][43] is true, _at least one_ Bi{displaystyle B_{i}}![B_{i}][59] will also be true.[[13]][62] Thus the empty sequent, having both cedents empty, is false.[[14]][63] One way to express this is that a comma to the left of the turnstile should be thought of as an "and", and a comma to the right of the turnstile should be thought of as an (inclusive) "or". The sequents 

: A1,…,An⊢B1,…,Bk{displaystyle A_{1},ldots ,A_{n}vdash B_{1},ldots ,B_{k}}![A_1, ldots, A_n vdash B_1, ldots, B_k][64]

and 

: ⊢(A1∧⋯∧An)→(B1∨⋯∨Bk){displaystyle vdash (A_{1}land cdots land A_{n})rightarrow (B_{1}lor cdots lor B_{k})}![vdash \(A_1 landcdotsland A_n\)rightarrow\(B_1 lorcdotslor B_k\)][65]

are equivalent in the strong sense that a proof of either one may be extended to a proof of the other. 

At first sight, this extension of the judgment form may appear to be a strange complication — it is not motivated by an obvious shortcoming of natural deduction, and it is initially confusing that the comma seems to mean entirely different things on the two sides of the turnstile. However, in a [classical context][19] the semantics of the sequent can also (by propositional tautology) be expressed either as 

: ⊢¬A1∨¬A2∨⋯∨¬An∨B1∨B2∨⋯∨Bk{displaystyle vdash neg A_{1}lor neg A_{2}lor cdots lor neg A_{n}lor B_{1}lor B_{2}lor cdots lor B_{k}}![vdash neg A_1 lor neg A_2 lor cdots lor neg A_n lor B_1 lor B_2 lorcdotslor B_k][66]

(at least one of the As is false, or one of the Bs is true) or as 

: ⊢¬(A1∧A2∧⋯∧An∧¬B1∧¬B2∧⋯∧¬Bk){displaystyle vdash neg (A_{1}land A_{2}land cdots land A_{n}land neg B_{1}land neg B_{2}land cdots land neg B_{k})}![vdash neg\(A_1 land A_2 land cdots land A_n land neg B_1 land neg B_2 landcdotsland neg B_k\)][67]

(it cannot be the case that all of the As are true and all of the Bs are false). In these formulations, the only difference between formulae on either side of the turnstile is that one side is negated. Thus, swapping left for right in a sequent corresponds to negating all of the constituent formulae. This means that a symmetry such as [De Morgan's laws][68], which manifests itself as logical negation on the semantic level, translates directly into a left-right symmetry of sequents — and indeed, the inference rules in sequent calculus for dealing with conjunction (∧) are mirror images of those dealing with disjunction (∨). 

Many logicians feel that this symmetric presentation offers a deeper insight in the structure of the logic than other styles of proof system, where the classical duality of negation is not as apparent in the rules. 

### Distinction between natural deduction and sequent calculus[[edit][69]]

Gentzen asserted a sharp distinction between his single-output natural deduction systems (NK and NJ) and his multiple-output sequent calculus systems (LK and LJ). He wrote that the intuitionistic natural deduction system NJ was somewhat ugly.[[15]][70] He said that the special role of the [excluded middle][71] in the classical natural deduction system NK is removed in the classical sequent calculus system LK.[[16]][72] He said that the sequent calculus LJ gave more symmetry than natural deduction NJ in the case of intuitionistic logic, as also in the case of classical logic (LK versus NK).[[17]][73] Then he said that in addition to these reasons, the sequent calculus with multiple succedent formulas is intended particularly for his principal theorem ("Hauptsatz").[[18]][74]

### Origin of word "sequent"[[edit][75]]

The word "sequent" is taken from the word "Sequenz" in Gentzen's 1934 paper.[[1]][18] Kleene makes the following comment on the translation into English: "Gentzen says 'Sequenz', which we translate as 'sequent', because we have already used 'sequence' for any succession of objects, where the German is 'Folge'."[[19]][76]

## Proving logical formulas[[edit][77]]

![][78]

A rooted tree describing a proof finding procedure by sequent calculus

### Reduction trees[[edit][79]]

Sequent calculus can be seen as a tool for proving formulas in [propositional logic][80], similar to the [method of analytic tableaux][81]. It gives a series of steps which allows one to reduce the problem of proving a logical formula to simpler and simpler formulas until one arrives at trivial ones.[[20]][82]

Consider the following formula: 

: ((p→r)∨(q→r))→((p∧q)→r){displaystyle ((prightarrow r)lor (qrightarrow r))rightarrow ((pland q)rightarrow r)}![{displaystyle \(\(prightarrow r\)lor \(qrightarrow r\)\)rightarrow \(\(pland q\)rightarrow r\)}][83]

This is written in the following form, where the proposition that needs to be proven is to the right of the [turnstile symbol][45] ⊢{displaystyle vdash }![vdash ][46]: 

: ⊢((p→r)∨(q→r))→((p∧q)→r){displaystyle vdash ((prightarrow r)lor (qrightarrow r))rightarrow ((pland q)rightarrow r)}![{displaystyle vdash \(\(prightarrow r\)lor \(qrightarrow r\)\)rightarrow \(\(pland q\)rightarrow r\)}][84]

Now, instead of proving this from the axioms, it is enough to assume the premise of the [implication][85] and then try to prove its conclusion.[[21]][86] Hence one moves to the following sequent: 

: (p→r)∨(q→r)⊢(p∧q)→r{displaystyle (prightarrow r)lor (qrightarrow r)vdash (pland q)rightarrow r}![{displaystyle \(prightarrow r\)lor \(qrightarrow r\)vdash \(pland q\)rightarrow r}][87]

Again the right hand side includes an implication, whose premise can further be assumed so that only its conclusion needs to be proven: 

: (p→r)∨(q→r),(p∧q)⊢r{displaystyle (prightarrow r)lor (qrightarrow r),(pland q)vdash r}![{displaystyle \(prightarrow r\)lor \(qrightarrow r\),\(pland q\)vdash r}][88]

Since the arguments in the left-hand side are assumed to be related by [conjunction][89], this can be replaced by the following: 

: (p→r)∨(q→r),p,q⊢r{displaystyle (prightarrow r)lor (qrightarrow r),p,qvdash r}![{displaystyle \(prightarrow r\)lor \(qrightarrow r\),p,qvdash r}][90]

This is equivalent to proving the conclusion in both cases of the [disjunction][91] on the first argument on the left. Thus we may split the sequent to two, where we now have to prove each separately: 

: p→r,p,q⊢r{displaystyle prightarrow r,p,qvdash r}![{displaystyle prightarrow r,p,qvdash r}][92]

: q→r,p,q⊢r{displaystyle qrightarrow r,p,qvdash r}![{displaystyle qrightarrow r,p,qvdash r}][93]

This process is continued until there are only atomic formulas in each side. The process can be graphically described by a [rooted tree graph][94], as depicted on the right. The root of the tree is the formula we wish to prove; the leaves consist of atomic formulas only. The tree is known as a **reduction tree**.[[20]][82][[22]][95]

The items to the left of the turnstile are understood to be connected by conjunction, and those to the right by disjunction. Therefore, when both consist only of atomic symbols, the sequent is provable (and always true) if and only if at least one of the symbols on the right also appears on the left. 

Following are the rules by which one proceeds along the tree. Whenever one sequent is split into two, the tree vertex has three edges (one coming from the vertex closer to the root), and the tree is branched. Additionally, one may freely change the order of the arguments in each side; Γ and Δ stand for possible additional arguments.[[20]][82]

The usual term for the horizontal line used in Gentzen-style layouts for natural deduction is **inference line**.[[23]][96]

| ----- |
| Left:  |  Right:  | 
| 

L∧{displaystyle Lland }![{displaystyle Lland }][97] rule: Γ,A∧B⊢ΔΓ,A,B⊢Δ{displaystyle quad {cfrac {Gamma ,Aland Bvdash Delta }{Gamma ,A,Bvdash Delta }}}![{displaystyle quad {cfrac {Gamma ,Aland Bvdash Delta }{Gamma ,A,Bvdash Delta }}}][98]

 | 

R∧{displaystyle Rland }![{displaystyle Rland }][99] rule: Γ⊢Δ,A∧BΓ⊢Δ,AΓ⊢Δ,B{displaystyle {cfrac {Gamma vdash Delta ,Aland B}{Gamma vdash Delta ,Aqquad Gamma vdash Delta ,B}}}![{displaystyle {cfrac {Gamma vdash Delta ,Aland B}{Gamma vdash Delta ,Aqquad Gamma vdash Delta ,B}}}][100]

 | 
| 

L∨{displaystyle Llor }![{displaystyle Llor }][101] rule: Γ,A∨B⊢ΔΓ,A⊢ΔΓ,B⊢Δ{displaystyle {cfrac {Gamma ,Alor Bvdash Delta }{Gamma ,Avdash Delta qquad Gamma ,Bvdash Delta }}}![{displaystyle {cfrac {Gamma ,Alor Bvdash Delta }{Gamma ,Avdash Delta qquad Gamma ,Bvdash Delta }}}][102]

 | 

R∨{displaystyle Rlor }![{displaystyle Rlor }][103] rule: Γ⊢Δ,A∨BΓ⊢Δ,A,B{displaystyle quad {cfrac {Gamma vdash Delta ,Alor B}{Gamma vdash Delta ,A,B}}}![{displaystyle quad {cfrac {Gamma vdash Delta ,Alor B}{Gamma vdash Delta ,A,B}}}][104]

 | 
| 

L→{displaystyle Lrightarrow }![{displaystyle Lrightarrow }][105] rule: Γ,A→B⊢ΔΓ⊢Δ,AΓ,B⊢Δ{displaystyle {cfrac {Gamma ,Arightarrow Bvdash Delta }{Gamma vdash Delta ,Aqquad Gamma ,Bvdash Delta }}}![{displaystyle {cfrac {Gamma ,Arightarrow Bvdash Delta }{Gamma vdash Delta ,Aqquad Gamma ,Bvdash Delta }}}][106]

 | 

R→{displaystyle Rrightarrow }![{displaystyle Rrightarrow }][107] rule: Γ⊢Δ,A→BΓ,A⊢Δ,B{displaystyle quad {cfrac {Gamma vdash Delta ,Arightarrow B}{Gamma ,Avdash Delta ,B}}}![{displaystyle quad {cfrac {Gamma vdash Delta ,Arightarrow B}{Gamma ,Avdash Delta ,B}}}][108]

 | 
| 

L¬{displaystyle Llnot }![{displaystyle Llnot }][109] rule: Γ,¬A⊢ΔΓ⊢Δ,A{displaystyle quad {cfrac {Gamma ,lnot Avdash Delta }{Gamma vdash Delta ,A}}}![{displaystyle quad {cfrac {Gamma ,lnot Avdash Delta }{Gamma vdash Delta ,A}}}][110]

 | 

R¬{displaystyle Rlnot }![{displaystyle Rlnot }][111] rule: Γ⊢Δ,¬AΓ,A⊢Δ{displaystyle quad {cfrac {Gamma vdash Delta ,lnot A}{Gamma ,Avdash Delta }}}![{displaystyle quad {cfrac {Gamma vdash Delta ,lnot A}{Gamma ,Avdash Delta }}}][112]

 | 
| Axiom:  |  |
| 

p,r⊢q,r{displaystyle p,rvdash q,r}![{displaystyle p,rvdash q,r}][113]

 | 

Starting with any formula in propositional logic, by a series of steps, the right side of the turnstile can be processed until it includes only atomic symbols. Then, the same is done for the left side. Since every logical operator appears in one of the rules above, and is omitted by the rule, the process terminates when no logical operators remain: The formula has been _decomposed_. 

Thus, the sequents in the leaves of the trees include only atomic symbols, which are either provable by the axiom or not, according to whether one of the symbols on the right also appears on the left. 

It is easy to see that the steps in the tree preserve the semantic truth value of the formulas implied by them, with conjunction understood between the tree's different branches whenever there is a split. It is also obvious that an axiom is provable if and only if it is true for every truth values of the atomic symbols. Thus this system is [sound][114] and [complete][115] in propositional logic. 

### Relation to standard axiomatizations[[edit][116]]

Sequent calculus is related to other axiomatizations of propositional calculus, such as [Frege's propositional calculus][117] or [Jan Łukasiewicz's axiomatization][118] (itself a part of the standard [Hilbert system][10]): Every formula that can be proven in these has a reduction tree. 

This can be shown as follows: Every proof in propositional calculus uses only axioms and the inference rules. Each use of an axiom scheme yields a true logical formula, and can thus be proven in sequent calculus; examples for these are [shown below][119]. The only inference rule in the systems mentioned above is modus ponens, which is implemented by the cut rule. 

## The system LK[[edit][120]]

This section introduces the rules of the sequent calculus **LK** (which just stands for "**k**lassische Prädikaten**l**ogik"), as introduced by Gentzen in 1934. A (formal) proof in this calculus is a sequence of sequents, where each of the sequents is derivable from sequents appearing earlier in the sequence by using one of the [rules][121] below. 

### Inference rules[[edit][122]]

The following notation will be used: 

* ⊢{displaystyle vdash }![vdash ][46] known as the [turnstile][45], separates the _assumptions_ on the left from the _propositions_ on the right
* A{displaystyle A}![A][123] and B{displaystyle B}![B][36] denote formulae of first-order predicate logic (one may also restrict this to propositional logic),
* Γ,Δ,Σ{displaystyle Gamma ,Delta ,Sigma }![Gamma, Delta, Sigma][124], and Π{displaystyle Pi }![Pi ][125] are finite (possibly empty) sequences of formulae (in fact, the order of formulae does not matter; see subsection [Structural Rules][126]), called contexts, 
    * when on the _left_ of the ⊢{displaystyle vdash }![vdash ][46], the sequence of formulas is considered _conjunctively_ (all assumed to hold at the same time),
    * while on the _right_ of the ⊢{displaystyle vdash }![vdash ][46], the sequence of formulas is considered _disjunctively_ (at least one of the formulas must hold for any assignment of variables),
* t{displaystyle t}![t][127] denotes an arbitrary term,
* x{displaystyle x}![x][128] and y{displaystyle y}![y][129] denote variables.
* a variable is said to occur [free][130] within a formula if it occurs outside the scope of quantifiers ∀{displaystyle forall }![forall ][131] or ∃{displaystyle exists }![exists ][132].
* A[t/x]{displaystyle A[t/x]}![A\[t/x\]][133] denotes the formula that is obtained by substituting the term t{displaystyle t}![t][127] for every free occurrence of the variable x{displaystyle x}![x][128] in formula A{displaystyle A}![A][123] with the restriction that the term t{displaystyle t}![t][127] must be free for the variable x{displaystyle x}![x][128] in A{displaystyle A}![A][123] (i.e., no occurrence of any variable in t{displaystyle t}![t][127] becomes bound in A[t/x]{displaystyle A[t/x]}![A\[t/x\]][133]).
* WL{displaystyle WL}![WL][134] and WR{displaystyle WR}![WR][135] stand for _Weakening Left/Right_, CL{displaystyle CL}![CL][136] and CR{displaystyle CR}![CR][137] for _Contraction_, and PL{displaystyle PL}![PL][138] and PR{displaystyle PR}![PR][139] for _Permutation_.

Note that, contrary to the rules for proceeding along the reduction tree presented above, the following rules are for moving in the opposite directions, from axioms to theorems. Thus they are exact mirror-images of the rules above, except that here symmetry is not implicitly assumed, and rules regarding [quantification][140] are added. 

| ----- |
| Axiom:  |  Cut:  | 
| 

A⊢A(I){displaystyle {cfrac {qquad }{Avdash A}}quad (I)}![ cfrac{qquad }{ A vdash A} quad \(I\) ][141]

 | 

Γ⊢Δ,AA,Σ⊢ΠΓ,Σ⊢Δ,Π(Cut){displaystyle {cfrac {Gamma vdash Delta ,Aqquad A,Sigma vdash Pi }{Gamma ,Sigma vdash Delta ,Pi }}quad ({mathit {Cut}})}![ 
   cfrac{Gamma vdash Delta, A qquad A, Sigma vdash Pi} {Gamma, Sigma vdash Delta, Pi} quad \(mathit{Cut}\)
 ][142]

 | 
| Left logical rules:  |  Right logical rules:  | 
| 

Γ,A⊢ΔΓ,A∧B⊢Δ(∧L1){displaystyle {cfrac {Gamma ,Avdash Delta }{Gamma ,Aland Bvdash Delta }}quad ({land }L_{1})}![ cfrac{Gamma, A vdash Delta} {Gamma, A and B vdash Delta} quad \({and}L_1\)
 ][143]

 | 

Γ⊢A,ΔΓ⊢A∨B,Δ(∨R1){displaystyle {cfrac {Gamma vdash A,Delta }{Gamma vdash Alor B,Delta }}quad ({lor }R_{1})}![ cfrac{Gamma vdash A, Delta}{Gamma vdash A or B, Delta} quad  \({or}R_1\)
 ][144]

 | 
| 

Γ,B⊢ΔΓ,A∧B⊢Δ(∧L2){displaystyle {cfrac {Gamma ,Bvdash Delta }{Gamma ,Aland Bvdash Delta }}quad ({land }L_{2})}![ cfrac{Gamma, B vdash Delta}{Gamma, A and B vdash Delta}  quad \({and}L_2\)
 ][145]

 | 

Γ⊢B,ΔΓ⊢A∨B,Δ(∨R2){displaystyle {cfrac {Gamma vdash B,Delta }{Gamma vdash Alor B,Delta }}quad ({lor }R_{2})}![ cfrac{Gamma vdash B, Delta}{Gamma vdash A or B, Delta} quad \({or}R_2\)
 ][146]

 | 
| 

Γ,A⊢ΔΣ,B⊢ΠΓ,Σ,A∨B⊢Δ,Π(∨L){displaystyle {cfrac {Gamma ,Avdash Delta qquad Sigma ,Bvdash Pi }{Gamma ,Sigma ,Alor Bvdash Delta ,Pi }}quad ({lor }L)}![ cfrac{Gamma, A vdash Delta qquad Sigma, B vdash Pi}{Gamma, Sigma, A or B vdash Delta, Pi} quad \({or}L\)
 ][147]

 | 

Γ⊢A,ΔΣ⊢B,ΠΓ,Σ⊢A∧B,Δ,Π(∧R){displaystyle {cfrac {Gamma vdash A,Delta qquad Sigma vdash B,Pi }{Gamma ,Sigma vdash Aland B,Delta ,Pi }}quad ({land }R)}![ cfrac{Gamma vdash A, Delta qquad Sigma vdash B, Pi}{Gamma, Sigma vdash A and B, Delta, Pi} quad \({and}R\)
 ][148]

 | 
| 

Γ⊢A,ΔΣ,B⊢ΠΓ,Σ,A→B⊢Δ,Π(→L){displaystyle {cfrac {Gamma vdash A,Delta qquad Sigma ,Bvdash Pi }{Gamma ,Sigma ,Arightarrow Bvdash Delta ,Pi }}quad ({rightarrow }L)}![
  cfrac{Gamma vdash A, Delta qquad Sigma, B vdash Pi}{Gamma, Sigma, Arightarrow B vdash Delta, Pi} quad  \({rightarrow }L\)
 ][149]

 | 

Γ,A⊢B,ΔΓ⊢A→B,Δ(→R){displaystyle {cfrac {Gamma ,Avdash B,Delta }{Gamma vdash Arightarrow B,Delta }}quad ({rightarrow }R)}![
   cfrac{Gamma, A vdash B, Delta}{Gamma vdash A rightarrow B, Delta} quad \({rightarrow}R\)
 ][150]

 | 
| 

Γ⊢A,ΔΓ,¬A⊢Δ(¬L){displaystyle {cfrac {Gamma vdash A,Delta }{Gamma ,lnot Avdash Delta }}quad ({lnot }L)}![
  cfrac{Gamma vdash A, Delta}{Gamma, lnot A vdash Delta} quad  \({lnot}L\)
 ][151]

 | 

Γ,A⊢ΔΓ⊢¬A,Δ(¬R){displaystyle {cfrac {Gamma ,Avdash Delta }{Gamma vdash lnot A,Delta }}quad ({lnot }R)}![
  cfrac{Gamma, A vdash Delta}{Gamma vdash lnot A, Delta} quad \({lnot}R\)
 ][152]

 | 
| 

Γ,A[t/x]⊢ΔΓ,∀xA⊢Δ(∀L){displaystyle {cfrac {Gamma ,A[t/x]vdash Delta }{Gamma ,forall xAvdash Delta }}quad ({forall }L)}![
  cfrac{Gamma, A\[t/x\] vdash Delta}{Gamma, forall x A vdash Delta} quad  \({forall}L\)
 ][153]

 | 

Γ⊢A[y/x],ΔΓ⊢∀xA,Δ(∀R){displaystyle {cfrac {Gamma vdash A[y/x],Delta }{Gamma vdash forall xA,Delta }}quad ({forall }R)}![
  cfrac{Gamma vdash A\[y/x\], Delta}{Gamma vdash forall x A, Delta} quad  \({forall}R\) 
 ][154]

 | 
| 

Γ,A[y/x]⊢ΔΓ,∃xA⊢Δ(∃L){displaystyle {cfrac {Gamma ,A[y/x]vdash Delta }{Gamma ,exists xAvdash Delta }}quad ({exists }L)}![
  cfrac{Gamma, A\[y/x\] vdash Delta}{Gamma, exists x A vdash Delta} quad  \({exists}L\)
 ][155]

 | 

Γ⊢A[t/x],ΔΓ⊢∃xA,Δ(∃R){displaystyle {cfrac {Gamma vdash A[t/x],Delta }{Gamma vdash exists xA,Delta }}quad ({exists }R)}![
  cfrac{Gamma vdash A\[t/x\], Delta}{Gamma vdash exists x A, Delta} quad  \({exists}R\)
 ][156]

 | 
| Left structural rules:  |  Right structural rules:  | 
| 

Γ⊢ΔΓ,A⊢Δ(WL){displaystyle {cfrac {Gamma vdash Delta }{Gamma ,Avdash Delta }}quad ({mathit {WL}})}![
  cfrac{Gamma vdash Delta}{Gamma, A vdash Delta} quad \(mathit{WL}\)
 ][157]

 | 

Γ⊢ΔΓ⊢A,Δ(WR){displaystyle {cfrac {Gamma vdash Delta }{Gamma vdash A,Delta }}quad ({mathit {WR}})}![
  cfrac{Gamma vdash Delta}{Gamma vdash A, Delta} quad \(mathit{WR}\)
 ][158]

 | 
| 

Γ,A,A⊢ΔΓ,A⊢Δ(CL){displaystyle {cfrac {Gamma ,A,Avdash Delta }{Gamma ,Avdash Delta }}quad ({mathit {CL}})}![
  cfrac{Gamma, A, A vdash Delta}{Gamma, A vdash Delta} quad \(mathit{CL}\)
 ][159]

 | 

Γ⊢A,A,ΔΓ⊢A,Δ(CR){displaystyle {cfrac {Gamma vdash A,A,Delta }{Gamma vdash A,Delta }}quad ({mathit {CR}})}![
  cfrac{Gamma vdash A, A, Delta}{Gamma vdash A, Delta} quad \(mathit{CR}\)
 ][160]

 | 
| 

Γ1,A,B,Γ2⊢ΔΓ1,B,A,Γ2⊢Δ(PL){displaystyle {cfrac {Gamma _{1},A,B,Gamma _{2}vdash Delta }{Gamma _{1},B,A,Gamma _{2}vdash Delta }}quad ({mathit {PL}})}![
  cfrac{Gamma_1, A, B, Gamma_2 vdash Delta}{Gamma_1, B, A, Gamma_2 vdash Delta} quad \(mathit{PL}\)
 ][161]

 | 

Γ⊢Δ1,A,B,Δ2Γ⊢Δ1,B,A,Δ2(PR){displaystyle {cfrac {Gamma vdash Delta _{1},A,B,Delta _{2}}{Gamma vdash Delta _{1},B,A,Delta _{2}}}quad ({mathit {PR}})}![
  cfrac{Gamma vdash Delta_1, A, B, Delta_2}{Gamma vdash Delta_1, B, A, Delta_2} quad \(mathit{PR}\)
 ][162]

 | 

_Restrictions: In the rules (∀R){displaystyle ({forall }R)}![\({forall}R\)][163] and (∃L){displaystyle ({exists }L)}![\({exists}L\)][164], the variable y{displaystyle y}![y][129] must not occur free anywhere in the respective lower sequents._

### An intuitive explanation[[edit][165]]

The above rules can be divided into two major groups: _logical_ and _structural_ ones. Each of the logical rules introduces a new logical formula either on the left or on the right of the [turnstile][45] ⊢{displaystyle vdash }![vdash ][46]. In contrast, the structural rules operate on the structure of the sequents, ignoring the exact shape of the formulae. The two exceptions to this general scheme are the axiom of identity (I) and the rule of (Cut). 

Although stated in a formal way, the above rules allow for a very intuitive reading in terms of classical logic. Consider, for example, the rule (∧L1){displaystyle ({land }L_{1})}![\({and}L_1\)][166]. It says that, whenever one can prove that Δ{displaystyle Delta }![Delta ][167] can be concluded from some sequence of formulae that contain A{displaystyle A}![A][123], then one can also conclude Δ{displaystyle Delta }![Delta ][167] from the (stronger) assumption that A∧B{displaystyle Aland B}![Aland B][168] holds. Likewise, the rule (¬R){displaystyle ({neg }R)}![\({neg}R\)][169] states that, if Γ{displaystyle Gamma }![Gamma ][170] and A{displaystyle A}![A][123] suffice to conclude Δ{displaystyle Delta }![Delta ][167], then from Γ{displaystyle Gamma }![Gamma ][170] alone one can either still conclude Δ{displaystyle Delta }![Delta ][167] or A{displaystyle A}![A][123] must be false, i.e. ¬A{displaystyle {neg }A}![{neg}A][171] holds. All the rules can be interpreted in this way. 

For an intuition about the quantifier rules, consider the rule (∀R){displaystyle ({forall }R)}![\({forall}R\)][163]. Of course concluding that ∀xA{displaystyle forall {x}A}![forall{x} A][172] holds just from the fact that A[y/x]{displaystyle A[y/x]}![A\[y/x\]][173] is true is not in general possible. If, however, the variable y is not mentioned elsewhere (i.e. it can still be chosen freely, without influencing the other formulae), then one may assume, that A[y/x]{displaystyle A[y/x]}![A\[y/x\]][173] holds for any value of y. The other rules should then be pretty straightforward. 

Instead of viewing the rules as descriptions for legal derivations in predicate logic, one may also consider them as instructions for the construction of a proof for a given statement. In this case the rules can be read bottom-up; for example, (∧R){displaystyle ({land }R)}![\({and}R\)][174] says that, to prove that A∧B{displaystyle Aland B}![Aland B][168] follows from the assumptions Γ{displaystyle Gamma }![Gamma ][170] and Σ{displaystyle Sigma }![Sigma ][175], it suffices to prove that A{displaystyle A}![A][123] can be concluded from Γ{displaystyle Gamma }![Gamma ][170] and B{displaystyle B}![B][36] can be concluded from Σ{displaystyle Sigma }![Sigma ][175], respectively. Note that, given some antecedent, it is not clear how this is to be split into Γ{displaystyle Gamma }![Gamma ][170] and Σ{displaystyle Sigma }![Sigma ][175]. However, there are only finitely many possibilities to be checked since the antecedent by assumption is finite. This also illustrates how proof theory can be viewed as operating on proofs in a combinatorial fashion: given proofs for both A{displaystyle A}![A][123] and B{displaystyle B}![B][36], one can construct a proof for A∧B{displaystyle Aland B}![Aland B][168]. 

When looking for some proof, most of the rules offer more or less direct recipes of how to do this. The rule of cut is different: it states that, when a formula A{displaystyle A}![A][123] can be concluded and this formula may also serve as a premise for concluding other statements, then the formula A{displaystyle A}![A][123] can be "cut out" and the respective derivations are joined. When constructing a proof bottom-up, this creates the problem of guessing A{displaystyle A}![A][123] (since it does not appear at all below). The [cut-elimination theorem][21] is thus crucial to the applications of sequent calculus in [automated deduction][32]: it states that all uses of the cut rule can be eliminated from a proof, implying that any provable sequent can be given a _cut-free_ proof. 

The second rule that is somewhat special is the axiom of identity (I). The intuitive reading of this is obvious: every formula proves itself. Like the cut rule, the axiom of identity is somewhat redundant: the [completeness of atomic initial sequents][176] states that the rule can be restricted to [atomic formulas][177] without any loss of provability. 

Observe that all rules have mirror companions, except the ones for implication. This reflects the fact that the usual language of first-order logic does not include the "is not implied by" connective ↚{displaystyle not leftarrow }![not leftarrow ][178] that would be the De Morgan dual of implication. Adding such a connective with its natural rules would make the calculus completely left-right symmetric. 

### Example derivations[[edit][179]]

Here is the derivation of "⊢A∨¬A{displaystyle vdash Alor lnot A}![ vdash A or lnot A ][180]", known as the [_Law of excluded middle][71]_ (_tertium non datur_ in Latin). 

Next is the proof of a simple fact involving quantifiers. Note that the converse is not true, and its falsity can be seen when attempting to derive it bottom-up, because an existing free variable cannot be used in substitution in the rules (∀R){displaystyle (forall R)}![\(forall R\)][181] and (∃L){displaystyle (exists L)}![\(exists L\)][182]. 

| ----- |
|    |     |  (I){displaystyle (I)}![
      \(I\)
    ][183]  | 
| p(x,y)⊢p(x,y){displaystyle p(x,y)vdash p(x,y)}![
      p\(x,y\) vdash p\(x,y\)
    ][184]  |     |  |
|    |  (∀L){displaystyle (forall L)}![
      \(forall L\)
    ][185]  | 
| ∀x(p(x,y))⊢p(x,y){displaystyle forall xleft(p(x,y)right)vdash p(x,y)}![
      forall x left\( p\(x,y\) right\) vdash p\(x,y\)
    ][186]  |     | 
|    |  (∃R){displaystyle (exists R)}![
      \(exists R\)
    ][187]  | 
| ∀x(p(x,y))⊢∃y(p(x,y)){displaystyle forall xleft(p(x,y)right)vdash exists yleft(p(x,y)right)}![
      forall x left\( p\(x,y\) right\) vdash exists y left\( p\(x,y\) right\)
    ][188]  |     | 
|    |  (∃L){displaystyle (exists L)}![
      \(exists L\)
    ][182]  | 
| ∃y(∀x(p(x,y)))⊢∃y(p(x,y)){displaystyle exists yleft(forall xleft(p(x,y)right)right)vdash exists yleft(p(x,y)right)}![
      exists y left\( forall x left\( p\(x,y\) right\) right\) vdash exists y left\( p\(x,y\) right\)
    ][189]  |     | 
|    |  (∀R){displaystyle (forall R)}![
      \(forall R\)
    ][181]  | 
| ∃y(∀x(p(x,y)))⊢∀x(∃y(p(x,y))){displaystyle exists yleft(forall xleft(p(x,y)right)right)vdash forall xleft(exists yleft(p(x,y)right)right)}![
      exists y left\( forall x left\( p\(x,y\) right\) right\) vdash forall x left\( exists y left\( p\(x,y\) right\) right\)
    ][190]  |     | 
|    |     | 

For something more interesting we shall prove ((A→(B∨C))→(((B→¬A)∧¬C)→¬A)){displaystyle {left(left(Arightarrow left(Blor Cright)right)rightarrow left(left(left(Brightarrow lnot Aright)land lnot Cright)rightarrow lnot Aright)right)}}![{displaystyle {left\(left\(Arightarrow left\(Blor Cright\)right\)rightarrow left\(left\(left\(Brightarrow lnot Aright\)land lnot Cright\)rightarrow lnot Aright\)right\)}}][191]. It is straightforward to find the derivation, which exemplifies the usefulness of LK in automated proving. 

| ----- |
| 

|   |      | 

| 

|   |      | 

|    |     |  (I){displaystyle (I)}![

      \(I\)
    ][183]  | 
| ¬A⊢¬A{displaystyle lnot Avdash lnot A}![
      lnot A vdash lnot A
    ][192]  |     |  | | | | | | | | |
|    |     | 

 | 

 |     |  (→L){displaystyle (rightarrow L)}![
      \(rightarrow L\)
    ][193]  | 
| (B∨C),¬C,(B→¬A)⊢¬A{displaystyle left(Blor Cright),lnot C,left(Brightarrow lnot Aright)vdash lnot A}![
      left\( B or C right\) , lnot C , left\( B rightarrow lnot A right\) vdash lnot A
    ][194]  |     | 
|    |  (∧L1){displaystyle (land L_{1})}![
      \(and L_1\)
    ][195]  | 
| (B∨C),¬C,((B→¬A)∧¬C)⊢¬A{displaystyle left(Blor Cright),lnot C,left(left(Brightarrow lnot Aright)land lnot Cright)vdash lnot A}![
      left\( B or C right\) , lnot C , left\( left\( B rightarrow lnot A right\) and lnot C right\) vdash lnot A
    ][196]  |     | 
|    |  (PL){displaystyle (PL)}![
      \(PL\)
    ][197]  | 
| (B∨C),((B→¬A)∧¬C),¬C⊢¬A{displaystyle left(Blor Cright),left(left(Brightarrow lnot Aright)land lnot Cright),lnot Cvdash lnot A}![
      left\( B or C right\) , left\( left\( B rightarrow lnot A right\) and lnot C right\) , lnot C vdash lnot A
    ][198]  |     | 
|    |  (∧L2){displaystyle (land L_{2})}![
      \(and L_2\)
    ][199]  | 
| (B∨C),((B→¬A)∧¬C),((B→¬A)∧¬C)⊢¬A{displaystyle left(Blor Cright),left(left(Brightarrow lnot Aright)land lnot Cright),left(left(Brightarrow lnot Aright)land lnot Cright)vdash lnot A}![
      left\( B or C right\) , left\( left\( B rightarrow lnot A right\) and lnot C right\) , left\( left\( B rightarrow lnot A right\) and lnot C right\) vdash lnot A
    ][200]  |     | 
|    |  (CL){displaystyle (CL)}![
      \(CL\)
    ][201]  | 
| (B∨C),((B→¬A)∧¬C)⊢¬A{displaystyle left(Blor Cright),left(left(Brightarrow lnot Aright)land lnot Cright)vdash lnot A}![
      left\( B or C right\) , left\( left\( B rightarrow lnot A right\) and lnot C right\) vdash lnot A
    ][202]  |     | 
|    |  (PL){displaystyle (PL)}![
      \(PL\)
    ][197]  | 
| ((B→¬A)∧¬C),(B∨C)⊢¬A{displaystyle left(left(Brightarrow lnot Aright)land lnot Cright),left(Blor Cright)vdash lnot A}![
      left\( left\( B rightarrow lnot A right\) and lnot C right\) , left\( B or C right\) vdash lnot A
    ][203]  |     | 
|    |     | 

 | 

 |     |  (→L){displaystyle (rightarrow L)}![
      \(rightarrow L\)
    ][193]  | 
| ((B→¬A)∧¬C),(A→(B∨C))⊢¬A,¬A{displaystyle left(left(Brightarrow lnot Aright)land lnot Cright),left(Arightarrow left(Blor Cright)right)vdash lnot A,lnot A}![
      left\( left\( B rightarrow lnot A right\) and lnot C right\) , left\( A rightarrow left\( B or C right\) right\) vdash lnot A , lnot A
    ][204]  |     | 
|    |  (CR){displaystyle (CR)}![
      \(CR\)
    ][205]  | 
| ((B→¬A)∧¬C),(A→(B∨C))⊢¬A{displaystyle left(left(Brightarrow lnot Aright)land lnot Cright),left(Arightarrow left(Blor Cright)right)vdash lnot A}![
      left\( left\( B rightarrow lnot A right\) and lnot C right\) , left\( A rightarrow left\( B or C right\) right\) vdash lnot A
    ][206]  |     | 
|    |  (PL){displaystyle (PL)}![
      \(PL\)
    ][197]  | 
| (A→(B∨C)),((B→¬A)∧¬C)⊢¬A{displaystyle left(Arightarrow left(Blor Cright)right),left(left(Brightarrow lnot Aright)land lnot Cright)vdash lnot A}![
      left\( A rightarrow left\( B or C right\) right\) , left\( left\( B rightarrow lnot A right\) and lnot C right\) vdash lnot A
    ][207]  |     | 
|    |  (→R){displaystyle (rightarrow R)}![
      \(rightarrow R\)
    ][208]  | 
| (A→(B∨C))⊢(((B→¬A)∧¬C)→¬A){displaystyle left(Arightarrow left(Blor Cright)right)vdash left(left(left(Brightarrow lnot Aright)land lnot Cright)rightarrow lnot Aright)}![
      left\( A rightarrow left\( B or C right\) right\) vdash left\( left\( left\( B rightarrow lnot A right\) and lnot C right\) rightarrow lnot A right\)
    ][209]  |     | 
|    |  (→R){displaystyle (rightarrow R)}![
      \(rightarrow R\)
    ][208]  | 
| ⊢((A→(B∨C))→(((B→¬A)∧¬C)→¬A)){displaystyle vdash left(left(Arightarrow left(Blor Cright)right)rightarrow left(left(left(Brightarrow lnot Aright)land lnot Cright)rightarrow lnot Aright)right)}![
      vdash left\( left\( A rightarrow left\( B or C right\) right\) rightarrow left\( left\( left\( B rightarrow lnot A right\) and lnot C right\) rightarrow lnot A right\) right\)
    ][210]  |     | 
|    |     | 

These derivations also emphasize the strictly formal structure of the sequent calculus. For example, the logical rules as defined above always act on a formula immediately adjacent to the turnstile, such that the permutation rules are necessary. Note, however, that this is in part an artifact of the presentation, in the original style of Gentzen. A common simplification involves the use of [multisets][211] of formulas in the interpretation of the sequent, rather than sequences, eliminating the need for an explicit permutation rule. This corresponds to shifting commutativity of assumptions and derivations outside the sequent calculus, whereas LK embeds it within the system itself. 

### Relation to analytic tableaux[[edit][212]]

For certain formulations (i.e. variants) of the sequent calculus, a proof in such a calculus is isomorphic to an upside-down, closed [analytic tableau][81].[[25]][213]

### Structural rules[[edit][214]]

The structural rules deserve some additional discussion. 

Weakening (W) allows the addition of arbitrary elements to a sequence. Intuitively, this is allowed in the antecedent because we can always restrict the scope of our proof (if all cars have wheels, then it's safe to say that all black cars have wheels); and in the succedent because we can always allow for alternative conclusions (if all cars have wheels, then it's safe to say that all cars have either wheels or wings). 

Contraction (C) and Permutation (P) assure that neither the order (P) nor the multiplicity of occurrences (C) of elements of the sequences matters. Thus, one could instead of [sequences][215] also consider [sets][216]. 

The extra effort of using sequences, however, is justified since part or all of the structural rules may be omitted. Doing so, one obtains the so-called [substructural logics][217]. 

### Properties of the system LK[[edit][218]]

This system of rules can be shown to be both [sound][114] and [complete][115] with respect to first-order logic, i.e. a statement A{displaystyle A}![A][123] follows [semantically][219] from a set of premises Γ{displaystyle Gamma }![Gamma ][170] (Γ⊨A){displaystyle (Gamma vDash A)}![\(Gamma vDash A\)][220] [iff][221] the sequent Γ⊢A{displaystyle Gamma vdash A}![Gamma vdash A][222] can be derived by the above rules.[[26]][223]

In the sequent calculus, the rule of [cut is admissible][224]. This result is also referred to as Gentzen's _Hauptsatz_ ("Main Theorem").[[2]][22][[3]][23]

## Variants[[edit][225]]

The above rules can be modified in various ways: 

### Minor structural alternatives[[edit][226]]

There is some freedom of choice regarding the technical details of how sequents and structural rules are formalized. As long as every derivation in LK can be effectively transformed to a derivation using the new rules and vice versa, the modified rules may still be called LK. 

First of all, as mentioned above, the sequents can be viewed to consist of sets or [multisets][211]. In this case, the rules for permuting and (when using sets) contracting formulae are obsolete. 

The rule of weakening will become admissible, when the axiom (I) is changed, such that any sequent of the form Γ,A⊢A,Δ{displaystyle Gamma ,Avdash A,Delta }![Gamma , A vdash A , Delta][227] can be concluded. This means that A{displaystyle A}![A][123] proves A{displaystyle A}![A][123] in any context. Any weakening that appears in a derivation can then be performed right at the start. This may be a convenient change when constructing proofs bottom-up. 

Independent of these one may also change the way in which contexts are split within the rules: In the cases (∧R),(∨L){displaystyle ({land }R),({lor }L)}![\({and}R\), \({or}L\)][228], and (→L){displaystyle ({rightarrow }L)}![\({rightarrow}L\)][229] the left context is somehow split into Γ{displaystyle Gamma }![Gamma ][170] and Σ{displaystyle Sigma }![Sigma ][175] when going upwards. Since contraction allows for the duplication of these, one may assume that the full context is used in both branches of the derivation. By doing this, one assures that no important premises are lost in the wrong branch. Using weakening, the irrelevant parts of the context can be eliminated later. 

### Absurdity[[edit][230]]

One can introduce ⊥{displaystyle bot }![bot ][231], the [absurdity constant][232] representing _false_, with the axiom: 

: ⊥⊢{displaystyle {cfrac {}{bot vdash quad }}}![{cfrac  {}{bot vdash quad }}][233]

Or if, as described above, weakening is to be an admissible rule, then with the axiom: 

: Γ,⊥⊢Δ{displaystyle {cfrac {}{Gamma ,bot vdash Delta }}}![{cfrac  {}{Gamma ,bot vdash Delta }}][234]

With ⊥{displaystyle bot }![bot ][231], negation can be subsumed as a special case of implication, via the definition ¬A⟺A→⊥{displaystyle neg Aiff Ato bot }![neg Aiff Ato bot ][235]. 

### Substructural logics[[edit][236]]

Alternatively, one may restrict or forbid the use of some of the structural rules. This yields a variety of [substructural logic][217] systems. They are generally weaker than LK (_i.e._, they have fewer theorems), and thus not complete with respect to the standard semantics of first-order logic. However, they have other interesting properties that have led to applications in theoretical [computer science][237] and [artificial intelligence][238]. 

### Intuitionistic sequent calculus: System LJ[[edit][239]]

Surprisingly, some small changes in the rules of LK suffice to turn it into a proof system for [intuitionistic logic][20].[[27]][240] To this end, one has to restrict to sequents with exactly one formula on the right-hand side, and modify the rules to maintain this invariant. For example, (∨L){displaystyle ({lor }L)}![\({or}L\)][241] is reformulated as follows (where C is an arbitrary formula): 

: Γ,A⊢CΣ,B⊢CΓ,Σ,A∨B⊢C(∨L){displaystyle {cfrac {Gamma ,Avdash Cqquad Sigma ,Bvdash C}{Gamma ,Sigma ,Alor Bvdash C}}quad ({lor }L)}![
  cfrac{Gamma, A vdash C qquad Sigma, B vdash C }{Gamma, Sigma, A or B vdash C} quad \({or}L\)
][242]

The resulting system is called LJ. It is sound and complete with respect to intuitionistic logic and admits a similar cut-elimination proof. This can be used in proving [disjunction and existence properties][243]. 

In fact, the only two rules in LK that need to be restricted to single-formula consequents are (→R){displaystyle ({to }R)}![\({to }R\)][244] and (¬R){displaystyle (neg R)}![\(neg R\)][245][[28]][246] (and the latter can be seen as a special case of the former, via ⊥{displaystyle bot }![bot ][231] as described above). When multi-formula consequents are interpreted as disjunctions, all of the other inference rules of LK are actually derivable in LJ, while the offending rule is 

: Γ,A⊢B∨CΓ⊢(A→B)∨C{displaystyle {cfrac {Gamma ,Avdash Blor C}{Gamma vdash (Ato B)lor C}}}![{cfrac  {Gamma ,Avdash Blor C}{Gamma vdash \(Ato B\)lor C}}][247]

This amounts to the propositional formula (A→(B∨C))→((A→B)∨C){displaystyle (Ato (Blor C))to ((Ato B)lor C)}![\(Ato \(Blor C\)\)to \(\(Ato B\)lor C\)][248], a classical tautology that is not constructively valid. 

## See also[[edit][249]]

1. ^ [_**a**_][250] [_**b**_][251] [Gentzen 1934][252], [Gentzen 1935][253].
2. ^ [_**a**_][254] [_**b**_][255] [Curry 1977][256], pp. 208–213, gives a 5-page proof of the elimination theorem. See also pages 188, 250.
3. ^ [_**a**_][257] [_**b**_][258] [Kleene 2009][259], pp. 453, gives a very brief proof of the cut-elimination theorem.
4. [**^][260]** [Curry 1977][256], pp. 189–244, calls Gentzen systems LC systems. Curry's emphasis is more on theory than on practical logic proofs.
5. [**^][261]** [Kleene 2009][259], pp. 440–516. This book is much more concerned with the theoretical, metamathematical implications of Gentzen-style sequent calculus than applications to practical logic proofs.
6. [**^][262]** [Kleene 2002][263], pp. 283–312, 331–361, defines Gentzen systems and proves various theorems within these systems, including Gödel's completeness theorem and Gentzen's theorem.
7. [**^][264]** [Smullyan 1995][265], pp. 101–127, gives a brief theoretical presentation of Gentzen systems. He uses the tableau proof layout style.
8. [**^][266]** [Curry 1977][256], pp. 184–244, compares natural deduction systems, denoted LA, and Gentzen systems, denoted LC. Curry's emphasis is more theoretical than practical.
9. [**^][267]** [Suppes 1999][268], pp. 25–150, is an introductory presentation of practical natural deduction of this kind. This became the basis of [System L][269].
10. [**^][270]** [Lemmon 1965][271] is an elementary introduction to practical natural deduction based on the convenient abbreviated proof layout style [System L][269] based on [Suppes 1999][268], pp. 25–150.
11. [**^][272]** Here, "whenever" is used as an informal abbreviation "for every assignment of values to the free variables in the judgment"
12. **[^][273]** [Shankar, Natarajan][274]; Owre, Sam; [Rushby, John M.][275]; Stringer-Calvert, David W. J. (2001-11-01). ["PVS Prover Guide"][276] (PDF). _User guide_. [SRI International][277]. Retrieved 2015-05-29. 
13. **[^][278]** For explanations of the disjunctive semantics for the right side of sequents, see [Curry 1977][256], pp. 189–190, [Kleene 2002][263], pp. 290, 297, [Kleene 2009][259], p. 441, [Hilbert & Bernays 1970][279], p. 385, [Smullyan 1995][265], pp. 104–105 and [Gentzen 1934][252], p. 180.
14. **[^][280]** [Buss 1998][281], p. 10
15. **[^][282]** [Gentzen 1934][252], p. 188. "Der Kalkül _NJ_ hat manche formale Unschönheiten."
16. **[^][283]** [Gentzen 1934][252], p. 191. "In dem klassischen Kalkül _NK_ nahm der Satz vom ausgeschlossenen Dritten eine Sonderstellung unter den Schlußweisen ein [...], indem er sich der Einführungs- und Beseitigungssystematik nicht einfügte. Bei dem im folgenden anzugebenden logistischen klassichen Kalkül _LK_ wird diese Sonderstellung aufgehoben."
17. **[^][284]** [Gentzen 1934][252], p. 191. "Die damit erreichte Symmetrie erweist sich als für die klassische Logik angemessener."
18. **[^][285]** [Gentzen 1934][252], p. 191. "Hiermit haben wir einige Gesichtspunkte zur Begründung der Aufstellung der folgenden Kalküle angegeben. Im wesentlichen ist ihre Form jedoch durch die Rücksicht auf den nachher zu beweisenden 'Hauptsatz' bestimmt und kann daher vorläufig nicht näher begründet werden."
19. **[^][286]** [Kleene 2002][263], p. 441.
20. ^ _**[a**_][287] _**[b**_][288] _**[c**_][289] [Applied Logic, Univ. of Cornell: Lecture 9][290]. Last Retrieved: 2016-06-25
21. **[^][291]** "Remember, the way that you [prove][292] an [implication][85] is by assuming the [hypothesis][293]." —[Philip Wadler][294], [on 2 November 2015, in his Keynote: "Propositions as Types". Minute 14:36 /55:28 of Code Mesh video clip ][295]
22. **[^][296]** Tait WW (2010). ["Gentzen's original consistency proof and the Bar Theorem"][297] (PDF). In Kahle R, Rathjen M. _Gentzen's Centenary: The Quest for Consistency_. New York: Springer. pp. 213–228. 
23. **[^][298]** Jan von Plato, _Elements of Logical Reasoning_, Cambridge University Press, 2014, p. 32.
24. **[^][299]** [Smullyan 1995][265], p. 107
25. **[^][300]** [Kleene 2002][263], p. 336, wrote in 1967 that "it was a major logical discovery by Gentzen 1934–5 that, when there is any (purely logical) proof of a proposition, there is a direct proof. The implications of this discovery are in theoretical logical investigations, rather than in building collections of proved formulas."
26. **[^][301]** [Gentzen 1934][252], p. 194, wrote: "Der Unterschied zwischen _intuitionistischer_ und _klassischer_ Logik ist bei den Kalkülen _LJ_ und _LK_ äußerlich ganz anderer Art als bei _NJ_ und _NK_. Dort bestand er in Weglassung bzw. Hinzunahme des Satzes vom ausgeschlossenen Dritten, während er hier durch die Sukzedensbedingung ausgedrückt wird." English translation: "The difference between _intuitionistic_ and _classical_ logic is in the case of the calculi _LJ_ and _LK_ of an extremely, totally different kind to the case of _NJ_ and _NK_. In the latter case, it consisted of the removal or addition respectively of the excluded middle rule, whereas in the former case, it is expressed through the succedent conditions."
27. **[^][302]** Structural Proof Theory (CUP, 2001), Sara Negri and Jan van Plato

## References[[edit][303]]

* Buss, Samuel R. (1998). "An introduction to proof theory". In Samuel R. Buss. _[Handbook of proof theory_][304]. Elsevier. pp. 1–78. [ISBN][305] [0-444-89840-9][306]. 
* [Curry, Haskell Brooks][307] (1977) [1963]. _Foundations of mathematical logic_. New York: Dover Publications Inc. [ISBN][305] [978-0-486-63462-3][308]. 
* [Gentzen, Gerhard Karl Erich][4] (1934). ["Untersuchungen über das logische Schließen. I"][309]. _Mathematische Zeitschrift_. **39** (2): 176–210. [doi][310]:[10.1007/BF01201353][311]. 
* [Gentzen, Gerhard Karl Erich][4] (1935). ["Untersuchungen über das logische Schließen. II"][312]. _Mathematische Zeitschrift_. **39** (3): 405–431. [doi][310]:[10.1007/bf01201363][313]. 
* [Girard, Jean-Yves][314]; Paul Taylor; Yves Lafont (1990) [1989]. _[Proofs and Types_][315]. Cambridge University Press (Cambridge Tracts in Theoretical Computer Science, 7). [ISBN][305] [0-521-37181-3][316]. 
* [Hilbert, David][6]; [Bernays, Paul][317] (1970) [1939]. _Grundlagen der Mathematik II_ (Second ed.). Berlin, New York: Springer-Verlag. [ISBN][305] [978-3-642-86897-9][318]. 
* [Kleene, Stephen Cole][319] (2009) [1952]. _Introduction to metamathematics_. Ishi Press International. [ISBN][305] [978-0-923891-57-2][320]. 
* [Kleene, Stephen Cole][319] (2002) [1967]. _Mathematical logic_. Mineola, New York: Dover Publications. [ISBN][305] [978-0-486-42533-7][321]. 
* [Lemmon, Edward John][322] (1965). _Beginning logic_. Thomas Nelson. [ISBN][305] [0-17-712040-1][323]. 
* [Smullyan, Raymond Merrill][324] (1995) [1968]. _First-order logic_. New York: Dover Publications. [ISBN][305] [978-0-486-68370-6][325]. 
* [Suppes, Patrick Colonel][326] (1999) [1957]. _Introduction to logic_. Mineola, New York: Dover Publications. [ISBN][305] [978-0-486-40687-9][327]. 

## External links[[edit][328]]

[1]: https://en.wikipedia.org/wiki/Argument "Argument"
[2]: /wiki/Tautology_(logic) "Tautology (logic)"
[3]: https://en.wikipedia.org/wiki/Sequent "Sequent"
[4]: https://en.wikipedia.org/wiki/Gerhard_Gentzen "Gerhard Gentzen"
[5]: https://en.wikipedia.org/wiki/Inference "Inference"
[6]: https://en.wikipedia.org/wiki/David_Hilbert "David Hilbert"
[7]: https://en.wikipedia.org/wiki/Theorem "Theorem"
[8]: https://en.wikipedia.org/wiki/First-order_logic "First-order logic"
[9]: https://en.wikipedia.org/wiki/Proof_calculus "Proof calculus"
[10]: https://en.wikipedia.org/wiki/Hilbert_system "Hilbert system"
[11]: https://en.wikipedia.org/wiki/Natural_deduction "Natural deduction"
[12]: /wiki/Quantification_(logic) "Quantification (logic)"
[13]: https://en.wikipedia.org/wiki/Propositional_calculus "Propositional calculus"
[14]: /w/index.php?title=Sequent_calculus&action=edit&section=1 "Edit section: Overview"
[15]: https://en.wikipedia.org/wiki/Proof_theory "Proof theory"
[16]: https://en.wikipedia.org/wiki/Mathematical_logic "Mathematical logic"
[17]: https://en.wikipedia.org/wiki/Formal_system "Formal system"
[18]: https://en.wikipedia.org#cite_note-gentzen19341935-1
[19]: https://en.wikipedia.org/wiki/Classical_logic "Classical logic"
[20]: https://en.wikipedia.org/wiki/Intuitionistic_logic "Intuitionistic logic"
[21]: https://en.wikipedia.org/wiki/Cut-elimination_theorem "Cut-elimination theorem"
[22]: https://en.wikipedia.org#cite_note-curry_cut_elimination-2
[23]: https://en.wikipedia.org#cite_note-kleene_cut_elimination-3
[24]: https://en.wikipedia.org/wiki/Metatheory "Metatheory"
[25]: https://en.wikipedia.org/wiki/Consistency "Consistency"
[26]: https://en.wikipedia.org/wiki/Gentzen%27s_consistency_proof "Gentzen's consistency proof"
[27]: https://en.wikipedia.org/wiki/G%C3%B6del%27s_incompleteness_theorems "Gödel's incompleteness theorems"
[28]: https://en.wikipedia.org#cite_note-4
[29]: https://en.wikipedia.org#cite_note-5
[30]: https://en.wikipedia.org#cite_note-6
[31]: https://en.wikipedia.org#cite_note-7
[32]: https://en.wikipedia.org/wiki/Automated_deduction "Automated deduction"
[33]: /w/index.php?title=Sequent_calculus&action=edit&section=2 "Edit section: Hilbert-style deduction systems"
[34]: /wiki/Judgment_(mathematical_logic) "Judgment (mathematical logic)"
[35]: https://en.wikipedia.org/wiki/Hilbert-style_deduction_system "Hilbert-style deduction system"
[36]: https://wikimedia.org/api/rest_v1/media/math/render/svg/47136aad860d145f75f3eed3022df827cee94d7a
[37]: https://en.wikipedia.org/wiki/Well-formed_formula "Well-formed formula"
[38]: https://en.wikipedia.org/wiki/Higher-order_logic "Higher-order logic"
[39]: https://en.wikipedia.org/wiki/Modal_logic "Modal logic"
[40]: https://en.wikipedia.org/wiki/Deduction_theorem "Deduction theorem"
[41]: /w/index.php?title=Sequent_calculus&action=edit&section=3 "Edit section: Natural deduction systems"
[42]: https://wikimedia.org/api/rest_v1/media/math/render/svg/834feee88f9259b496df4ee0536e0f6d14b0b42d
[43]: https://wikimedia.org/api/rest_v1/media/math/render/svg/1aed3b5def921afbe6cc48aaf8f9b11c6f1c1e2d
[44]: https://wikimedia.org/api/rest_v1/media/math/render/svg/ce8a1b7b3bc3c790054d93629fc3b08cd1da1fd0
[45]: /wiki/Turnstile_(symbol) "Turnstile (symbol)"
[46]: https://wikimedia.org/api/rest_v1/media/math/render/svg/a0c0d30cf8cb7dba179e317fcde9583d842e80f6
[47]: https://en.wikipedia.org#cite_note-8
[48]: https://en.wikipedia.org#cite_note-9
[49]: https://en.wikipedia.org#cite_note-10
[50]: https://wikimedia.org/api/rest_v1/media/math/render/svg/aaa02acb341837055be929c605bd7a9edf73b6de
[51]: https://en.wikipedia.org#cite_note-11
[52]: https://wikimedia.org/api/rest_v1/media/math/render/svg/6bc2435b217c1a0f46f8a517ffa225c6f9440e81
[53]: https://wikimedia.org/api/rest_v1/media/math/render/svg/3ec73b8bc9abc3efb934f5a6ec2803713771f4bc
[54]: https://wikimedia.org/api/rest_v1/media/math/render/svg/8643d36d902d2eb9eb1eab467ade9f9ac63b05d2
[55]: https://wikimedia.org/api/rest_v1/media/math/render/svg/844ec7c4c2c8d531fcedd7b4e5b3ce19a6ce2aa5
[56]: /w/index.php?title=Sequent_calculus&action=edit&section=4 "Edit section: Sequent calculus systems"
[57]: https://wikimedia.org/api/rest_v1/media/math/render/svg/e0c5b66a8fed889a35d2ea5ff95f0c604351346d
[58]: https://en.wikipedia.org#cite_note-pvs-prover-12
[59]: https://wikimedia.org/api/rest_v1/media/math/render/svg/82cda0578ec6b48774c541ecb9bee4a90176e62f
[60]: https://wikimedia.org/api/rest_v1/media/math/render/svg/a601995d55609f2d9f5e233e36fbe9ea26011b3b
[61]: https://wikimedia.org/api/rest_v1/media/math/render/svg/c3c9a2c7b599b37105512c5d570edc034056dd40
[62]: https://en.wikipedia.org#cite_note-13
[63]: https://en.wikipedia.org#cite_note-14
[64]: https://wikimedia.org/api/rest_v1/media/math/render/svg/7a12c681fb9761c410c8b55147e3f57ae7094107
[65]: https://wikimedia.org/api/rest_v1/media/math/render/svg/a4ef47d624521644f3f5a9b10164b568f9384019
[66]: https://wikimedia.org/api/rest_v1/media/math/render/svg/da43ee3920c33bda6817e59937c45b365faaab33
[67]: https://wikimedia.org/api/rest_v1/media/math/render/svg/198fef72f087d0d42752f15d4981495e4f86530e
[68]: https://en.wikipedia.org/wiki/De_Morgan%27s_laws "De Morgan's laws"
[69]: /w/index.php?title=Sequent_calculus&action=edit&section=5 "Edit section: Distinction between natural deduction and sequent calculus"
[70]: https://en.wikipedia.org#cite_note-15
[71]: https://en.wikipedia.org/wiki/Law_of_excluded_middle "Law of excluded middle"
[72]: https://en.wikipedia.org#cite_note-16
[73]: https://en.wikipedia.org#cite_note-17
[74]: https://en.wikipedia.org#cite_note-18
[75]: /w/index.php?title=Sequent_calculus&action=edit&section=6 "Edit section: Origin of word "sequent""
[76]: https://en.wikipedia.org#cite_note-19
[77]: /w/index.php?title=Sequent_calculus&action=edit&section=7 "Edit section: Proving logical formulas"
[78]: https://upload.wikimedia.org/wikipedia/commons/thumb/0/0d/Sequent_calculus_proof_tree_example.png/220px-Sequent_calculus_proof_tree_example.png
[79]: /w/index.php?title=Sequent_calculus&action=edit&section=8 "Edit section: Reduction trees"
[80]: https://en.wikipedia.org/wiki/Propositional_logic "Propositional logic"
[81]: https://en.wikipedia.org/wiki/Method_of_analytic_tableaux "Method of analytic tableaux"
[82]: https://en.wikipedia.org#cite_note-Cornell09-20
[83]: https://wikimedia.org/api/rest_v1/media/math/render/svg/8f0a4c16c6f572e16a452d3d9fd468a43731b2d7
[84]: https://wikimedia.org/api/rest_v1/media/math/render/svg/133ab0203763a2827d36da4fe413745917b158d5
[85]: https://en.wikipedia.org/wiki/Logical_consequence "Logical consequence"
[86]: https://en.wikipedia.org#cite_note-Wadler-21
[87]: https://wikimedia.org/api/rest_v1/media/math/render/svg/95d3d73f6d661d3f9ae8be02d36507820b856da8
[88]: https://wikimedia.org/api/rest_v1/media/math/render/svg/5bc118d330b9c6c62d2b1ac0478c89bdef91e9cc
[89]: https://en.wikipedia.org/wiki/Logical_conjunction "Logical conjunction"
[90]: https://wikimedia.org/api/rest_v1/media/math/render/svg/4779ea20cd9ed6cebb2a062c2d50c17c3fb436b0
[91]: https://en.wikipedia.org/wiki/Logical_disjunction "Logical disjunction"
[92]: https://wikimedia.org/api/rest_v1/media/math/render/svg/af89bce99551b51386e65b5adb9eabae3164a302
[93]: https://wikimedia.org/api/rest_v1/media/math/render/svg/052d86a90b348baca28b86158fdd01d2d0e3eec7
[94]: /wiki/Tree_(graph_theory) "Tree (graph theory)"
[95]: https://en.wikipedia.org#cite_note-Tait-22
[96]: https://en.wikipedia.org#cite_note-23
[97]: https://wikimedia.org/api/rest_v1/media/math/render/svg/c282cb8ae3beadf16650b2ab642ca0afeea33ef9
[98]: https://wikimedia.org/api/rest_v1/media/math/render/svg/e30d39da25cd5420fa1eb877b0ef709d25ce8014
[99]: https://wikimedia.org/api/rest_v1/media/math/render/svg/f04cb04a42217d9a01afb8e16bcef2677456ad66
[100]: https://wikimedia.org/api/rest_v1/media/math/render/svg/691a365c5515d597c3f303893ba5771ff10f2b59
[101]: https://wikimedia.org/api/rest_v1/media/math/render/svg/0f8515d7e76827f1558c0cb750652d9a6d661eae
[102]: https://wikimedia.org/api/rest_v1/media/math/render/svg/446cf26d3c9f49c32009cd21fec238461e6b99ef
[103]: https://wikimedia.org/api/rest_v1/media/math/render/svg/2ae00b8a09336294c1f58277bfc050041ee5fdd8
[104]: https://wikimedia.org/api/rest_v1/media/math/render/svg/73f29f765b49fdb2c3407538b663ca304e3ea856
[105]: https://wikimedia.org/api/rest_v1/media/math/render/svg/34b1a0413a36dac30a164972441b1164f1e07a10
[106]: https://wikimedia.org/api/rest_v1/media/math/render/svg/f3630e3a5ac83c683ec1218f9c5fec22ddcaa6d6
[107]: https://wikimedia.org/api/rest_v1/media/math/render/svg/905a18e006d22b982738c74abc957a0c8426fb8c
[108]: https://wikimedia.org/api/rest_v1/media/math/render/svg/725b6de09cf7e2c5cea9462bd01a269b840e1762
[109]: https://wikimedia.org/api/rest_v1/media/math/render/svg/e9f89a9461d11f7a26e5f6bc49e134d241e351eb
[110]: https://wikimedia.org/api/rest_v1/media/math/render/svg/23355941edfddad9ee32b1e0b9fd1ccd0eed2108
[111]: https://wikimedia.org/api/rest_v1/media/math/render/svg/db367ea0fe2ad1898994f4068458447ee4c9f117
[112]: https://wikimedia.org/api/rest_v1/media/math/render/svg/ed16ae229be8f20f91bb44633aeab82faaff588e
[113]: https://wikimedia.org/api/rest_v1/media/math/render/svg/8a4b54ff7eb17107d9cedeea1ef09f247e788ee1
[114]: https://en.wikipedia.org/wiki/Soundness "Soundness"
[115]: /wiki/Completeness_(logic) "Completeness (logic)"
[116]: /w/index.php?title=Sequent_calculus&action=edit&section=9 "Edit section: Relation to standard axiomatizations"
[117]: https://en.wikipedia.org/wiki/Frege%27s_propositional_calculus "Frege's propositional calculus"
[118]: https://en.wikipedia.org/wiki/Propositional_calculus#Example_1._Simple_axiom_system "Propositional calculus"
[119]: https://en.wikipedia.org/wiki/Sequent_calculus#Example_derivations "Sequent calculus"
[120]: /w/index.php?title=Sequent_calculus&action=edit&section=10 "Edit section: The system LK"
[121]: https://en.wikipedia.org/wiki/Rule_of_inference "Rule of inference"
[122]: /w/index.php?title=Sequent_calculus&action=edit&section=11 "Edit section: Inference rules"
[123]: https://wikimedia.org/api/rest_v1/media/math/render/svg/7daff47fa58cdfd29dc333def748ff5fa4c923e3
[124]: https://wikimedia.org/api/rest_v1/media/math/render/svg/fcd9d1ee2fb8d06dc14078b25795e2427042736d
[125]: https://wikimedia.org/api/rest_v1/media/math/render/svg/eed3e3db6cc2028a183af948212ed2551d25c954
[126]: https://en.wikipedia.org/wiki/Sequent_calculus#Structural_rules "Sequent calculus"
[127]: https://wikimedia.org/api/rest_v1/media/math/render/svg/65658b7b223af9e1acc877d848888ecdb4466560
[128]: https://wikimedia.org/api/rest_v1/media/math/render/svg/87f9e315fd7e2ba406057a97300593c4802b53e4
[129]: https://wikimedia.org/api/rest_v1/media/math/render/svg/b8a6208ec717213d4317e666f1ae872e00620a0d
[130]: https://en.wikipedia.org/wiki/Free_variables_and_bound_variables "Free variables and bound variables"
[131]: https://wikimedia.org/api/rest_v1/media/math/render/svg/bfc1a1a9c4c0f8d5df989c98aa2773ed657c5937
[132]: https://wikimedia.org/api/rest_v1/media/math/render/svg/77ed842b6b90b2fdd825320cf8e5265fa937b583
[133]: https://wikimedia.org/api/rest_v1/media/math/render/svg/dbfa2b158d3cc834d4594e5e6854d010635d5144
[134]: https://wikimedia.org/api/rest_v1/media/math/render/svg/424f8e6ea0804e568afc75a67d5d520e8633a4b9
[135]: https://wikimedia.org/api/rest_v1/media/math/render/svg/3fc4634ed597b2de7e15c6085fdade2b918f63b0
[136]: https://wikimedia.org/api/rest_v1/media/math/render/svg/134e48c63b788be291ce05f72eca6057a0fb9f11
[137]: https://wikimedia.org/api/rest_v1/media/math/render/svg/5f4aa3025e892cbaae55fa86843d0cda54d18b42
[138]: https://wikimedia.org/api/rest_v1/media/math/render/svg/f62de5fa65ee246c0938bf516a8d285df56adb26
[139]: https://wikimedia.org/api/rest_v1/media/math/render/svg/b782f4107c0f8f8ef28ef9b19302d22ed6c352ca
[140]: /wiki/Quantifier_(logic) "Quantifier (logic)"
[141]: https://wikimedia.org/api/rest_v1/media/math/render/svg/cd471768c68275b1e35a5749ad47195878906eab
[142]: https://wikimedia.org/api/rest_v1/media/math/render/svg/7d208fdfcb6f456eee663cb91d1f1548f2eab8f1
[143]: https://wikimedia.org/api/rest_v1/media/math/render/svg/500fe143d83c178c11cc25efc1a4389664cae8fc
[144]: https://wikimedia.org/api/rest_v1/media/math/render/svg/21eb15792ba3b0927b020c420b9184fa34a9f211
[145]: https://wikimedia.org/api/rest_v1/media/math/render/svg/fe8666fdc967bf1d9c852388954b062c41708bce
[146]: https://wikimedia.org/api/rest_v1/media/math/render/svg/ee9e21a14c006746fa36798d4468c6c571034f9b
[147]: https://wikimedia.org/api/rest_v1/media/math/render/svg/e0f0371c9864ec0797ff1d1253bd9e88e5679bb7
[148]: https://wikimedia.org/api/rest_v1/media/math/render/svg/72d7d2c7999285db0e48db1d92736c07ac632a7b
[149]: https://wikimedia.org/api/rest_v1/media/math/render/svg/ecac447fa6dcace2441b1a58aebef5bb8cf70190
[150]: https://wikimedia.org/api/rest_v1/media/math/render/svg/6bcaa032d8989769320791e1b8144a0b44313a90
[151]: https://wikimedia.org/api/rest_v1/media/math/render/svg/30dbf25817cdb63dc256f3e679e5acf0d843c9d9
[152]: https://wikimedia.org/api/rest_v1/media/math/render/svg/edab013ee777226405e0e78f5714b1368ad1adad
[153]: https://wikimedia.org/api/rest_v1/media/math/render/svg/958b77d782b26c79d96ced1a09fe9dbf7d2b1232
[154]: https://wikimedia.org/api/rest_v1/media/math/render/svg/2cee48be60cb7105a90d1fc022d6e7f641656a2f
[155]: https://wikimedia.org/api/rest_v1/media/math/render/svg/09a30ed98427f2b670efd26034258aa191760913
[156]: https://wikimedia.org/api/rest_v1/media/math/render/svg/35ad7e06b254cdd8eddb2978de23f9918585945f
[157]: https://wikimedia.org/api/rest_v1/media/math/render/svg/7af4f3d3feecfb1e4624e73ff5617968e68e9d37
[158]: https://wikimedia.org/api/rest_v1/media/math/render/svg/9f5ea107c7551050a3ed3656dd85fc28864fa115
[159]: https://wikimedia.org/api/rest_v1/media/math/render/svg/baab340ba762b65cded5001b6a636a632405dce7
[160]: https://wikimedia.org/api/rest_v1/media/math/render/svg/4e4fc949198f78c3bc84c9c0342dde9510eecaf8
[161]: https://wikimedia.org/api/rest_v1/media/math/render/svg/5efe37c082272aff5ceeb23b57962d1c00256aad
[162]: https://wikimedia.org/api/rest_v1/media/math/render/svg/94c91648f1efcee4ba02658be3c4997c6465c873
[163]: https://wikimedia.org/api/rest_v1/media/math/render/svg/e18abac25b0444108a597684e2a956d712d5d66d
[164]: https://wikimedia.org/api/rest_v1/media/math/render/svg/52c921f5e25f559965063ec7d9c7c9909c98ecfd
[165]: /w/index.php?title=Sequent_calculus&action=edit&section=12 "Edit section: An intuitive explanation"
[166]: https://wikimedia.org/api/rest_v1/media/math/render/svg/cfa20a1f3da6bd4435da0ea659af94736ec580de
[167]: https://wikimedia.org/api/rest_v1/media/math/render/svg/32769037c408874e1890f77554c65f39c523ebe2
[168]: https://wikimedia.org/api/rest_v1/media/math/render/svg/74954195333a8593163b93a9688695b8dc74da55
[169]: https://wikimedia.org/api/rest_v1/media/math/render/svg/99104d335301ddc3a553ef95f1f04ce9b3a2f944
[170]: https://wikimedia.org/api/rest_v1/media/math/render/svg/4cfde86a3f7ec967af9955d0988592f0693d2b19
[171]: https://wikimedia.org/api/rest_v1/media/math/render/svg/06eb33da4ce0e2657ef5f2109fc3a729a97f75db
[172]: https://wikimedia.org/api/rest_v1/media/math/render/svg/cc351bde4496ea10e3bd324e0e230209dc3fe936
[173]: https://wikimedia.org/api/rest_v1/media/math/render/svg/940011a91dd40805a7110f35f32363866a542984
[174]: https://wikimedia.org/api/rest_v1/media/math/render/svg/9320a849485307c9b95f9a77d1a2d3e32e384c9a
[175]: https://wikimedia.org/api/rest_v1/media/math/render/svg/9e1f558f53cda207614abdf90162266c70bc5c1e
[176]: https://en.wikipedia.org/wiki/Completeness_of_atomic_initial_sequents "Completeness of atomic initial sequents"
[177]: https://en.wikipedia.org/wiki/Atomic_formula "Atomic formula"
[178]: https://wikimedia.org/api/rest_v1/media/math/render/svg/4acf12661d69611db01ff7f036bf8c5b5dfec902
[179]: /w/index.php?title=Sequent_calculus&action=edit&section=13 "Edit section: Example derivations"
[180]: https://wikimedia.org/api/rest_v1/media/math/render/svg/0a597d924488a6f9f0013c4d78892695d0316dea
[181]: https://wikimedia.org/api/rest_v1/media/math/render/svg/af4c0ea2fa2a0d2bb469a112e99a1c06ebd10993
[182]: https://wikimedia.org/api/rest_v1/media/math/render/svg/875a86dc658240273b3f98667d8c4d1553dbfd62
[183]: https://wikimedia.org/api/rest_v1/media/math/render/svg/de88ac06fdac52b05a081982de2cb95cabb9f1b4
[184]: https://wikimedia.org/api/rest_v1/media/math/render/svg/830443951202077c150632be2a8ce7a525f2adf0
[185]: https://wikimedia.org/api/rest_v1/media/math/render/svg/d3e016a72c9022a9cf3bc0316443192b3cb9e736
[186]: https://wikimedia.org/api/rest_v1/media/math/render/svg/21ef3605df7ffe41d5bc9f80996c477bde9501fe
[187]: https://wikimedia.org/api/rest_v1/media/math/render/svg/a96c5327a2e9cba39cac8927ba67b2db8c3c9bdc
[188]: https://wikimedia.org/api/rest_v1/media/math/render/svg/fc893522a935b79cc2318d36e2a4b2e791aa380f
[189]: https://wikimedia.org/api/rest_v1/media/math/render/svg/8b48c763a9fc73e480e50256c0f4e6e39ce93ecb
[190]: https://wikimedia.org/api/rest_v1/media/math/render/svg/b6ebe97f7404315f8e1f2d6aef016eebeef1a7db
[191]: https://wikimedia.org/api/rest_v1/media/math/render/svg/6971c2ccdc46055428e915cfab7647f43e8eff7a
[192]: https://wikimedia.org/api/rest_v1/media/math/render/svg/d4df9dcce88e91c4ad87ab8fda951812f3808794
[193]: https://wikimedia.org/api/rest_v1/media/math/render/svg/0ada40ee77899adfc020555632a2284b503b76fd
[194]: https://wikimedia.org/api/rest_v1/media/math/render/svg/75cc4c9d60dc9bff9ba9f23e6be9aee11da3a662
[195]: https://wikimedia.org/api/rest_v1/media/math/render/svg/ccb0fc1ca85682253e31d5eaa06ca132326ae178
[196]: https://wikimedia.org/api/rest_v1/media/math/render/svg/0db99501c0a3b342d0c59be70b59726fc6615c6a
[197]: https://wikimedia.org/api/rest_v1/media/math/render/svg/ed66095d797692a15655e55f89c923d40c525000
[198]: https://wikimedia.org/api/rest_v1/media/math/render/svg/1dde32d081e702621b915fca9a20c4fa4391fc66
[199]: https://wikimedia.org/api/rest_v1/media/math/render/svg/915618907fe26b859073b682c29ed4279ae8f672
[200]: https://wikimedia.org/api/rest_v1/media/math/render/svg/57874285d7aeb19ffe937ff9fbd7d43902e3f9b7
[201]: https://wikimedia.org/api/rest_v1/media/math/render/svg/4ab03005dc2e259301d2b53b51047f682b4bd96e
[202]: https://wikimedia.org/api/rest_v1/media/math/render/svg/6efb670ae65168487bde1052191d8d5f7bfaba63
[203]: https://wikimedia.org/api/rest_v1/media/math/render/svg/db63ae042293fa230cd998c76cb02db5072ca9ba
[204]: https://wikimedia.org/api/rest_v1/media/math/render/svg/3e75d6ef9a830d511a67255357b15681bef6ee6d
[205]: https://wikimedia.org/api/rest_v1/media/math/render/svg/47ea876c18025c3ff9e3c0f24358dd0c88916c2a
[206]: https://wikimedia.org/api/rest_v1/media/math/render/svg/107a37c2ce6a603d9e50295eff569df7a63cdaa4
[207]: https://wikimedia.org/api/rest_v1/media/math/render/svg/1c1748ef057a1db142523773d313bccaba8ac499
[208]: https://wikimedia.org/api/rest_v1/media/math/render/svg/d857d9a985d9afa2509f026d20f2b9ad849ed136
[209]: https://wikimedia.org/api/rest_v1/media/math/render/svg/2f631e766811804d168be662bb88e302434e2f4e
[210]: https://wikimedia.org/api/rest_v1/media/math/render/svg/f098eeafee24eba88b8842a6d863edf46648418b
[211]: https://en.wikipedia.org/wiki/Multiset "Multiset"
[212]: /w/index.php?title=Sequent_calculus&action=edit&section=14 "Edit section: Relation to analytic tableaux"
[213]: https://en.wikipedia.org#cite_note-25
[214]: /w/index.php?title=Sequent_calculus&action=edit&section=15 "Edit section: Structural rules"
[215]: https://en.wikipedia.org/wiki/Sequence "Sequence"
[216]: /wiki/Set_(mathematics) "Set (mathematics)"
[217]: https://en.wikipedia.org/wiki/Substructural_logic "Substructural logic"
[218]: /w/index.php?title=Sequent_calculus&action=edit&section=16 "Edit section: Properties of the system LK"
[219]: https://en.wikipedia.org/wiki/Semantics "Semantics"
[220]: https://wikimedia.org/api/rest_v1/media/math/render/svg/ba19426707017b12b9632fa99d554349ce3b80b1
[221]: https://en.wikipedia.org/wiki/Iff "Iff"
[222]: https://wikimedia.org/api/rest_v1/media/math/render/svg/0b4c81934043042b8bcc9ce610a307f52d95d92a
[223]: https://en.wikipedia.org#cite_note-26
[224]: https://en.wikipedia.org/wiki/Cut-elimination "Cut-elimination"
[225]: /w/index.php?title=Sequent_calculus&action=edit&section=17 "Edit section: Variants"
[226]: /w/index.php?title=Sequent_calculus&action=edit&section=18 "Edit section: Minor structural alternatives"
[227]: https://wikimedia.org/api/rest_v1/media/math/render/svg/41921175ae5001905da502cadc6a76f363517012
[228]: https://wikimedia.org/api/rest_v1/media/math/render/svg/3adf12cc068be705ed193fc7e7d39092e03434bf
[229]: https://wikimedia.org/api/rest_v1/media/math/render/svg/922b9ca3903425fdc0ac9e1b02753940606003b0
[230]: /w/index.php?title=Sequent_calculus&action=edit&section=19 "Edit section: Absurdity"
[231]: https://wikimedia.org/api/rest_v1/media/math/render/svg/f282c7bc331cc3bfcf1c57f1452cc23c022f58de
[232]: https://en.wikipedia.org/wiki/Principle_of_explosion "Principle of explosion"
[233]: https://wikimedia.org/api/rest_v1/media/math/render/svg/2d479518157c11480bd5b912bf21f46466da1bd6
[234]: https://wikimedia.org/api/rest_v1/media/math/render/svg/e3b4f8bb2248e680cda26a5be656e8aad8225fd0
[235]: https://wikimedia.org/api/rest_v1/media/math/render/svg/24107d8ce94e834dd0afcb4c0542da65038dceca
[236]: /w/index.php?title=Sequent_calculus&action=edit&section=20 "Edit section: Substructural logics"
[237]: https://en.wikipedia.org/wiki/Computer_science "Computer science"
[238]: https://en.wikipedia.org/wiki/Artificial_intelligence "Artificial intelligence"
[239]: /w/index.php?title=Sequent_calculus&action=edit&section=21 "Edit section: Intuitionistic sequent calculus: System LJ"
[240]: https://en.wikipedia.org#cite_note-27
[241]: https://wikimedia.org/api/rest_v1/media/math/render/svg/cb47080d5e3a42b7c274c50607da72a9093fed24
[242]: https://wikimedia.org/api/rest_v1/media/math/render/svg/3c628632b40ff461bdcb22b1e0beed4d90ea7fc9
[243]: https://en.wikipedia.org/wiki/Disjunction_and_existence_properties "Disjunction and existence properties"
[244]: https://wikimedia.org/api/rest_v1/media/math/render/svg/8b786d00a141ac2db6f0bed4dca03b964e19e22c
[245]: https://wikimedia.org/api/rest_v1/media/math/render/svg/087e80aa1838161d093a7f629d9c92ad2daa4386
[246]: https://en.wikipedia.org#cite_note-28
[247]: https://wikimedia.org/api/rest_v1/media/math/render/svg/aecf7b0eb441fa1695323f6bc4270755c4fde815
[248]: https://wikimedia.org/api/rest_v1/media/math/render/svg/fe8546dcff331ec12104d2fbfcb363df8c802f98
[249]: /w/index.php?title=Sequent_calculus&action=edit&section=22 "Edit section: See also"
[250]: https://en.wikipedia.org#cite_ref-gentzen19341935_1-0
[251]: https://en.wikipedia.org#cite_ref-gentzen19341935_1-1
[252]: https://en.wikipedia.org#CITEREFGentzen1934
[253]: https://en.wikipedia.org#CITEREFGentzen1935
[254]: https://en.wikipedia.org#cite_ref-curry_cut_elimination_2-0
[255]: https://en.wikipedia.org#cite_ref-curry_cut_elimination_2-1
[256]: https://en.wikipedia.org#CITEREFCurry1977
[257]: https://en.wikipedia.org#cite_ref-kleene_cut_elimination_3-0
[258]: https://en.wikipedia.org#cite_ref-kleene_cut_elimination_3-1
[259]: https://en.wikipedia.org#CITEREFKleene2009
[260]: https://en.wikipedia.org#cite_ref-4
[261]: https://en.wikipedia.org#cite_ref-5
[262]: https://en.wikipedia.org#cite_ref-6
[263]: https://en.wikipedia.org#CITEREFKleene2002
[264]: https://en.wikipedia.org#cite_ref-7
[265]: https://en.wikipedia.org#CITEREFSmullyan1995
[266]: https://en.wikipedia.org#cite_ref-8
[267]: https://en.wikipedia.org#cite_ref-9
[268]: https://en.wikipedia.org#CITEREFSuppes1999
[269]: https://en.wikipedia.org/wiki/System_L "System L"
[270]: https://en.wikipedia.org#cite_ref-10
[271]: https://en.wikipedia.org#CITEREFLemmon1965
[272]: https://en.wikipedia.org#cite_ref-11
[273]: https://en.wikipedia.org#cite_ref-pvs-prover_12-0
[274]: https://en.wikipedia.org/wiki/Natarajan_Shankar "Natarajan Shankar"
[275]: https://en.wikipedia.org/wiki/John_Rushby "John Rushby"
[276]: http://pvs.csl.sri.com/doc/pvs-prover-guide.pdf
[277]: https://en.wikipedia.org/wiki/SRI_International "SRI International"
[278]: https://en.wikipedia.org#cite_ref-13
[279]: https://en.wikipedia.org#CITEREFHilbertBernays1970
[280]: https://en.wikipedia.org#cite_ref-14
[281]: https://en.wikipedia.org#CITEREFBuss1998
[282]: https://en.wikipedia.org#cite_ref-15
[283]: https://en.wikipedia.org#cite_ref-16
[284]: https://en.wikipedia.org#cite_ref-17
[285]: https://en.wikipedia.org#cite_ref-18
[286]: https://en.wikipedia.org#cite_ref-19
[287]: https://en.wikipedia.org#cite_ref-Cornell09_20-0
[288]: https://en.wikipedia.org#cite_ref-Cornell09_20-1
[289]: https://en.wikipedia.org#cite_ref-Cornell09_20-2
[290]: http://www.cs.cornell.edu/courses/cs4860/2009sp/lec-09.pdf
[291]: https://en.wikipedia.org#cite_ref-Wadler_21-0
[292]: /wiki/Proof_(truth) "Proof (truth)"
[293]: https://en.wikipedia.org/wiki/Hypothesis "Hypothesis"
[294]: https://en.wikipedia.org/wiki/Philip_Wadler "Philip Wadler"
[295]: https://www.youtube.com/watch?v=OGF-TGd-CIo&list=PLWbHc_FXPo2jB6IZ887vLXsPoympL3KEy&index=11
[296]: https://en.wikipedia.org#cite_ref-Tait_22-0
[297]: http://home.uchicago.edu/~wwtx/Gentzen.original.pdf
[298]: https://en.wikipedia.org#cite_ref-23
[299]: https://en.wikipedia.org#cite_ref-25
[300]: https://en.wikipedia.org#cite_ref-26
[301]: https://en.wikipedia.org#cite_ref-27
[302]: https://en.wikipedia.org#cite_ref-28
[303]: /w/index.php?title=Sequent_calculus&action=edit&section=24 "Edit section: References"
[304]: http://math.ucsd.edu/~sbuss/ResearchWeb/handbookI/
[305]: https://en.wikipedia.org/wiki/International_Standard_Book_Number "International Standard Book Number"
[306]: https://en.wikipedia.org/wiki/Special%3ABookSources/0-444-89840-9 "Special:BookSources/0-444-89840-9"
[307]: https://en.wikipedia.org/wiki/Haskell_Curry "Haskell Curry"
[308]: https://en.wikipedia.org/wiki/Special%3ABookSources/978-0-486-63462-3 "Special:BookSources/978-0-486-63462-3"
[309]: http://gdz.sub.uni-goettingen.de/dms/resolveppn/?PPN=GDZPPN002375508
[310]: https://en.wikipedia.org/wiki/Digital_object_identifier "Digital object identifier"
[311]: https://doi.org/10.1007/BF01201353
[312]: http://gdz.sub.uni-goettingen.de/dms/resolveppn/?PPN=GDZPPN002375605
[313]: https://doi.org/10.1007/bf01201363
[314]: https://en.wikipedia.org/wiki/Jean-Yves_Girard "Jean-Yves Girard"
[315]: http://www.paultaylor.eu/stable/Proofs%2BTypes.html
[316]: https://en.wikipedia.org/wiki/Special%3ABookSources/0-521-37181-3 "Special:BookSources/0-521-37181-3"
[317]: https://en.wikipedia.org/wiki/Paul_Bernays "Paul Bernays"
[318]: https://en.wikipedia.org/wiki/Special%3ABookSources/978-3-642-86897-9 "Special:BookSources/978-3-642-86897-9"
[319]: https://en.wikipedia.org/wiki/Stephen_Cole_Kleene "Stephen Cole Kleene"
[320]: https://en.wikipedia.org/wiki/Special%3ABookSources/978-0-923891-57-2 "Special:BookSources/978-0-923891-57-2"
[321]: https://en.wikipedia.org/wiki/Special%3ABookSources/978-0-486-42533-7 "Special:BookSources/978-0-486-42533-7"
[322]: https://en.wikipedia.org/wiki/John_Lemmon "John Lemmon"
[323]: https://en.wikipedia.org/wiki/Special%3ABookSources/0-17-712040-1 "Special:BookSources/0-17-712040-1"
[324]: https://en.wikipedia.org/wiki/Raymond_Smullyan "Raymond Smullyan"
[325]: https://en.wikipedia.org/wiki/Special%3ABookSources/978-0-486-68370-6 "Special:BookSources/978-0-486-68370-6"
[326]: https://en.wikipedia.org/wiki/Patrick_Suppes "Patrick Suppes"
[327]: https://en.wikipedia.org/wiki/Special%3ABookSources/978-0-486-40687-9 "Special:BookSources/978-0-486-40687-9"
[328]: /w/index.php?title=Sequent_calculus&action=edit&section=25 "Edit section: External links"

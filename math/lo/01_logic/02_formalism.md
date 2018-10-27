# Formalism

By formalization we understand the faithful representation or simulation of the reasoning processes of mathematics in general (pure logic), or of a particular mathematical theory (applied logic), within an activity that is, in principle, driven exclusively by the form or syntax of mathematical statements, ignoring their meaning.


## Formal logical systems
At its core, mathematical logic deals with mathematical concepts expressed using formal logical systems. These systems, though they differ in many details, share the common property of considering only expressions in a fixed formal language. The systems of propositional logic and first-order logic are the most widely studied today, because of their applicability to foundations of mathematics and because of their desirable proof-theoretic properties. Stronger classical logics such as second-order logic or infinitary logic are also studied, along with nonclassical logics such as intuitionistic logic.



## Formal language
https://www.wikiwand.com/en/Formal_language

A **formal language** is a set of strings of symbols together with a set of rules that are specific to it.



---


Formal language is needed because, although processing natural language has come a long way, natural languages like English can be verbose and ambiguous. By transforming a natural language into strictly defined formal symbolic language, we can focus more on the logical structure of sentences, disregarding their meaning, in order to emphasize the argumentation.

We don't need the entirety of a natural language transformed into a formal one - we're only interested in precise declarative sentences, used for argumentation, that can take on one of the two truth values, i.e. that can either be true or false, or at least, about whose truth value we can argue.

An important feature of declarative sentences is that they have a truth value - they can be either true or false, but the last two of these declarative sentences have unclear truth value. The third sentence (Goldbach's conjecture) seems straightforward because it can be either true or false, but so far nobody managed to prove which. It is not even clear whether this could be shown by some finite means, even if it were true.

The last sentence, traditionally called "the Liar", has kept many a logician awake for nights. It is related to an old paradox about a Cretan who said, "All Cretans are liars". The thing is that the truth value of this sentence quickly explodes: if it is true, then it is false, and if it is false, then it is true. Some philosophers think it's neither true nor false, while others believe it's both true and false.

Since ancient times, philosophers have come to accept the basic principles called bivalence and the principle of non-contradiction. **Bivalence** is the view that there are only two truth values, namely, true and false. **The principle of non-contradiction** states that the two truth values exclude each other - a claim cannot be both true and false at the same time. It seems that these principles are violated by "the Liar" claim.

> Principle of Bivalence: Each proposition must be either true or false, not both, not neither.

Due to these problems, we need a formal language without vagueness and ambiguity, in which the sentences are either true or false (and not kind of true, partially true, true from one perspective, etc.), and that is free of paradoxes.

Similarly to transforming a subset of a natural language, we can transform a subset of a programming language, opening up the possibility for **automatic program verification** based on the rules of logic. We need a calculus of reasoning which allows us to draw conclusions from given assumptions, like initialized variables, which are reliable in the sense that they preserve truth: if all our assumptions are true, then our conclusion ought to be true as well. A much more difficult question is whether, given any true property of a computer program, we can find an argument in our calculus that has this property as its conclusion.

To develop a formal symbolic logic, we transform a certain, sufficiently large, subset of all English declarative sentences into strings of symbols. This gives us a compressed, but still complete encoding of declarative sentences and allows us to concentrate on the mere mechanics of our argumentation. This is important since specifications of systems or software are sequences of such declarative sentences and it opens up the possibility of automatic manipulation of such specifications.

The best strategy is to start by finding the smallest sentences that cannot be decomposed further, called **atomic sentences**. Then, we can assign symbols to stand in for these atomic sentences, allowing us to compose compound sentences using the logical connectives.

> Atomic sentence is a sentence that has no parts that are themselves sentences.


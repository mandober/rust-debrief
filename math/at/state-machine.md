# Automata

## Stack machine
In some cases, a stack machine refers to a type of computer, while in other cases to a software scheme that simulates a stack machine. The main difference from other computers is that most of its instructions operate on a pushdown stack of numbers rather than numbers in registers. Most computers implement a stack in some form to pass parameters and link to subroutines - this does not make them stack machines. The common alternatives to stack machines are register machines, in which each instruction explicitly names specific registers for its operands and result.


## Finite-state machine
A finite-state machine (FSM) or finite-state automaton (FSA, plural: automata), finite automaton, or simply a state machine, is an abstract machine that can be in exactly one of a finite number of states at any given time. The FSM can change from one state to another in response to some external inputs; the change from one state to another is called a transition. An FSM is defined by a list of its states, its initial state, and the conditions for each transition.

Finite state machines can be subdivided into 
- transducers
- acceptors
- classifiers
- sequencers

Acceptors, also called recognizers and sequence detectors, produce boolean output, indicating whether or not the received input is accepted.

A classifier is a generalization of a finite state machine that, similar to an acceptor, produces a single output on termination but has more than two terminal states.

Transducers generate output based on a given input and/or a state using actions. They are used for control applications.
- Moore FSM uses only entry actions, i.e. output depends only on the state.
- Mealy FSM uses input actions, i.e. output depends on input and state.

Generators or sequencers are a subclass of the acceptor and transducer types that have a single-letter input alphabet. They produce only one sequence which can be seen as an output sequence of acceptor or transducer outputs.

A further distinction is between deterministic (DFA) and non-deterministic (NFA, GNFA) automata. In a deterministic automaton, every state has exactly one transition for each possible input. In a non-deterministic automaton, an input can lead to one, more than one, or no transition for a given state. The powerset construction algorithm can transform any nondeterministic automaton into a (usually more complex) deterministic automaton with identical functionality.


---
https://www.wikipedia.com/en/Finite-state_machine
https://www.wikipedia.com/en/State_pattern

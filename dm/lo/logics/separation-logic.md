# Separation logic

Separation logic is an extension of Hoare logic, a way of reasoning about programs.

It was developed by John C. Reynolds, Peter O'Hearn, Samin Ishtiaq and Hongseok Yang, drawing upon early work by Rod Burstall.

The assertion language of separation logic is a special case of the __logic of bunched implications (BI)__.

Separation logic facilitates reasoning about:
- programs that manipulate pointer data structures—including information hiding in the presence of pointers;
- **transfer of ownership** (avoidance of semantic frame axioms); and
- **virtual separation** (modular reasoning) between concurrent modules.

Separation logic supports the developing field of research described by Peter O'Hearn and others as local reasoning, whereby specifications and proofs of a program component mention only the portion of memory used by the component, and not the entire global state of the system. Applications include automated program verification (where an algorithm checks the validity of another algorithm) and automated parallelization of software.


---
https://en.wikipedia.org/wiki/Separation_logic

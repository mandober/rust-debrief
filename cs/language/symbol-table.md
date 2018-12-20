# Symbol table


A symbol table is a data structure used by a compiler where each identifier in a program's source code is associated with information relating to its declaration or appearance in the source.

A symbol table only exist in memory during the compilation phase, although it may be embedded in the output, e.g. in ABI object file for dubugging purposes.


A compiler may use one large symbol table for all symbols or use separated, hierarchical symbol tables for different scopes. The symbol table is accessed by most phases of a compiler, beginning with lexical analysis, and continuing through optimization. Lexical analyser spends a great proportion of its time looking up the symbol table, so the overall speed of the compiler is dependent on using an efficient data structure.

A commonly used data structure is a (key/value) hash map: the identifier (symbol) is hashed to produce a key (index), while the value is filled with any relavant information about that identifier (scope, type, kind, namespace, etc.).



https://www.wikiwand.com/en/Symbol_table
https://www.wikiwand.com/en/Debug_symbol


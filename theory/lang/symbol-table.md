# Symbol table

https://www.wikiwand.com/en/Symbol_table

https://www.wikiwand.com/en/Debug_symbol

A symbol table is data structure used by a language translator (compiler or interpreter) where each identifier (symbol) in a program's source code is associated with information relating to its declaration or appearance in the source.

A compiler may use one large symbol table for all symbols or use separated, hierarchical symbol tables for different scopes. The symbol table is accessed by most phases of a compiler, beginning with lexical analysis, and continuing through optimization. Lexical analyser spends a great proportion of its time looking up the symbol table, so the overall speed of the compiler is dependent on using an efficient data structure.

A commonly used data structure is a (key/value) hash map: the identifier (symbol) is hashed to produce a key (index), while the value is filled with any relavant information about that identifier (scope, type, kind, namespace, etc.).

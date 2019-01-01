# SQL

Structured Query Language is a DSL designed for data management through a relational database management system (RDBMS) with management of database schemas and management of database objects' permissions.

SQL language is sub-divided into several language elements:
- Statements, which may have a persistent effect on schemas and data, or which may control transactions, program flow, connections, sessions, or diagnostics.
- Queries, which retrieve data based on specific criteria.
- Expressions, which can produce either scalar values or tables consisting of columns and rows of data.
- Predicates, which specify conditions that can be evaluated in a three-value logic. Boolean truth values and are commonly used to limit the effects of statements and queries, or to change program flow.
- Clauses, which are, someyimes optional, constituent components of statements and queries.

The *SQL:2003* standard makes minor modifications, compared to *SQL:1999*, officially introducing sevaral new features:
- XML-related features
- Window functions
- Sequence generator, which allows standardized sequences
- New column types: auto-generated values and identity-columns
- New `MERGE` statement
- Extensions of `CREATE TABLE`: `CREATE TABLE AS` and `CREATE TABLE LIKE`
- Deprecation of `bit` and `bit varying` types

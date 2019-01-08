# PL/SQL

PL/SQL is Oracle's procedural language extension to SQL.
proprietary, server-based,

Some other SQL database management systems offer languages similar to PL/SQL.

Its syntax strongly resembles that of Ada.

PL/SQL supports variables, conditions, arrays, and exceptions. Implementations from version 8 of the Oracle RDBMS onwards have included features associated with object-orientation.

The underlying SQL functions as a declarative language. Standard SQL—unlike some functional programming languages—does not require implementations to convert tail calls to jumps. SQL does not readily provide "first row" and "rest of table" accessors, and it cannot easily perform some constructs such as loops. PL/SQL, however, as a Turing-complete procedural language which fills in these gaps, allows Oracle database developers to interface with the underlying relational database in an imperative manner. SQL statements can make explicit in-line calls to PL/SQL functions, or can cause PL/SQL triggers to fire upon pre-defined Data Manipulation Language (DML) events.


PL/SQL stored procedures (functions, procedures, packages, and triggers) which perform DML get compiled into an Oracle database: to this extent their SQL code can undergo syntax-checking. Programmers working in an Oracle database environment can construct PL/SQL blocks of such functionality to serve as procedures, functions; or they can write in-line segments of PL/SQL within SQL*Plus scripts.

While programmers can readily incorporate SQL DML statements into PL/SQL (as cursor definitions, for example, or using the SELECT ... INTO syntax), Data Definition Language (DDL) statements such as CREATE TABLE/DROP INDEX etc require the use of "Dynamic SQL". Earlier versions of Oracle required the use of a complex built-in DBMS_SQL package for Dynamic SQL where the system needed to explicitly parse and execute an SQL statement. Later versions have included an EXECUTE IMMEDIATE syntax called "Native Dynamic SQL" which considerably simplifies matters. Any use of DDL in Oracle will result in an implicit commit. Programmers can also use Dynamic SQL to execute DML where they do not know the exact content of the statement in advance.

PL/SQL offers several pre-defined packages for specific purposes. Such PL/SQL packages include:

DBMS_OUTPUT - for output operations to non-database destinations
DBMS_JOB - for running specific procedures/functions at a particular time (i.e. scheduling)
DBMS_XPLAN - for formatting "Explain Plan" output
DBMS_SESSION - provides access to SQL ALTER SESSION and SET ROLE statements, and other session information.
DBMS_METADATA - for extracting meta data from the data dictionary (such as DDL statements)
UTL_FILE - for reading and writing files on disk
UTL_HTTP - for making requests to web servers from the database
UTL_SMTP - for sending mail from the database (via an SMTP server)
Oracle Corporation customarily adds more packages and/or extends package functionality with each successive release of the Oracle DBMS.

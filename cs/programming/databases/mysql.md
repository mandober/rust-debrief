# MySQL

http://dev.mysql.com

MySQL is a multithreaded, multi-user SQL database management system (DBMS). It was created by MySQL AB. MySQL is written in ANSI C and C++; the SQL parser uses yacc. All major PL have support for accessing MySQL database; PLs supporting an ODBC interface can use `MyODBC` as a communication relay to access a MYSQL db.

MySQL features multiple storage engines, making the engine choice flexible and applicable at the table level resolution. In MySQL 5.0 storage engines had to be compiled in, but in MySQL 5.1 storage engines can be dynamically loaded (i.e. loaded at run time).

Storage Engines
- Native storage engines
  - MyISAM
  - Falcon
  - Merge
  - Memory (heap)
  - Federated
  - Archive
  - CSV
  - Blackhole
  - Cluster
- Partner-developed storage engines
  - InnoDB
  - solidDB
  - NitroEDB
  - BrightHouse
- Community-developed storage engines
- Custom storage engines

MySQL also support **commit grouping**, whih is gathering of multiple transactions from multiple connections together to increase the number of commits per second.

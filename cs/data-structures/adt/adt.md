# Abstract Data Types

https://www.wikiwand.com/en/Abstract_data_type
https://www.geeksforgeeks.org/abstract-data-types/


Abstract Data Type (ADT) is a type whose bahavior is defined by a minimum set of specific methods that it must support, but the implementation details of those methods and type's internal structure are left unspecified, allowing for their optimal designing apprach. Sometimes, ADT also prescribes complexity level for each required operation.

ADT is like an interface that a type implements.

For example, a stack ADT is a collection supporting the `push` and `pop` operations for adding and removing elements, respectively. It may also prescribe the temporal complexity requirement, e.g., that both operations should complete in O(1) time.

When implementing an ADT, one is free to select the most suitable data structure (array, lnked list, etc.) and come up with whatever design desired, as long as the prescribed requirements hold.

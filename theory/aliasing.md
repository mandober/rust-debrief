# Aliasing

A variable is a memory location paired with an associated symbolic name that contains some quantity of information referred to as a value. Using that symbolic name (the name of a variable) is the usual way to reference the stored value.

Aliasing describes a situation in which a memory location can be accessed through different symbolic names. Modifying the value through one name implicitly modifies the values associated with all aliased names, which may not be expected by the programmer. Furthermore, aliasing makes it particularly difficult to understand, analyze and optimize programs.


## Example in C

```c
int name = 5;
int *ptr_name = &name;

void foo(int &param1, int &param2) {}

foo(name, name);
```

If the value of `*ptr_name` is changed, the value referenced by `name` also changes because `*ptr_name` aliases it; it becomes another symbolic name for the same value. Also, both parameters of function `foo` alias i.e. they refer to the same value (the same memory location).




---
https://www.wikiwand.com/en/Aliasing_(computing)
https://www.wikiwand.com/en/Pointer_aliasing
http://www.drdobbs.com/cpp/type-based-alias-analysis/184404273?_requestid=510121
http://dbp-consulting.com/tutorials/StrictAliasing.html


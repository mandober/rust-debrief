# Builder pattern


## Prologue
In OO paradigm, objects are first described, by defining a class that encompasses the characteristics of the object (as properties) and the related actions (as methods), and then used (instantiated), while in Rust this concept can be realized using the struct to define object's properties and the implementation block to define its methods. The structs use fields to group pieces of information together and all the fields need to have a value set every time the object (struct) is instantiated.

Whether the reluctance to accommodate this strict struct's demand is stemming from the inconvenience, redundancy or the untimely information requisition, 
the builder pattern curbs this predicament by allowing postponed and gradual procurement of values.

## Strategy
We are obligated to provide all the fields with a value when instantiating a struct, but, depending on the situation, we can use a sensible default value that may not even need to be modified further. We can also use null as a placeholder for the value, or, when there's no null, there's the Option.

The builder pattern can be implemented only for methods because it relies on the method chaining; it requires `self` as both, input and output parameter.

- method chaining
- nice API
- scales well

- only applicable to methods
- a trifle bit verbose implementation


Some data structures are complicated to construct, due to:
- large number of inputs
- compound input types
- optional configuration data
- different choices for input value

This can easily lead to many distinct constructor functions, each taking a lot of arguments.

If T is such a data structure, consider introducing a T builder:
- Introduce a separate data type TBuilder for incremental configuration
- The builder constructor should take as parameters only the data required to to make a T.
- The builder should offer a suite of convenient methods for configuration, including setting up compound inputs (like slices) incrementally. These methods should return self to allow chaining.
- The builder should provide one or more "terminal" methods for actually building a T.
- The builder pattern is especially appropriate when building a T involves side effects, such as spawning a thread or launching a process.

In Rust, there are two variants of the builder pattern, differing in the treatment of ownership: consuming and non-consuming builders.


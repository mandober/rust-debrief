## Analysis of Algorithms

https://www.wikiwand.com/en/Analysis_of_algorithms
https://www.wikiwand.com/en/Big_O_notation
https://www.wikiwand.com/en/Computational_complexity


Algorithm analysis estimates the time and space computational complexity of algorithms, classifing them according to their efficiency (grow rate).

It does so by relating an algorithm's input size to its growth rate in terms of time (time complexity) or memory consumption (space complexity).

In order to estimate growth rate, as the input size gets very large, it is convenient to present an algoritham as a polynomial function and then determine which of its terms contributes the most to its growth rate.

Since we desire an absolute estimatation we can ignore constants, and, in the case of sum function, we can consider only the most critical term; in case of product function, we should take alike terms in consideration.

For example, in the function $$f = 4n^2 + 64n + 128$$, the term that contributes the most to the growth rate of this function, as the size of $$n$$ increases (approaching infinity) is the first one, $$4n^2$$. We can also disregard the constant, $$4$$, and say that the growth rate of this function is $$n^2$$ (i.e. quadratic) when the input gets significantly large.

The other terms, individually or combined, may initially exert the greater influence on the function's growth, when the input size is still relatively small, but there will always be a point when the most significant term catches up and then quickly surpasses all the other influencing factors.


---

An algorithm is said to be efficient when this function's values are small, or grow slowly compared to a growth in the size of the input.

Different inputs of the same size may cause the algorithm to have different behavior, so best, worst and average case descriptions might all be of practical interest.

When not otherwise specified, the function describing the performance of an algorithm is usually an upper bound, determined from the worst case inputs to the algorithm.



## Asymptotic Notation
Asymptotic notation is a way to classify efficiency of algorithams as the input size increases.

It represents algorithm’s growth rate, independent of capabilities of a computer system the algorithm is running on.

Instead of comparing algorithms by counting the number of primitive operations or by measuring the time, asymptotic notation tries to estimate the absolute space and time requirements of different approaches.

It relates input size to the algorithms' (function's) grow rate, ignoring constant expressions in the polynomial because they don't exhibit significant influence pass the certain point, i.e. after the size of input gets sufficienly large.

For example, in the polynomial, $$f = n^2 + 2n + n$$, the term that has the prevailing influence on the function's grow rate, as $$n$$ increases, is $$n^2$$.


For example, considering the task of searching through an array in order to find a specific element that is actually absent from the array. The simplest algorithm would, starting with the first element, go through the whole array one element at the time. The efficiency of such algorithm would depend only on the number of elements in the array.





a function $$f(n) = 4n$$
$$f = n^2 - 2n + n$$


to calculate the doubled square of natural numbers might look like `f = n -> 2n^2`


## Big O notation
In computer science, big O notation is used to classify algorithms according to how their running time or space requirements grow as the input size grows.

Big O notation characterizes functions according to their growth rates.




## Time and space complexity

Time and space complexity of various data structures in big O (asymptotic) notation.

Worst case
DS        | Space   | Search   | Insert   | Delete
----------|---------|----------|----------|---------
BST       | O(n)    | O(n)     | O(n)     | O(n)

Average case
DS        | Space   | Search   | Insert   | Delete
----------|---------|----------|----------|---------
BST       | O(n)    | O(log n) | O(log n) | O(log n)

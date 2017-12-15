fn new() -> String

Creates a new empty String.

Given that the String is empty, this will not allocate any initial buffer. While that means that this initial operation is very inexpensive, but may cause excessive allocation later, when you add data. If you have an idea of how much data the String will hold, consider the with_capacity method to prevent excessive re-allocation.
Examples

Basic usage:

let s = String::new();
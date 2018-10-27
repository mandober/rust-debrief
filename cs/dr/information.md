# Information


Information is any entity or form that provides the answer to a question of some kind or resolves uncertainty. It is thus related to data and knowledge, as data represents values attributed to parameters, and knowledge signifies understanding of real things or abstract concepts.

A core precept of modern physics is that, in principle, information cannot be destroyed. It is also held that the existance of information does not depend on the consumer (for example, it exists beyond an event horizon of a black hole), while in the case of knowledge, the information requires a cognitive observer.


## Units of Information

A **unit of information** is the capacity of some standard data storage system or communication channel, used to measure the capacities of other systems and channels. In information theory, units of information are also used to measure the entropy of random variables and information contained in messages.

The funamental unit of data storage capacity (information) is the **bit**, the capacity of a system that has only 2 states. The name "bit" is portmanteau of "Binary digIT", they are not the same: a bit is the maximum amount of information that can be conveyed by a binary digit.

A **binary digit** is a number that can take one out of two possible values, either 0 or 1. By analogy, a binary digit is like a container, whereas information is the amount of matter in the container.

One bit is typically defined as the information entropy of a random binary variable that is 0 or 1 with equal probability, or the information that is gained when the value of such variable is consumed (i.e. when it becomes known).


> The most significant digit in a byte is bit#7 and the least significant digit is bit#0, otherwise known as **msb** and **lsb** respectively in lowercase; lsb is always bit#0, msb varies. **MSB** (uppercase) is the most significant byte.


## Byte
Today, a **byte** almost always means 8 bits, an octet. A byte can represent 256 distinct variants (a bit has 2 variants, 8 bits have 2^8 variants).

The byte is the smallest individual piece of data that can be accessed (or modified); it is the most important data structure used by CPUs today. Many computers use the byte as a minimum addressable piece of data.

## Nibble
Half of byte, 4 bits, is called a **nibble**, it can represent 16 distinct values (2^4). One nibble corresponds to one hexadecimal digit - for example,
`1011 1100` is a sequence of 8 bits, where the former nibble is 0xb in hex (11 in decimal), while the latter is 0xc hex (12 decimal), so the value of this byte (interpreted as unsigned integer) is 0xbc in hex or 188 in decimal.


## Word, block, and page
Computers usually manipulate bits in groups of a fixed size, conventionally called **words**. The number of bits in a word is defined by the size of the registers in CPU or by the number of data bits that can be fetched from the main memory in a single operation. Some machine instructions and computer number formats use two words (double word, dword), or four words (quad word, quad). Computer memory caches usually operate on blocks of memory that consist of several consecutive words. These units are customarily called **cache blocks**, or, in CPU caches, **cache lines**. Virtual memory systems partition the computer's main storage into even larger units, traditionally called **pages**.


https://www.wikiwand.com/en/Bit
https://www.wikiwand.com/en/Word_(computer_architecture)
https://en.wikibooks.org/wiki/C%2B%2B_Programming/Programming_Languages/C%2B%2B/Code/Statements/Variables



# Units of Information

**Unit of information** is the capacity of a data storage system (or communication channel), used to measure the capacities of other systems.

**Bit**, the funamental unit of information, is the capacity of a system that has only 2 states. **Binary digits** are concrete symbols (i.e. 1 and 0) that can be placed in a bit. The name "bit" is portmanteau of "binary digit", but they are not the same: a bit is the maximum amount of information that can be conveyed by a binary digit.

One bit is typically defined as the information entropy of a random binary variable that is 0 or 1 with equal probability, or the information that is gained when the value of such variable is consumed (i.e. when the value becomes known).

**Bit pattern** is some (random) sequence of bits.


**Number** is an abstract primitive (undefined) mathermatical concept, that can be represented using different number systems.


## Numeral system
- Numeral system is used to denote a particular number.
- Positional system has positions filled from right to left.
- Right-most digit is the most significant digit, `MSD ↔ LSD`
- System: binary, octal, decimal, hexadecimal, base32, base64, etc.
- Numeral system can be defined by its base (also radix)
- Base indicates the number of symbols of a system.    
  e.g. base16 (hex) has 16 different digits, 0-F.
- Radix is used as a subscript to disambiguate numbers, e.g. $$100_2$$

To disambiguate:
- radix is used as a subscript, e.g. $$100_2$$
- suffix letter, $$1010H$$, $$1010h$$


- the base to which the digit's position index (zero-based) is raised to get the decimal value at that position: 
  A digit's value is the digit multiplied by the value of its place.


In general, in base, $$b$$, there are $$b$$ digits and the number

$$
\displaystyle
a_3\ a_2\ a_1\ a_0\ =
\ (a_3\times b^{3}) + 
(a_{2}\times b^{2}) + 
(a_{1}\times b^{1}) + 
(a_{0}\times b^{0})
$$

(Note that a_3 a_2 a_1 a_0 represents a sequence of digits, not multiplication)



## Binary number system
- positinal, base 2, number system
- with only two symbols

A Binary number is a weighted number who’s weighted value increases from right to left
The weight of a binary digit doubles from right to left
A decimal number can be converted to a binary number by using the sum-of-weights method or the repeated division-by-2 method



**Byte** is an ordered sequence of 8 bits (an octet). A byte can represent 256 distinct variants (a bit has 2 variants, 8 bits have 2^8 variants). The byte is the smallest individual piece of data that can be accessed. Most computers use byte as a minimum addressable unit of data. Memory addresses are 1 byte apart - a memory address can store 1 byte of data.

Storing larger data, e.g. a 32-bit number, `int`, requires 4 bytes: the lowest memory address will be used as the address of that integer, although the bytes comprising it will spread accross 4 memory addresses; the lowest address, e.g. 0xF000 will hold byte 1, the next address, 0xF001, will hold the byte number 2, the next addess, 0xF002, byte 3 and finally the address, 0xF003, will hold the byte 4 of theat intteger.




The most significant digit in a byte is bit#7 and the least significant digit is bit#0, otherwise known as msb and lsb respectively in lowercase; lsb is always bit#0, msb varies. MSB (uppercase) is the most significant byte.



## Nibble
Half of byte, 4 bits, is called a **nibble**, it can represent 16 distinct values (2^4). One nibble corresponds to one hexadecimal digit - for example,
`1011 1100` is a sequence of 8 bits, where the former nibble is 0xb in hex (11 in decimal), while the latter is 0xc hex (12 decimal), so the value of this byte (interpreted as unsigned integer) is 0xbc in hex or 188 in decimal.


## Word, block, and page
Computers usually manipulate bits in groups of a fixed size, conventionally called **words**. The number of bits in a word is defined by the size of the registers in CPU or by the number of data bits that can be fetched from the main memory in a single operation. Some machine instructions and computer number formats use two words (double word, dword), or four words (quad word, quad). Computer memory caches usually operate on blocks of memory that consist of several consecutive words. These units are customarily called **cache blocks**, or, in CPU caches, **cache lines**. Virtual memory systems partition the computer's main storage into even larger units, traditionally called **pages**.


https://www.wikiwand.com/en/Bit
https://www.wikiwand.com/en/Word_(computer_architecture)
https://en.wikibooks.org/wiki/C%2B%2B_Programming/Programming_Languages/C%2B%2B/Code/Statements/Variables



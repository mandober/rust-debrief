# Array in Memory

http://www.wikipedia.com/en/Array_data_structure

Two pieces of information are needed to retrieve the value of a variable:
- type of the variable
- memory address of its first byte

The type identifies the size in bytes needed to store a value of that type. Knowing the size of a variable in bytes and the memory locations of its initial byte, the value can be correctly interpreted.

There are several factors influencing the mechanism of manipulating the data (value): the size of the particular type, address resolution unit and word size. Assuming the address resolution unit is 1 byte and the size of word is 4 bytes, we're left with the size of the type as the only variable information:


An array data structure is a collection of elements (values or variables), each identified by an index. It is stored in memory so that the position of each element can be computed from its index.
For example, an array of 10 32-bit integer variables, with indices 0 through 9, may be stored as 10 words at memory addresses 2000, 2004, 2008, ... 2036, so that the element with index i has the address 2000 + 4 × i.[4]
The memory address of the first element of an array is called the base address.




When an array of a given size, say N, and of a given type is declared, the compiler allocates enough memory to hold all N pieces of data. If M bytes of memory is required for each piece of data of the type specified, then a total of N*M bytes of contiguous memory are allocated to that array. 

The data for the first element is stored in the first M bytes, the data for the second element is stored in the next M bytes, etc. 

The compiler remembers the address of the first byte of an array only. In fact the address of the first byte is considered as the memory address for the entire array. 

Knowing the address of the first byte the computer can easily calculate to find the memory address for any other element of the array. 

For example given an array A, to retrieve the data stored in A[n], the computer goes to the address of the array plus n*M and retrieves the next M bytes of data. [This explains why it is better to start the subscript at 0 rather than at 1.] 

The rest of this section is devoted to discussing the most subtle mistake in dealing with arrays. This mistake results from accessing an element of an array using an array index that is out of range.

To illustrate, consider an integer array exam_score that is declared for use to store the exam score for a class of 12 students.:

const int CLASS_SIZE = 12;
int exam_score[CLASS_SIZE];

Suppose the 12 scores have been assigned to the array, and the following code is used to compute the class average:
float average;
int sum = 0, n;
for (n = 1; n <= CLASS_SIZE; n++)
    sum += exam_score[n];
average = double(sum)/double(CLASS_SIZE);

This code when embedded in a suitable program will compile and run on most systems. However the result for the average is almost always wrong. The reason is that the element exam_score[12] is referenced in the last iteration of the loop. The 12 elements of the array are indexed from 0 to 11, and the index of 12 is out-of-range. 

The problem is that for the sake of speed and efficiency most C++ compilers do not check for out-of-range indexes. When asked to retrieve the value stored in exam_score[12], the computer simply compute based on the address of the first byte of the array the memory location of where the thirteenth element is supposed to be and retrieve the value located at that address in memory. The value stored at that location is totally unpredictable and so the sum computed is also unpredictable. 

Even more potentially problematic is assigning a value to an out-of-range array variable. Most computer will not issue a warning or an error message and the problem can go unnoticed.

As an example, suppose your program has the following assignment statement:

exam_score[n] = 64;

and index n happens to be out-of-range. The computer calculates the address of that element based on the address of the first byte of the array and store the value of 64 there. That memory location may belong to some other variables and many unexpected results may occur. 







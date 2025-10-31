# Memory Areas in a C++ program
_______________________________________________________________________________
data section: global and static variables

stack: primitive variables (size of these variables is known at compile time)
Automatic memory allocation

heap: This is for (size of the variable can't be determind at compile time, 
only during runtime). Dynamic memory allocation. 

The programmer is responsible for managing this.
_______________________________________________________________________________

All memory is taken from the process address space.
_______________________________________________________________________________
## Functions that are used to allocate memory on the Heap

malloc: raw memory on the heap. 
It accepts a size and then returns a pointer to the the allocated memory.

Note that malloc does not initialize the memory, it returns the raw memory.

calloc: allocates memory on the heap and intializes the memory to zero.

realloc: allocates a larger chunk of memory for an existing allocation.

free: delocates memory. Basically it tells the OS that the memory address
is available again.

_______________________________________________________________________________

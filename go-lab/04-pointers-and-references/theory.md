A pointer is a variable that stores the memory address of where a value 
is stored in memory.

What is the purpose of pointers?

To increase the performance of a program by avoiding 
uncessary `pass by value` situations. 

By default, functions in Go will accept arguments using `pass by value`.

Passing a pointer to a function, allows you to directly modify the value
of the variable.

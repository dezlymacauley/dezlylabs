/*

ABOUT: Pass by value

value_b will get a copy of the value of value_a

TypeScript is a high level language and so it does not have a way to view
the memory address of a variable.

But value_a and value_b are stored at different locations in memory.

To be specific, primitive variable types are stored on the stack.

Primitive types in TypeScript are:
  - number
  - string
  - boolean
  - null
  - undefined
  - symbol
  - bigint

*/

let value_a: number = 5;
console.log(`value_a is ${value_a}`);
// value_a is 5

let value_b: number = value_a;
console.log(`value_b is ${value_b}`);
// value_b is 5


//_____________________________________________________________________________

// If you update `value_b`, the original `value_a` will not be affected.

value_b = 10;
console.log(`value_b updated from 5 to ${value_b}`);
// value_b updated from 5 to 10

console.log(`value_a is still ${value_a}`);
// value_a is still 5

//_____________________________________________________________________________

// If you update `value_a`, `value_b` will not be affected

value_a = 20;
console.log(`value_a updated from 5 to ${value_a}`);
// value_b updated from 5 to 20

console.log(`value_b is still ${value_b}`);
// value_a is still 10

//_____________________________________________________________________________

/*

ABOUT: Pass by reference

This is what happens when you are working with types that are not primitive.

*/

const objectA: { value: number } = { value: 5 };
console.log(`objectA.value is ${objectA.value}`)
// objectA.value is 5

const objectB: {value: number} = objectA;
console.log(`objectB.value is ${objectB.value}`)
// objectB.value is 5

//_____________________________________________________________________________

// If you update `value_b`, the original `value_a` will be updated

objectB.value = 10;
console.log(`objectB.value updated from 5 to ${objectB.value}`);
// objectB.value updated from 5 to 10

console.log(`objectA.value is still ${objectA.value}`);
// objectA.value is now 10

//_____________________________________________________________________________

// If you update `objectA`, `objectB` will be updated.

objectA.value = 20;
console.log(`objectA.value updated from 5 to ${objectA.value}`);
// objectA.value  updated from 5 to 20

console.log(`objectB.value is still ${objectB.value}`);
// objectB.value is now 20

//_____________________________________________________________________________

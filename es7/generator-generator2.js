"use strict";
let delegatedIterator = (function* () {
   yield 'Hello!';
   yield 'Bye!'; 
}())

let delegatingIterator = (function* () {
    yield 'Greetings';
    yield* delegatedIterator;
    yield 'Ok, bye.';
}());

for(let value of delegatingIterator) {
    console.log(value);
}

function* gen() {
    yield* ["a", "b", "c"];
}

console.log(gen().next())

let read = (function* () {
   yield 'hello';
   yield* 'hello'; 
}())

console.log(read.next().value);
console.log(read.next().value);
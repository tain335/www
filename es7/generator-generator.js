"use strict";
function* foo() {
    yield 'a';
    yield 'b';
}

function* bar() {
    yield 'x';
    //foo();
    yield* foo();
    yield 'y';
}

for(let v of bar()) {
    console.log(v);
}
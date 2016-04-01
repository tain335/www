"use strict";
function* numbers() {
    yield 1;
    try {
        yield 2;
        yield 3;
    } catch (error) {
        yield 4;
        yield 5;
    }
    yield 6;
}

var g = numbers();
console.log(g.next());
console.log(g.next());
//v8引擎 还没有实现return
console.log(g.return(7));
console.log(g.next());
console.log(g.next());
# My mini compiler

## Intro

A mini compiler, written in rust. Still deciding what to compile to.
Currently C...

## Features

- [x] Lexing
- [x] Parsing
- [x] Emiting

## Credits

- https://austinhenley.com/blog/teenytinycompiler1.html
- https://www.youtube.com/watch?v=GAU51Dqsp3Y&t=328s

## Example

```C
print "How many fibonaccis nrs to print\n";
input nums;

let a = 0;
let b = 1;
while nums > 0
{
    print a;
    let c = a + b;
    a = b;
    b = c;
    nums = nums - 1;
}

if a != b
{
    print "Extra crap\n";
    print -a * b + b /-5;
}
```

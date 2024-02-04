# My mini compiler

## Intro

A mini compiler, written in rust. Compiles a mini c-like scripting language to actual C code. 
Just for fun...

## Features

- [x] Lexing
- [x] Parsing
- [x] Emiting

- [x] Better CLI
~~- [ ] New Target -> .NET IL~~
- [ ] Work on performance

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



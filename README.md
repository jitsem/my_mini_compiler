# My mini compiler

## Intro

A mini compiler, written in rust. Still deciding what to compile to.
Only a lexer at the moment...

## Features

- [x] Lexing
- [ ] Parsing
- [ ] Emiting

## Credits

- https://austinhenley.com/blog/teenytinycompiler1.html
- https://www.youtube.com/watch?v=GAU51Dqsp3Y&t=328s

## Example

```
print "How many fibonacci numbers do you want?";
input nums;

let a = 0;
let b = 1;
while nums > 0
{
    print a;
    let c = a + b;
    let a = b;
    let b = c;
    let nums = nums - 1;
}

if a == b
{
    print "Equals";
}
else
{
    print a;
    print b;
}
```

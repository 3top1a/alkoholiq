# Alkoholiq
Alcoholiq, a programming language that transpiles to Brainfuck


## Syntax

The only valid file extension is .🍺 (U+1F37A)
```js
// Comment
/* 
 Multiline comment
 */

// Variables
// There are no types, only u8
// Values over 255 are illegal
named_var = 5

// In this case named_char's value is 99
// Single quotes are used for characters, double quotes for strings
// Non ascii characters are not supported for characters
named_char = 'c'

// false is 0, true is anything not 0
bool = true

// Fixed size arrays
// Arrays are denoted by the * symbol as they work like pointers
array* = [1, 2, 3, 4, 5]
// Strings are arrays
string* = "Hello, World!"
beer* = "🍺"

// Operators
// Math with priority
math = 5 + 5 * 2 - 2 / 2
```

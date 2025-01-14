# Alkoholiq

Alcoholiq, a programming language that transpiles to Brainfuck

The ultimate goal for this semi-esoteric language is to be able to write itself in a readable way, with sytnax similar
to rust and operation similar to C.

## Syntax

The only valid file extension is .🍺 (U+1F37A)
Not all features of this section's example are implemented.

```js
// Comment
/* 
 Multiline comment
 */

// Variables
// There are no types, only u8
// Values over 255 wrap around
named_var = 5

// In this case named_char's value is 99
// Single quotes are used for characters, double quotes for strings
// Non ascii characters are not supported for characters
named_char = 'c'

// false is 0, true is 1
// In comparison, 0 is false, everything else is true
bool = true

// Fixed size arrays
// Arrays are denoted by the * symbol as they work like pointers
array = [ 1 2 3 4 { + ( 3 2 ) } ]
// An array can also be created this way, substituting memset
array_empty = [ 0 ; 3 ]
// Strings are arrays
string = "Hello, World!"
beer = "🍺"

// Operators
// Math with priority
two = 2
// I'm too lazy to make actual math
math = +( 5 *(5 two))

// Functions
input_array = [0;16]
// Get user input
// Maximum length is 16
input(input_array, 16)

// Print
print("Hello ")
print(input_array)
print("!\n")

// Raw Brainfuck
// This should only be used in the standard library, e.g. input(), not in user code
// If this project succeeds it's ultimate goal, basm will be the only function from the compiler except math,
// and all other functions will be written in alkoholiq
basm("<>+-")
```

## The name?

The name stems from alcoholism, because, well, I'm from the Czech Republic where beer is cheaper than water.
*Alkoholik* is a short Czech translation of an alcohol addict.
Inspired by our automotive manufacturer, *Škoda*, that have started naming all of their new cars with the letter q at the end, I decided to do the same.
Tune in next time for *QDE Qonnect*.


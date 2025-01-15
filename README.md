# Alkoholiq

Alkoholiq, a programming language that transpiles to Brainfuck.
The output is not executable immediately, and needs to be piped into a tool such as [my 540 byte brainfuck interpreter](https://github.com/3top1a/sbfi-rs).

The ultimate goal for this semi-esoteric language is to be able to write itself in a readable way, with syntax similar
to rust and operation similar to C.

## Syntax

The only valid file extension is .🍺 (U+1F37A).

Not all features of this section's example are implemented.
Syntax may also change, I might possibly maybe try to think about doing math properly someday for three seconds.

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
// Expressions can be used inside anything
array = [ 1 2 3 4 { + ( 3 2 ) } ]
// An array can also be created this way, substituting memset
array_empty = [ 0 ; 3 ]
// Strings are arrays
string = "Hello, World!"
// This string is four characters long!
beer = "🍺"

// Operators
two = 2
// I'm too lazy to make actual math so enjoy reversed reverse polish notation
math = +( 5 *(5 two))

// Functions
input_array = [0;16]
// Get user input
// Maximum length is 16
input(input_array 16)

// Print
print("Hello ")
print(input_array)
print("!\n")
// Printf is used for printing strings and numbers in decimal
// This prints `c`
print(c)
// This prints `99`
printf(c)

// Raw Brainfuck
// This should only be used in the standard library, e.g. input(), not in user code
// If this project succeeds it's ultimate goal, basm will be the only function from
// the compiler except math, and all other functions will be written in alkoholiq
basm("<>+-")

// Iterators work like in Rust
for ch in input_array {
	// Indents are 8 wide tabs
	printf(ch);
    // \n prints out a new line, a single \ does not need to be escaped
	print('\n');
}

// Example foobar implementation
hit = false
for i in 0..254 {
	if %(i 5) == 0 {
		hit = true
		print("Foo")
	}
	if %(i 7) == 0 {
		hit = true
		print("Bar")
	}
	if hit == true {
		print("\n")
	} else {
		printf(i)
		print('\n')
	}
}

```

## The name?

The name stems from alcoholism, because, well, I'm from the Czech Republic where beer is cheaper than water.
*Alkoholik* is a short Czech translation of an alcohol addict.
Inspired by our automotive manufacturer, *Škoda*, that have started naming all of their new cars with the letter q at the end, I decided to do the same.
Tune in next time for *QDE Qonnect*.

## Q&A

I will not be taking any questions, thank you.

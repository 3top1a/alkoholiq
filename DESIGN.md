# Alkoholiq design document

Let's start from the bottom and work up to a usable language
As the complexity grows n^2 where n is the number of features, each pass should be the simplest it can be, relying on
the
power of abstraction.

## Lower Intermediate Representation (LIR)

Due to the design of brainfuck, several limitations are imposed on the design of further passes.
For example, the absence of jump instructions means function calls must be implemented as a recursive match case
statement.

The memory layout looks like this:

```
[ Temporary variables used by instructions] [ 0 ] [+Variable storage]
```

Simply using a variable name will automatically reserve space for it. Some instructions need to have the variable be
used beforehand.

### Instructions

- `set <var>, <value>` - Set a variable to a value
- `copy <var>, <var>` - Copy from variable a to variable b

- `inc <var>` - Increment a variable by one
- `dec <var>` - Decrement a variable by one
- `inc_by <var>, <value>` - Increment a variable by a value
- `dec_by <var>, <value>` - Decrement a variable by a value

- `add <var>, <var>` - Add two variables together
- `sub <var>, <var>` - Subtract two variables
- `compare <var>, <var>, <var>` - Compare two variables, store the result in a third variable

- `read <var>` - Read one byte from stdin and store it in a variable
- `print <var>` - Print a variable to stdout
- `print_msg <string>` - Print a string to stdout

- `if_eq <var>, <var>` - If two variables are equal, run the next block
- `if_neq <var>, <var>` - If two variables are not equal, run the next block
- `if_eq <var>, <const>` - If a variable is equal to a constant, run the next block
- `until_eq <var>, <var>` - Run until a variables are equal
- `while_nz <var>` - Run while a variable is not zero
- `end` - End the current block

- `raw` - Insert raw brainfuck code


### Examples

A simple `cat` program:

```js
read data // Read from stdin into a variable called data
while_nz data // While data is not zero
print data // Print data
read data // Read again, if the buffer is empty, it will return zero and exit the loop
end
```

A program that takes in two characters and compares their ASCII values:

```js
read a
read b

compare a, b, res

if_eq_const res, 0
    print_msg "Numbers are equal"
end

if_eq_const res, 1
    print_msg "Right number is greater"
end

if_eq_const res, 2
    print_msg "Left number is greater"
end
```


Here is fibonacci's sequence:

```js
// Initialize first two numbers of the sequence
set a, 1  // F(1) = 1
set b, 1  // F(2) = 1
set count, 10  // How many numbers to generate

// Print first two numbers
print a
print_msg " "
print b
print_msg " "

// Decrement count by 2 since we've printed two numbers
dec_by count, 2

// Generate remaining numbers
while_nz count
    // temp = a + b
    copy a, temp
    add temp, b

    // Print the new number
    print temp
    print_msg " "

    // Shift numbers: a = b, b = temp
    copy b, a
    copy temp, b

    // Decrement counter
    dec count
end

```

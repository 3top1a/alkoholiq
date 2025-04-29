# Alkoholiq design document

Let's start from the bottom and work up to a usable language
As the complexity grows n^2 where n is the number of features, each pass should be the simplest it can be, relying on
the
power of abstraction.

## Lower Intermediate Representation (LIR)

Due to the design of brainfuck, several limitations are imposed on the design of further passes.
For example, the absence of jump instructions means function calls must be implemented as a series of if statements and a global "function pointer".

The memory layout looks like this:

```
[ Temporary variables used by instructions] [ 0 ] [+Variable storage]
```

Temporary variables are used by instructions to store intermediate results, and are not accessible to the user.
They are also stored in memory below zero, if your interpreter/compiler complains about pointer underflow, chuck ">>>>>>>>>" in front of the code.

Simply using a variable name will automatically reserve space for it. Some instructions need to have the variable be
used beforehand.

### Instructions

Also see the [lir.rs](https://github.com/3top1a/alkoholiq/blob/main/src/lir/lir.rs) file.

- `set <var> <value>` - Set a variable to a value
- `copy <var> <var>` - Copy from variable a to variable b

- `inc <var>` - Increment a variable by one
- `dec <var>` - Decrement a variable by one
- `inc_by <var> <value>` - Increment a variable by a value
- `dec_by <var> <value>` - Decrement a variable by a value

- `add <var> <var>` - Add two variables together
- `sub <var> <var>` - Subtract two variables
- `mul <var> <var>` - Multiply two variables
- `div <var> <var> <var> <var>` - Divide two variables, store the result in a third variable and remainder in a fourth variable
- `compare <var> <var> <var>` - Compare two variables, store the result in a third variable

- `read <var>` - Read one byte from stdin and store it in a variable
- `print <var>` - Print a variable to stdout
- `printc <var>` - Print the decimal value of a variable to stdout
- `print_msg <string>` - Print a static string to stdout

- `if_eq <var> <var>` - If two variables are equal, run the next block
- `if_eq <var> <const>` - If a variable is equal to a constant, run the next block
- `if_neq <var> <var>` - If two variables are not equal, run the next block
- `until_eq <var> <var>` - Run until a variables are equal
- `while_nz <var>` - Run while a variable is not zero
- `end` - End the current block

- `raw <string>` - Insert raw brainfuck code


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

// `res` is defined here, but a and b must be defined beforehand
compare a b res

if_eq res 0
    print_msg "Numbers are equal"
end

if_eq res 1
    print_msg "Right number is greater"
end

if_eq res 2
    print_msg "Left number is greater"
end
```


Here is fibonacci's sequence:

```js
set f_n-1 1
set f_n-2 1
set n 10
set one 1

printc f_n-1
print_msg " "
printc f_n-2
print_msg " "

dec_by n 2

while_nz n
    copy f_n-1 f_n
    add f_n f_n-2

    // Printc will pretty print numbers
    printc f_n

    if_neq n one
        print_msg " "
    end

    copy f_n-2 f_n-1
    copy f_n f_n-2

    dec n
end
// Also include a new line; '' and "" are equivalent
print_msg '\n'
```

See the LIR's [example folder](https://github.com/3top1a/alkoholiq/tree/main/examples/lir) for more examples.

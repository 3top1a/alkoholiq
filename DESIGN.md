# Alkoholiq design document

Let's start from the bottom and work up to a usable language
As the compexity grows n^2 where n is the number of features, each pass should be the simplest it can be, relying on the
power of abstraction.

## Lower IR

Due to the design of brainfuck, multiple limitations are imposed on the design of further passes.
For example, the absence of jump instructions means function calls must be implemented as a recursive match case
statement.

It is imperative that both the generated brainfuck code and LIR code leave the stack size determenistic.

Function calls can be implemented as a flag and switch case statement.
See [os.bf](https://github.com/bf-enterprise-solutions/os.bf/blob/master/os.bf).

For control structure design, see [bf.style](https://github.com/bf-enterprise-solutions/bf.style).
For if statements, see [bottom of this gist](https://gist.github.com/roachhd/dce54bec8ba55fb17d3a).

Brainfuck forces you to think about the memory layout before any code is written. The code gen is no different.
Before any code can be generated, the LIR needs to be analyzed and a concrete memory layout generated.
This step will also ensure that brainfuck can be generated.

The LIR does not have a concept of functions, instead a function switch like in os.bf is implemented in LIR form.
A function can be viewed as a set of instructions that has a side effect on the stack, just like a single instruction.

If, if else, else are implemented with the `Match` instruction.

Most operations should consume the top of the stack.
Inefficiencies caused by this should be optimized away by the optimizer.

The memory layout looks like this:

```
[+Variable storage] [+Working stack]
```

The variable storage stores variables and are retrieved and stored at runtime by their index of insertion. The size is
fixed and must be known at compile time.
The working stack is what the functions use for their computation. It can be as big as possible, allowing using it as a
heap for functions.
Temporary cells are used for duplicating cells safely.

LIR therefore only needs a few instructions:

1) Stack operations modifying the stack
    - `Push <value: immidiate>` - Push a value on top of the stack
    - `Pop` - Remove top value from stack
    - `Dup` - Duplicate top value

2) Data manipulation (with a modified and a consumed atom)
    - `Add <modified: var OR stack> <con: immidiate OR var OR stack>`
    - `Sub <modified: var OR stack> <con: immidiate OR var OR stack>`
    - `Mul` ... same as above
    - `Div` ... same as above
    - `Eq <modified: var OR stack> <con: immidiate OR var OR stack>` - Equality check, 1 if equal, 0 if not

3) Variable modification
    - `Move <to: var OR stack> <from: immidiate OR var OR stack>` - Moves a value from one place to another

4) I/O
    - `Read <to: var OR stack>` - Read from stdin
    - `Print <from: var OR stack OR immidiate>` - Print to stdout

5) Control loops
    - `Match <i: var OR stack>` - match works both as an if, if else, else
    - `While <i: var OR stack>` - run while `i` isn't zero

For example:

```asm
Push 65 ; [65]
Print (Stack) ; Consumes 65, leaving stack empty

Push 5 ; [5]
Dup ; [5] [5]
Eq ; [1] Tests for equality

Match (Stack) ; Consumes top of stack
    CaseDefault
        Print (67)
    Case (1)
        Print (66)
    Case (0)
        Print (65)
EndMatch

Read (Stack) ; Pushes one byte of used input to stack
Move (from: Stack) (to: Variable 0 ) ; Pops and puts it into variable storage

; This will print all ASCII characters from z to 0
; Assign to variable, 'z' is 122
Move (122) (Variable 0)
While (Variable 0)
    Print (Variable 0)
    Sub (Variable 0) (1)
EndWhile
```

NOTE: Match and while statements are defined in a recursive manner, not linear, this has been simplified for reading.

For simpler testing and development, a simple parser has been written.

```asm
// Move from -> to
mov 1, $0
while $0
    read stack
    dup
    match stack
    case 0
        mov 0, $0
    default
        print stack
    end
end
```

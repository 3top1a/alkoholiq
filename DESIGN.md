# Alkoholiq design document

Let's start from the bottom and work up to a usable language
As the compexity grows n^2 where n is the number of features, each pass should be the simplest it can be, relying on the power of abstraction.

## Lower IR

Due to the design of brainfuck, multiple limitations are imposed on the design of further passes.
For example, the absence of jump instructions means function calls must be implemented as a recursive match case statement.

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

If, else and while are implemented with `match` `case` instructions.
?? Is this optimal? Might make implementation easier but runtime slower.

Most operations should consume the top of the stack.
Inefficiencies caused by this should be optimized in the next step.

The memory layout looks like this:

```
[+variable storage] [2 temp] [+working stack]
```

The variable storage stores variables and are retrieved and stored at runtime by their index of insertion. The size is fixed and must be known at compile time.
The working stack is what the functions use for their computation. It can be as big as possible, allowing using it as a heap for functions.
Temporary cells are used for duplicating cells safely.

LIR therefore only needs a few instructions:
1) Stack operations modifying the stack
- Push <value: immidiate> - Push a value on top of the stack
- Pop - Remove top value from stack
- Dup - Duplicate top value

2) Data manipulation (with a modified and a consumed atom)
- Add <modified: var OR stack> <con: immidiate OR var OR stack>
- Sub <modified: var OR stack> <con: immidiate OR var OR stack>
- Mul ...
- Div ...

3) Variable modification
- Copy <to: var OR stack> <from: immidiate OR var OR stack>
?? Maybe one instruction is all I need?

4) I/O
- read <to: var OR stack>
- print <from: var OR stack OR immidiate>

5) Control loops
- match <i: var OR stack> - match works both as a if and while
- case <c: var OR stack OR immidiate OR *>
- endmatch


For efficiency, instructions can modify the stack or variables without copying.


# Alkoholiq design document

Features:
- Expression oriented, e.g. the return value(s) of a block can be used to assign a variable
- Use S-Expression syntax
- Optimizing for size and speed


Limitations:
- The size of all variables must be known at compile time as there is no heap

Let's start from the bottom and work up to a usable language


## Brainfuck

It is imperative that both the generated brainfuck code and LIR code leave the stack size determenistic.

Function calls can be implemented as a flag and switch case statement.
See [os.bf](https://github.com/bf-enterprise-solutions/os.bf/blob/master/os.bf).


For control structure design, see [bf.style](https://github.com/bf-enterprise-solutions/bf.style).
For if statements, see [bottom of this gist](https://gist.github.com/roachhd/dce54bec8ba55fb17d3a).


## Lower IR

Brainfuck forces you to think about the memory layout before any code is written. The code gen is no different.
Before any code can be generated, the LIR needs to be analyzed and a concrete memory layout generated.
This step will also ensure that brainfuck can be generated.

The LIR does not have a concept of functions, instead a function switch like in os.bf is implemented in LIR form.
A function can be viewed as a set of instructions that has a side effect on the stack, just like a single instruction.


If, else and while are implemented with `match` `case` instructions.
?? Is this optimal? Might make implementation easier but runtime slower.

Most operations should consume the top of the stack.
Inefficiencies caused by this should be optimized in the next step.




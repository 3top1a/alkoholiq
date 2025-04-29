# Alkoholiq

[![codecov](https://codecov.io/gh/3top1a/alkoholiq/graph/badge.svg?token=ZSXBRAWT1M)](https://codecov.io/gh/3top1a/alkoholiq)
![brain? fucked.](https://img.shields.io/badge/brain-fucked-darkred?style=flat)

```js
// Set some variables
set A 64
set Z 91
set a 96
set z 123

read char
while_nz char
    // Between A and Z
    compare char A res
    if_eq res 2
        compare char Z res
        if_eq res 1
            // Add 13
            dec_by char 13

            // Check for underflow
            compare char A res
            if_eq res 1
                // Wrap around
                inc_by char 26
            end
        end
    end


    // Between a and z
    compare char a res
    if_eq res 2
        compare char z res
        if_eq res 1
            // Add 13
            dec_by char 13

            // Check for underflow
            compare char a res
            if_eq res 1
                // Wrap around
                inc_by char 26
            end
        end
    end

    print char

    read char
end
```


Alkoholiq, a programming language that transpiles to Brainfuck.

You can pipe the syntax above into the program.
The output is not executable immediately, and needs to be piped into a brainfuck interpreter/compiler such as [my 540 byte brainfuck interpreter](https://github.com/3top1a/sbfi-rs) or the online [Brainfuck Debugger](https://kvbc.github.io/bf-ide/).

For example, run
```bash
cat examples/lir/fib.lir | cargo r
```
and copy the output into the online Brainfuck Debugger.


The internals are explained in the [DESIGN.md](https://github.com/3top1a/alkoholiq/blob/main/DESIGN.md) document.

## The name?

The name stems from alcoholism, because, well, I'm from the Czech Republic where beer is cheaper than water.
*Alkoholik* is a short Czech translation of an alcohol addict.
Inspired by our automotive manufacturer, *Škoda*, that have started naming all of their new cars with the letter q at the end, I decided to do the same.
Tune in next time for *QDE Qonnect*.

## Q&A

I will not be taking any questions, thank you.

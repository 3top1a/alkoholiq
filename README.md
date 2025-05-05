# Alkoholiq

[![codecov](https://codecov.io/gh/3top1a/alkoholiq/graph/badge.svg?token=ZSXBRAWT1M)](https://codecov.io/gh/3top1a/alkoholiq)
![brain? fucked.](https://img.shields.io/badge/brain-fucked-darkred?style=flat)

```apache
set f_n-1 1
set f_n-2 1
set n 10

printc f_n-1
print_msg " "
printc f_n-2
print_msg " "

dec_by n 2

while_nz n
    copy f_n-1 f_n
    add f_n f_n-2

    printc f_n

    if_neq n 1
        print_msg ' '
    end

    copy f_n-2 f_n-1
    copy f_n f_n-2

    dec n
end
print_msg '\n'
```


Alkoholiq, a programming language that transpiles to Brainfuck.

You can pipe the syntax above into the program, or put it into a file.
The binary can either compile the code and print it, or it can also interpret the code and print what it outputs.

For example, run
```bash
cargo r -- examples/lir/fib.lir
# 1 1 2 3 5 8 13 21 34 55
```

or to see the compiled Brainfuck code:
```bash
cargo r -- examples/lir/fib.lir -b
# ... safe to say it's long
```


The internals are explained in the [DESIGN.md](https://github.com/3top1a/alkoholiq/blob/main/DESIGN.md) document.

Examples can be found in the [examples](https://github.com/3top1a/alkoholiq/tree/main/examples/lir) folder.

## The name?

The name stems from alcoholism, because, well, I'm from the Czech Republic where beer is cheaper than water.
*Alkoholik* is a short Czech translation of an alcohol addict.
Inspired by our automotive manufacturer, *Škoda*, that have started naming all of their new cars with the letter q at the end, I decided to do the same.
Tune in next time for *QDE Qonnect*.

## Q&A

I will not be taking any questions, thank you.

## License

Licensed under [Good Luck With That Shit Public License](https://github.com/me-shaon/GLWTPL/tree/master) or [Derivative Gardens Misattribution-OnlyCommercial-ShareUnlike 6.9 Unportable License](https://www.boringcactus.com/2023/06/15/the-derivative-gardens-license.html), which ever you vibe with more.

![Good luck GIF](https://github.com/me-shaon/GLWTPL/blob/master/good-luck.gif?raw=true)


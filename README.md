# brainfuck

A lightning-fast brainfuck interpreter in Rust.

## What is brainfuck

Brainfuck is an esoteric programming language created in 1993 by Urban
Müller. The language only has 8 commands and a single block of memory
consisting only of bytes, as well as a pointer to an byte in the memory.

| Op    | Description                             |
| ----- | --------------------------------------- |
| `+`   | Increment value of pointer by 1         |
| `-`   | Decrement value of pointer by 1         |
| `>`   | Move pointer one cell forwards          |
| `<`   | Move pointer one cell backwards         |
| `.`   | Print the value of pointer as a `char`  |
| `,`   | Set the value of the pointer from input |
| `[`   | If the value of the pointer is zero, skip forwards to corresponding `]`        |
| `]`   | If the value of the pointer is not zero, skip backwards to corresponding `[` ] |

## Usage

```
Usage: bf <SRC>

Arguments:
  <SRC>

Options:
  -h, --help  Print help information
```

The `SRC` passed to the interpreter can either be the path to a file
to interpret, or a string.

```console
foo@bar:~$ ./bf hello_world.bf
```

```console
foo@bar:~$ ./bf "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
```

## Features

| Feature                | Description                                     | Default |
| ---------------------- | ----------------------------------------------- | ------- |
| `comments`             | Interpret any unknown character as a comment    | `true`  |
| `debug_token`          | Print memory content on every `#`               | `false` |
| `precompiled_patterns` | Optimize source code with pre-compiled patterns | `true`  |


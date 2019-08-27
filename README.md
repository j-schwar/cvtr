# cvtr

`cvtr` (convert-radix) is a simple radix conversion command line tool.

# Usage
```
$ cvtr --help
cvtr 0.1.0
CLI numeric base converter

USAGE:
    cvtr [FLAGS] [OPTIONS] <number>

FLAGS:
    -b, --bin        Convert number to binary
    -d, --dec        Convert number to decimal
    -x, --hex        Convert number to hexadecimal
    -o, --oct        Convert number to octal
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --radix <radix>    Radix of input number

ARGS:
    <number>    Number to convert
```

# Examples

From hex to decimal

```
$ cvtr -d 0x20
decimal: 32
```

From decimal to hex

```
$ cvtr -x 32
hex:     20
```

# cvtr

`cvtr` (convert-radix) is a simple radix conversion command line tool.

# Usage
```
$ cvtr --help
USAGE:
    cvtr [FLAGS] <number>

FLAGS:
    -b               Convert number to binary
    -d               Convert number to decimal
    -x               Convert number to hexadecimal
    -o               Convert number to octal
    -B               Convert number from binary
    -D               Convert number from decimal
    -X               Convert number from hexadecimal
    -O               Convert number from octal
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <number>    Number to convert
```

# Examples

From hex to decimal

```
$ cvtr -d 0x20
decimal:   32
```

Or without a prefix, using `-X` to specify a input radix of 16

```
$ cvtr -Xd 20
decimal:   32
```

From decimal to hex

```
$ cvtr -x 32
hex:       20
```

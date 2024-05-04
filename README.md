# nme-c

**nme-c** is a C compiler for the `nme` [6502](https://en.wikipedia.org/wiki/WDC_65C02)-based breadboard computer. It is written in Rust, and compiles a subset of the C language to 6502 assembly.

# Usage

## Keywords

The compiler supports the following C keywords:
```

```

```
auto        enum         restrict    unsigned
break       extern       return      void
case        float        short       volatile
char        for          signed      while
const       goto         sizeof      _Bool
continue    if           static      _Complex
default     inline       struct      _Imaginary
do          int          switch
double      long         typedef
else        register     union
```

It does not support:
```
auto        enum         restrict    unsigned
break       extern       return      void
case        float        short       volatile
char        for          signed      while
const       goto         sizeof      _Bool
continue    if           static      _Complex
default     inline       struct      _Imaginary
do          int          switch
double      long         typedef
else        register     union
```

# Acknowledgements

- [cc65](https://github.com/cc65/cc65)
- [6502js](https://itema-as.github.io/6502js/)
- [wrecc](https://github.com/PhilippRados/wrecc)

# License

This project is licensed under the [MIT License](https://github.com/Infinixius/nme-c/blob/main/LICENSE). You are free to use, modify, and distribute the compiler as you see fit.
# nme-c

**nme-c** is a C compiler for the `nme` [6502](https://en.wikipedia.org/wiki/WDC_65C02)-based breadboard computer, and was created for our school's 2023-2024 year long project. It is written in Rust, and compiles a subset of the C language to 6502 assembly.

# Usage

## Keywords

The compiler supports the following C keywords:

![keyword](https://github.com/Infinixius/nme-c/blob/main/.github/keywords.png)

In addition, all keywords that start with `_` are also unsupported (`_Bool`, `_Complex, etc). Most keywords that are unsupported are because of the limited architecture of the 6502, or because of the complexity of implementing them.

# Acknowledgements

- [cc65](https://github.com/cc65/cc65)
- [6502js](https://itema-as.github.io/6502js/)
- [wrecc](https://github.com/PhilippRados/wrecc)

# License

This project is licensed under the [MIT License](https://github.com/Infinixius/nme-c/blob/main/LICENSE). You are free to use, modify, and distribute the compiler as you see fit.

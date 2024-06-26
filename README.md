# nme-c

**nme-c** is a C compiler for the `nme` [6502](https://en.wikipedia.org/wiki/MOS_Technology_6502)-based breadboard computer, and was created for our school's 2023-2024 year long project. It is written in Rust, and compiles a subset of the C language to 6502 assembly.

![showcase](https://github.com/Infinixius/nme-c/blob/main/.github/showcase.png)

The core of this project is the [MOS Technology 6502](https://en.wikipedia.org/wiki/MOS_Technology_6502), an 8-bit microprocessor from the 1970s. The original ship was used in many retro computers and consoles from back in the day, from the [Commodore 64](https://en.wikipedia.org/wiki/Commodore_64) to the [NES](https://en.wikipedia.org/wiki/Nintendo_Entertainment_System). More specifically, we are using the [WDC 65C02](https://en.wikipedia.org/wiki/WDC_65C02), a modern version of the original chip with additional capabilities, aswell as fixes for some of the original's bugs. For graphics, we are using the [TMS9918](https://en.wikipedia.org/wiki/TMS9918), and the [KM62256CLP](https://octopart.com/km62256clp-7-samsung-7411989) 32k x 8-bit SRAM for memory.

# Usage

## Installation

Executables can be found on the [releases](https://github.com/Infinixius/nme-c/releases) page. To build manually:

```bash
git clone https://github.com/Infinixius/nme-c.git
cd nme-c
cargo build

# Executable should be located at nme-c/target/debug/nme-c.exe
```

## Keywords

The compiler supports the following C keywords:

![keywords](https://github.com/Infinixius/nme-c/blob/main/.github/keywords.png)

In addition, all keywords that start with `_` are also unsupported (`_Bool`, `_Complex, etc). Most keywords that are unsupported are because of the limited architecture of the 6502, or simply because of the complexity of implementing them.


## Examples

```c

```

## Header

Due to the limited nature of the 6502 microprocessor, none of the [C standard library](https://en.wikipedia.org/wiki/C_standard_library) is supported. Instead, the `nme.h` header file is provided for use with the compiler as a psuedo-standard library. It provides the following functions:

```h
/* Booleans */
typedef int bool;
#define true 1
#define false 0

/* Math functons */

int add(int x, int y);
int sub(int x, int y);
int mul(int x, int y);
int div(int x, int y);
int mod(int x, int y);

int abs(int x);
int min(int x, int y);
int max(int x, int y);
int pow(int x, int y);

/* Graphics functions */

#define COLOR int
#define TRANSPARENT 0
#define BLACK 1
#define MEDIUM_GREEN 2
#define LIGHT_GREEN 3
#define DARK_BLUE 4
#define LIGHT_BLUE 5
#define DARK_RED 6
#define CYAN 7
#define MEDIUM_RED 8
#define LIGHT_RED 9
#define DARK_YELLOW 10
#define LIGHT_YELLOW 11
#define DARK_GREEN 12
#define MAGENTA 13
#define GRAY 14
#define WHITE 15

int get_screen_width();
int get_screen_height();

void clear_screen();
void fill_screen(COLOR color);

void get_pixel(int x, int y);
void set_pixel(int x, int y, COLOR color);

// These functions are technically macros, the compiler
// will replace them with set_pixel calls at compile time.
void draw_line(int x1, int y1, int x2, int y2, COLOR color);
void draw_rect(int x, int y, int width, int height, COLOR color);
void draw_circle(int x, int y, int radius, COLOR color);
void draw_text(int x, int y, char* text, COLOR color);
void draw_image(int x, int y, int w, int h, char* file_path);

/* I/O & misc functions */

int get_key_press();
int get_random_number();
void print(char* text);
void sleep(int ms);

// In addition, the asm() function is supported for inline assembly.
// Example: asm("LDA #$01")
```

# Acknowledgements

- [cc65](https://github.com/cc65/cc65)
- [6502js](https://itema-as.github.io/6502js/)
- [wrecc](https://github.com/PhilippRados/wrecc)
- [Ben Eater](https://www.youtube.com/@BenEater)

This project was created by:

- [r3ggo](https://github.com/r3ggo/)
- [infinixius](https://github.com/Infinixius/)
- [gianni](https://github.com/xogianni)
- [funset](https://github.com/fungset)

# License

This project is licensed under the [MIT License](https://github.com/Infinixius/nme-c/blob/main/LICENSE). You are free to use, modify, and distribute the compiler as you see fit.

```
Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.```
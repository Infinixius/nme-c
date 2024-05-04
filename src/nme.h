// nme.h
// This header file is the "standard library" for the nme-c compiler.

#ifndef NME_H
#define NME_H

/* Booleans */
typedef int bool;
#define true 1
#define false 0

/* Math functons */

int add(int x, int y);
int sub(int x, int y);
int mul(int x, int y);
int div(int x, int y);

// https://gist.github.com/hausdorff/5993556
int mod(int x, int y);

int abs(int x);
int min(int x, int y);
int max(int x, int y);
int pow(int x, int y);
int sqrt(int x);

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

#endif
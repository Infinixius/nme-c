#include <nme.h>

void main() {
	int x = 1;

	// Assignment

	x = 5;
	x += 5;
	x -= 5;
	x *= 5;
	x /= 5;
	x %= 5;
	x &= 5;
	x |= 5;
	x ^= 5;
	x <<= 5;
	x >>= 5;

	// Unary operators

	+5;
	-5;

	// Increment/Decrement

	x++;
	x--;

	// Relational operators

	int a = 3;
	int b = 7;

	a < b;
	a > b;
	a <= b;
	a >= b;
	a == b;
	a != b;

	// Logical operators

	bool t = true;
	bool f = false;

	t && f;
	t || f;
	!t;

	// Bitwise operators

	a & b;
	a | b;
	a ^ b;
	~a;
	a << 1;
	a >> 1;
}
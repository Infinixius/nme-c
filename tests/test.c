// Single-line comment

/*
  Multi-line
  comment
*/

typedef int bool;
#define true 1
#define false 0

void main() {
  // Variables & arithmetic

  int x = 1;
  int y = 2;

  int u;

  int a = x + y;
  int s = x - y;

  int m = mul(x, y);
  int d = div(x, y);

  // Printing

  printf("Hello, World!\n");
  printf("The sum of %d and %d is %d\n", x, y, a);
  printf("The difference of %d and %d is %d\n", x, y, s);
  printf("The product of %d and %d is %d\n", x, y, m);
  printf("The quotient of %d and %d is %d\n", x, y, d);

  // Some more advanced arithmetic

  int a1 = 1 + 2 * 3;
  int b2 = (1 + 2) * 3;
  int c3 = 1 + 2 / 3;
  int d4 = (1 + 2) / 3;

  printf("The result of 1 + 2 * 3 is %d\n", a1);
  printf("The result of (1 + 2) * 3 is %d\n", b2);
  printf("The result of 1 + 2 / 3 is %d\n", c3);
  printf("The result of (1 + 2) / 3 is %d\n", d4);

  // If statements

  if (5 > 3) {
    printf("5 is greater than 3\n");
  } else {
    printf("5 is not greater than 3\n");
  }

  // Loops

  for (int f = 1; f <= 10; f++) {
    printf("For loop: %d\n", f);
  }

  int w = 10;
  while (w > 0) {
    printf("While loop: %d\n", w);
    w--;
  }
  
  int dw = 10;
  do {
    printf("Do-while loop: %d\n", dw);
    dw--;
  } while (dw > 0);

  // Arrays

  int arr[5] = {1, 2, 3, 4, 5};

  for (int i = 0; i < 5; i++) {
    printf("Array element: %d\n", arr[i]);
  }

  // Pointers

  int g = 10;
  int h = 20;

  int *p = &g;
  int *q = &h;

  printf("The pointer %p points to the value %d\n", p, *p);
  printf("The pointer %p points to the value %d\n", q, *q);

  *p = 15;
  *q = 25;

  printf("The pointer %p now points to the value %d\n", p, *p);
  printf("The pointer %p now points to the value %d\n", q, *q);

  // Structs
  
  struct Point {
    int x;
    int y;
  };

  struct Point p1 = {1, 2};

  printf("The point is (%d, %d)\n", p1.x, p1.y);

  // Typedefs/booleans

  bool b = true;
  
  if (b) {
    printf("The boolean is true\n");
  } else {
    printf("The boolean is false\n");
  }

  // Enums & switch statements

  enum Color {
    RED,
    GREEN,
    BLUE
  };

  enum Color c = GREEN;
  
  switch (c) {
    case RED:
      printf("The color is red\n");
      break;
    case GREEN:
      printf("The color is green\n");
      break;
    case BLUE:
      printf("The color is blue\n");
      break;
  }

  // Bitwise operators

  int i = 5;
  int j = 10;
  
  int bitwise_and = i & j;
  int bitwise_or = i | j;
  int bitwise_xor = i ^ j;
  int bitwise_not = ~i;
  int left_shift = i << 1;
  int right_shift = i >> 1;

  printf("The bitwise AND of %d and %d is %d\n", i, j, bitwise_and);
  printf("The bitwise OR of %d and %d is %d\n", i, j, bitwise_or);
  printf("The bitwise XOR of %d and %d is %d\n", i, j, bitwise_xor);
  printf("The bitwise NOT of %d is %d\n", i, bitwise_not);
  printf("The left shift of %d is %d\n", i, left_shift);
  printf("The right shift of %d is %d\n", i, right_shift);

  // Ternary operator

  int k = 5;
  int l = 10;

  char result = k > l ? 'y' : 'n';

  printf("Ternary operator result: %c\n", result);

  // Recursion

  printf("The factorial of 5 is %d\n", factorial(5));
  printf("The factorial of 10 is %d\n", factorial(10));
}

int mul(int a, int b) {
  return a * b;
}

int div(int a, int b) {
  return a / b;
}

int factorial(int n) {
  if (n == 1) {
    return 1;
  } else {
    return n * factorial(n - 1);
  }
}
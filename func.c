#include <stdio.h>

int foo() {
    printf("func foo OK\n");
    return 32;
}

int bar(int x, int y) {
    printf("func bar OK %d %d\n", x, y);
    return 75;
}

int baz(int a, int b, int c, int d, int e, int f) {
    printf("func baz OK %d %d %d %d %d %d\n", a, b, c, d, e, f);
    return 125;
}
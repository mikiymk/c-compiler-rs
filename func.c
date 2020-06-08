#include <stdio.h>
#include <stdlib.h>

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

int foobar(int d) {
    printf("func foobar OK %d\n", d);
    return d;
}

int alloc4(int **p, int a, int b, int c, int d) {
    *p = (int *)malloc(sizeof(int) * 4);
    if (*p == NULL) printf("null\n");
    *(*p + 0) = a;
    *(*p + 1) = b;
    *(*p + 2) = c;
    *(*p + 3) = d;
    return 205;
}
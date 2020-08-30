#include "func.h"
int main() {
    int a;
    int p[10];
    int *q;
    int *r;
    p[5] = 15;
    a = 5;
    q = p + a;
    r = q + p;
    return *q;
}
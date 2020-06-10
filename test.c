#include "func.h"
int main() {
    int a[10];
    int *p;
    *a = 99;
    *(a + 1) = 102;
    p = a;
    return *(p + 1);
}
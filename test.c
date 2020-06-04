#include "func.h"

int test() {
    { return 0; }
    { return 42; }
    { return 31+22; }
    { return 55-43; }
    { return 5+20-4; }
    { return 12+34-5; }

    { return 8*9; }
    { return 15/3; }
    { return 5+6*7; }
    { return 5*(9-6); }
    { return (3+5)/2; }

    { return +5; }
    { return -7; }
    { return -4+5; }
    // { return 6--8; }
    { return 6- -8; }
    { return +5-4*-5; }

    { return 1 == 5; }
    { return 1 != 5; }
    { return 1 < 5; }
    { return 1 <= 5; }
    { return 1 > 5; }
    { return 1 >= 5; }
    { return 2+3>=4*5; }
    { return (3*7+(5-1==-3))*3; }

    { int a; return a=5; }
    { int a; a=3; return a*2; }
    { int a; int b; a=15; b=127; return b; }
    { int a; int b; a=15; b=127; return a; }
    { int a; int b; a=5; b=1; return a+b; }
    { int a; int b; int c; int d; int e; int f; int g; int h; int i; int j; int k; int l; int m; int n; int o; int p; int q; int r; int s; int t; int u; int v; int w; int x; int y; int z; a=b=c=d=e=f=g=h=i=j=k=l=m=n=o=p=q=r=s=t=u=v=w=x=y=z=1; return a; }
    { int foo; int bar; foo=1; bar=2+3; return foo+bar; }

    { return 89; return 7+5; }
    { int a; int b; a=53; return a; a=0; b=2; }

    { int a; a=32; if(4>2) return a; return 5+13; }
    { int a; a=32; if(4<2) return a; return 5+13; }
    { int a; a=32; if(4>2) { a=13; a=a*2; } return 5+a; }
    { int a; a=32; if(4<2) { a=13; a=a*2; } return 5+a; }

    { int a; a=100; if(11==10) a=5; else a=10; return a; }
    { int a; a=100; if(10==10) a=5; else a=10; return a; }
    { int a; a=100; if(11==10) { a=2*a; return a; } else { a=2+a; return a; } return 3; }
    { int a; a=100; if(10==10) { a=2*a; return a; } else { a=2+a; return a; } return 3; }

    { int tkg; tkg=10; while(tkg) tkg=tkg-1; return 38+tkg; }
    { int tkg; tkg=10; while(tkg) { tkg=tkg-1; return tkg; } return 38+tkg; }

    { int s; int i; s=2; for(i=0; i<=10; i=i+1) s=s+i; return s; }
    { int s; int i; int i2; s=93; for(i=0; i<=10; i=i+1) { for(i2=0; i2<=i; i2=i2+1){ if(i*i2>s) s=s+i+i2+10; } } return s; }

    { int a; a=foo(); return a; }
    { int a; int b; int c; a=5; b=86; c=bar(a,b); return c; }
    { int a; int b; int c; int d; int e; int f; int g; a=30; b=28; c=56; d=83; e=127; f=204; g=baz(a,b,c,d,e,f); return g; }

    // { return hoge(); } int hoge() { return 5; }
    // { return hoge(62); } int hoge(int a) { return a; }
    // { return hoge(1, 1, 1, 1, 1, 1); } int hoge(int a, int b, int c, int d, int e, int f) { return a+b+c+d+e+f; }
    // { return hoge(5); } int hoge(int a) { if(a==0) return 1; else if(a==1) return 1; else return hoge(a-1)+hoge(a-2); }

    { int a; int *b; b = &a; *b = 14; return a; }
    { int a; int *b; int **c; b = &a; c = &b; **c = 15; return a; }
    { int a; int *b; int **c; int ***d; int ****e; int *****f; b = &a; c = &b; d = &c; e = &d; f = &e; *****f = 16; return a; }

    { int *p; int *q; int *r; alloc4(&p, 17, 2, 19, 18); return *p; }
    { int *p; int *q; int *r; alloc4(&p, 17, 2, 19, 18); q = p + 3; r = q - 1; return *q; }
    { int *p; int *q; int *r; alloc4(&p, 17, 2, 19, 18); q = p + 3; r = q - 1; return *r; }
    { return sizeof 1; }
    { int a; return sizeof a; }
    { int a; return sizeof (a + 1); }
    { int *a; return sizeof a; }
}
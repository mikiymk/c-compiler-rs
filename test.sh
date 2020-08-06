#!/bin/bash

# プログラム文が期待する戻り値を返すことを確かめる関数。
# 期待する戻り値でない場合、シェルスクリプトを終了する。
assert() {
    expected="$1"
    input="$2"

    ./target/debug/ccc9 "$input" > tmp.s
    cc -o tmp tmp.s func.o
    ./tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
        echo
    else
        echo "$input => $expected expected, but got $actual"
        exit 1
    fi
}

# 1つの数字
assert 0 "int main() { return 0; }"
assert 42 "int main() { return 42; }"

# 足し算 引き算
assert 53 "int main() { return 31+22; }"
assert 12 "int main() { return 55-43; }"
assert 21 "int main() { return 5+20-4; }"
assert 41 "int main() { return 12+34-5; }"

# 掛け算 割り算 ()
assert 72 "int main() { return 8*9; }"
assert 5 "int main() { return 15/3; }"
assert 47 "int main() { return 5+6*7; }"
assert 15 "int main() { return 5*(9-6); }"
assert 4 "int main() { return (3+5)/2; }"

# 単項 + -
assert 5 "int main() { return +5; }"
assert 249 "int main() { return -7; }"
assert 1 "int main() { return -4+5; }"
assert 14 "int main() { return 6--8; }"
assert 25 "int main() { return +5-4*-5; }"

# 比較 == != < <= > >=
assert 0 "int main() { return 1 == 5; }"
assert 1 "int main() { return 1 != 5; }"
assert 1 "int main() { return 1 < 5; }"
assert 1 "int main() { return 1 <= 5; }"
assert 0 "int main() { return 1 > 5; }"
assert 0 "int main() { return 1 >= 5; }"
assert 0 "int main() { return 2+3>=4*5; }"
assert 63 "int main() { return (3*7+(5-1==-3))*3; }"

# 代入 =
assert 5 "int main() { int a; return a=5; }"
assert 6 "int main() { int a; a=3; return a*2; }"
assert 127 "int main() { int a; int b; a=15; b=127; return b; }"
assert 15 "int main() { int a; int b; a=15; b=127; return a; }"
assert 6 "int main() { int a; int b; a=5; b=1; return a+b; }"
assert 1 "int main() { int a; int b; int c; int d; int e; int f; int g; int h; int i; int j; int k; int l; int m; int n; int o; int p; int q; int r; int s; int t; int u; int v; int w; int x; int y; int z; a=b=c=d=e=f=g=h=i=j=k=l=m=n=o=p=q=r=s=t=u=v=w=x=y=z=1; return a; }"
assert 6 "int main() { int foo; int bar; foo=1; bar=2+3; return foo+bar; }"

# return文
assert 89 "int main() { return 89; 7+5; }"
assert 53 "int main() { int a; int b; a=53; return a; a=0; b=2; }"

# if文
assert 32 "int main() { int a; a=32; if(4>2) return a; return 5+13; }"
assert 18 "int main() { int a; a=32; if(4<2) return a; return 5+13; }"
assert 31 "int main() { int a; a=32; if(4>2) { a=13; a=a*2; } return 5+a; }"
assert 37 "int main() { int a; a=32; if(4<2) { a=13; a=a*2; } return 5+a; }"

# if-else文
assert 10 "int main() { int a; a=100; if(11==10) a=5; else a=10; return a; }"
assert 5 "int main() { int a; a=100; if(10==10) a=5; else a=10; return a; }"
assert 102 "int main() { int a; a=100; if(11==10) { a=2*a; return a; } else { a=2+a; return a; } return 3; }"
assert 200 "int main() { int a; a=100; if(10==10) { a=2*a; return a; } else { a=2+a; return a; } return 3; }"
assert 50 "int main() { if(1) { return 50; } else { return 40; } return 30; }"
assert 40 "int main() { if(0) { return 50; } else { return 40; } return 30; }"
assert 51 "int main() { int a; a=1; if(a) { return 51; } else { return 41; } return 31; }"
assert 41 "int main() { int a; a=0; if(a) { return 51; } else { return 41; } return 31; }"

# while文
assert 38 "int main() { int tkg; tkg=10; while(tkg) tkg=tkg-1; return 38+tkg; }"
assert 9 "int main() { int tkg; tkg=10; while(tkg) { tkg=tkg-1; return tkg; } return 38+tkg; }"
assert 32 "int main() { int tkg; int gms; tkg=10; gms=2; while(tkg>6) { tkg=tkg-1; gms=gms*2; } return gms; }"

# for文
assert 57 "int main() { int s; int i; s=2; for(i=0; i<=10; i=i+1) s=s+i; return s; }"
assert 123 "int main() { int s; int i; int i2; s=93; for(i=0; i<=10; i=i+1) { for(i2=0; i2<=i; i2=i2+1){ if(i*i2>s) s=s+i+i2+10; } } return s; }"
assert 15 "int main() { for(;;) { return 15; } return 20; }"

# 関数呼び出し
assert 32 "int main() { int a; a=foo(); return a; }"
assert 32 "int main() { return foo(); }"
assert 75 "int main() { int a; int b; int c; a=5; b=86; c=bar(a,b); return c; }"
assert 75 "int main() { return bar(26,85); }"
assert 125 "int main() { int a; int b; int c; int d; int e; int f; int g; a=30; b=28; c=56; d=83; e=127; f=204; g=baz(a,b,c,d,e,f); return g; }"
assert 125 "int main() { return baz(32,587,65,79,0,4); }"
assert 154 "int main() { int a; int b; a=154; b=foobar(a); return b; }"
assert 28 "int main() { return foobar(28); }"

# 関数定義
assert 5 "int main() { return hoge(); } int hoge() { return 5; }"
assert 5 "int main() { int a; a=hoge(); return a; } int hoge() { return 5; }"
assert 62 "int main() { return hoge(62); } int hoge(int a) { return a; }"
assert 137 "int main() { return hoge(62, 75); } int hoge(int a, int b) { return a+b; }"
assert 21 "int main() { return hoge(1, 2, 3, 4, 5, 6); } int hoge(int a, int b, int c, int d, int e, int f) { return a+b+c+d+e+f; }"
assert 8 "int main() { return hoge(5); } int hoge(int a) { if(a==0) return 1; else if(a==1) return 1; else return hoge(a-1)+hoge(a-2); }"

# ポインタ
assert 14 "int main() { int a; int *b; b = &a; *b = 14; return a; }"
assert 114 "int main() { int a; int *b; int *c; b = &a; c = b; *c = 114; return a; }"
assert 15 "int main() { int a; int *b; int **c; b = &a; c = &b; **c = 15; return a; }"
assert 16 "int main() { int a; int *b; int **c; int ***d; int ****e; int *****f; b = &a; c = &b; d = &c; e = &d; f = &e; *****f = 16; return a; }"

# ポインタの加減算
assert 17 "int main() { int *p; alloc4(&p, 17, 2, 19, 18); return *p; }"
assert 2 "int main() { int *p; alloc4(&p, 17, 2, 19, 18); return *(p + 1); }"
assert 18 "int main() { int *p; int *q; alloc4(&p, 17, 2, 19, 18); q = p + 3; return *q; }"
assert 19 "int main() { int *p; int *q; int *r; alloc4(&p, 17, 2, 19, 18); q = p + 3; r = q - 1; return *r; }"
assert 20 "int main() { int *p; int *q; alloc4(&p, 17, 2, 19, 18); q = p + 3; *(p + 3) = 20; return *q; }"

# sizeof演算子
assert 4 "int main() { return sizeof 1; }"
assert 4 "int main() { int a; return sizeof a; }"
assert 4 "int main() { int a; return sizeof (a + 1); }"
assert 8 "int main() { int *a; return sizeof a; }"

# 配列
assert 12 "int main() { int a[10]; return 12; }"
assert 25 "int main() { int a[10]; *a = 1; return 25; }"
assert 87 "int main() { int a[10]; *a = 87; return *a; }"
assert 50 "int main() { int a[10]; *(a + 1) = 11; return 50; }"
assert 98 "int main() { int a[1]; *a = 98; return *a; }"
assert 39 "int main() { int a[2]; *(a + 1) = 39; return *(a + 1); }"
assert 71 "int main() { int a[10]; *a = 71; *(a + 1) = 89; return *a; }"
assert 160 "int main() { int a[10]; *a = 71; *(a + 1) = 89; return *a + *(a + 1); }"
assert 160 "int main() { int a[2]; *a = 71; *(a + 1) = 89; return *a + *(a + 1); }"
assert 99 "int main() { int a[10]; int *p; *a = 99; p = a; return *p; }"
assert 102 "int main() { int a[10]; int *p; *a = 99; *(a + 1) = 102; p = a; return *(p + 1); }"
assert 99 "int main() { int a[1]; int *p; *a = 99; p = a; return *p; }"
assert 99 "int main() { int a[1]; int *p; *a = 99; p = a; return *p; }"

assert 3 "int main() {
    int a[2];
    *a = 1;
    *(a + 1) = 2;
    int *p;
    p = a;
    return *p + *(p + 1);  // → 3
}"

assert 40 "int main() { int a[10]; return sizeof a; }"
assert 80 "int main() { int *a[10]; return sizeof a; }"
assert 84 "int main() { int a[21]; return sizeof a; }"

echo OK

#!/bin/bash
assert() {
    expected="$1"
    input="$2"

    ./target/debug/ccc9 "$input" > tmp.s
    cc -o tmp tmp.s func.o
    ./tmp
    actual="$?"

    if [ "$actual" =  "$expected" ]; then
        echo "$input => $actual"
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

# while文
assert 38 "int main() { int tkg; tkg=10; while(tkg) tkg=tkg-1; return 38+tkg; }"
assert 9 "int main() { int tkg; tkg=10; while(tkg) { tkg=tkg-1; return tkg; } return 38+tkg; }"

# for文
assert 57 "int main() { int s; int i; s=2; for(i=0; i<=10; i=i+1) s=s+i; return s; }"
assert 123 "int main() { int s; int i; int i2; s=93; for(i=0; i<=10; i=i+1) { for(i2=0; i2<=i; i2=i2+1){ if(i*i2>s) s=s+i+i2+10; } } return s; }"

# 関数呼び出し
assert 32 "int main() { int a; a=foo(); return a; }"
assert 75 "int main() { int a; int b; int c; a=5; b=86; c=bar(a,b); return c; }"
assert 125 "int main() { int a; int b; int c; int d; int e; int f; int g; a=30; b=28; c=56; d=83; e=127; f=204; g=baz(a,b,c,d,e,f); return g; }"

# 関数定義
assert 5 "int main() { return hoge(); } int hoge() { return 5; }"
assert 62 "int main() { return hoge(62); } int hoge(int a) { return a; }"
assert 6 "int main() { return hoge(1, 1, 1, 1, 1, 1); } int hoge(int a, int b, int c, int d, int e, int f) { return a+b+c+d+e+f; }"
assert 8 "int main() { return hoge(5); } int hoge(int a) { if(a==0) return 1; else if(a==1) return 1; else return hoge(a-1)+hoge(a-2); }"

# ポインタ
assert 14 "int main() { int a; int *b; b = &a; *b = 14; return a; }"
assert 15 "int main() { int a; int *b; int **c; b = &a; c = &b; **c = 15; return a; }"
assert 16 "int main() { int a; int *b; int **c; int ***d; int ****e; int *****f; b = &a; c = &b; d = &c; e = &d; f = &e; *****f = 16; return a; }"

# ポインタの加減算
assert 17 "int main() { int *p; int *q; int *r; alloc4(&p, 17, 2, 19, 18); return *p; }"
assert 18 "int main() { int *p; int *q; int *r; alloc4(&p, 17, 2, 19, 18); q = p + 3; r = q - 1; return *q; }"
assert 19 "int main() { int *p; int *q; int *r; alloc4(&p, 17, 2, 19, 18); q = p + 3; r = q - 1; return *r; }"

echo OK

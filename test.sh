#!/bin/bash
assert() {
    expected="$1"
    input="$2"

    ./ccc9 "$input" > tmp.s
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

# assert 0 "0;"
# assert 42 "42;"
# assert 21 "5+20-4;"
# assert 41 " 12 + 34 - 5 ;"
# assert 47 "5+6*7;"
# assert 15 "5*(9-6);"
# assert 4 "(3+5)/2;"
# assert 25 "+5-4*-5;"
# assert 1 "1 < 5;"
# assert 0 "2+3>=4*5;"
# assert 63 "(3*7+(5-1==-3))*3;"
# assert 82 "32+5; 54; 100-18;"
# assert 5 "a=5;"
# assert 6 "a=3;a*2;"
# assert 6 "a=5;b=1;a+b;"
# assert 1 "a=b=c=d=e=f=g=h=i=j=k=l=m=n=o=p=q=r=s=t=u=v=w=x=y=z=1;a;"
# assert 6 "foo = 1; bar = 2 + 3; foo + bar;"
# assert 89 "return 89; 7+5;"
# assert 53 "a=53; return a; a=0; b=2;"
# assert 32 "a=32; if (4 > 2) return a; 5+3;"
# assert 10 "a=100; b=10; if (11 == b) 5; else a / b;"
# assert 38 "tkg=10; while( tkg) tkg=tkg-1;38+tkg;"
# assert 57 "s=2;for(i=0;i<=10;i=i+1)s=s+i;return s;"
# assert 123 "s=93; for(i=0;i<=10;i=i+1){for(i2=0;i2<=i;i2=i2+1){if(i*i2>s)s=s+i+i2+10;}}return s;"
# assert 47 "a=b=3;b=c=93; if(a<b){c=47;}return c;"
# assert 32 "a=foo();a;"
# assert 75 "a=5;b=86;c=bar(a,b);return c;"

# 関数定義できるようになった。
# main関数が最初に定義されるようになった。
assert 0 "main() return 0;"
assert 1 "main() if(2>3) return 2; else return 1;"
assert 2 "main() if(2<3) return 2; else return 1;"
assert 3 "main() {return 3;}"
assert 3 "main() {1;2;3;return 3;}"
assert 4 "main() {a=4; return a;}"
assert 32 "main() return foo();"
assert 32 "main() {a=foo();return a;}"
assert 75 "main() return bar(1, 2);"
assert 75 "main() {a=bar(1, 2);return a;}"
assert 125 "main() return baz(1, 2, 3, 4, 5, 6);"
assert 5 "main() return hoge(); hoge() return 5;"
assert 6 "main() return hoge(6); hoge(a) return a;"
assert 7 "main() return hoge(2, 1, 1, 1, 1, 1); hoge(a, b, c, d, e, f) return a+b+c+d+e+f;"
assert 8 "main() return hoge(5); hoge(a) if(a==0) return 1; else if(a==1) return 1; else return hoge(a-1)+hoge(a-2);"

# &(アドレス)と*(参照)を実装
assert 9 "main() { x = 9; y = &x; return *y; }"
assert 10 "main() { x = 10; y = 11; z = &y + 8; return *z; }"

echo OK

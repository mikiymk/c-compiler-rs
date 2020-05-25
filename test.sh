#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  ./ccc9 "$input" > tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" =  "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 "0;"
assert 42 "42;"
assert 21 "5+20-4;"
assert 41 " 12 + 34 - 5 ;"
assert 47 "5+6*7;"
assert 15 "5*(9-6);"
assert 4 "(3+5)/2;"
assert 25 "+5-4*-5;"
assert 1 "1 < 5;"
assert 0 "2+3>=4*5;"
assert 63 "(3*7+(5-1==-3))*3;"
assert 82 "32+5; 54; 100-18;"
assert 5 "a=5;"
assert 6 "a=3;a*2;"
assert 6 "a=5;b=1;a+b;"
assert 1 "a=b=c=d=e=f=g=h=i=j=k=l=m=n=o=p=q=r=s=t=u=v=w=x=y=z=1;a;"
assert 6 "foo = 1; bar = 2 + 3; foo + bar;"
assert 89 "return 89; 7+5;"
assert 53 "a=53; return a; a=0; b=2;"
assert 32 "a=32; if (4 > 2) return a; 5+3;"
assert 10 "a=100; b=10; if (11 == b) 5; else a / b;"
assert 38 "tkg=10; while( tkg) tkg=tkg-1;38;"
assert 57 "s=2;for(i=0;i<=10;i=i+1)s=s+i;return s;"
assert 123 "s=93; for(i=0;i<=10;i=i+1){for(i2=0;i2<=i;i2=i2+1){if(i*i2>s)s=s+i+i2+10;}}return s;"
assert 47 "a=b=3;b=c=93; if(a<b){c=47;}return c;"

echo OK


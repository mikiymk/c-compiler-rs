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

echo OK


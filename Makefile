CFLAGS=-std=c11 -g -static

ccc9: *.rs
	rustc ccc9.rs

func.o: func.c

test: ccc9 func.o
	./test.sh

clean:
	rm -f ccc9 *.o *~ tmp*

.PHONY: test clean


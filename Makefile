CFLAGS=-std=c11 -g -static

ccc9:
	cargo build

func.o: func.c

test: ccc9 func.o
	./test.sh

clean:
	rm -f ccc9 *.o *~ tmp*

.PHONY: test clean


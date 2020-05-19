CFLAGS=-std=c11 -g -static

ccc9: ccc9.rs token.rs parse.rs assemble.rs
	rustc ccc9.rs

test: ccc9
	./test.sh

clean:
	rm -f ccc9 *.o *~ tmp*

.PHONY: test clean


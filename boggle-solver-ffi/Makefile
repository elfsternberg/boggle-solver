GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

build:
	$(CARGO_BIN) build --release
	$(GCC_BIN) -o ./example/csolve ./example/csolve.c -Isrc  -L. -l:target/release/libboggle_solver.so

run: build
	./example/csolve

clean:
	$(CARGO_BIN) clean
	rm -f ./example/csolve


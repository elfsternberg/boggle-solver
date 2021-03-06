.PHONY: default help build release test clean install

SOURCE_FILES = $(wildcard src/*.rs src/**/*.rs)
BIN_INSTALL=/usr/local/bin
BINARY= boggle-solve
DEBUG_TARGET=./target/debug/$(BINARY)
RELEASE_TARGET=./target/release/$(BINARY)

default: help

# Makefiles *are* documentation.  They list what the developer cared
# about in generating his project.  This somewhat magical Perl script
# processes the Makefile twice: first to find the longest command
# we're going to be providing help for, and then to print out the help
# table with the right amount of white-space to make two neat columns
# of command & documentation.  The '@' header makes it silent, so you
# don't have to watch *this* particular bit of sausage being made.

help:	## Print this help message
	@M=$$(perl -ne 'm/^((\w|-)*):.*##/ && print length($$1)."\n"' Makefile | \
		sort -nr | head -1) && \
		perl -ne "m/^((\w|-)*):.*##\s*(.*)/ && print(sprintf(\"%s: %s\t%s\n\", \$$1, \" \"x($$M-length(\$$1)), \$$3))" Makefile

$(DEBUG_TARGET): $(SOURCE_FILES)
	cargo build

$(RELEASE_TARGET): $(SOURCE_FILES)
	cargo build --release

build: $(DEBUG_TARGET)	## Build the debug binary

release: $(RELEASE_TARGET)	## Build the release binary

test: 	## Run all the unit tests available
	cargo test
	cargo test --features=large_board
	cargo test --features=threaded
	cargo test --features=large_board,threaded

clean:	## Delete all built and intermediate features
	cargo clean

install: $(RELEASE_TARGET)	## Install the release binary into /usr/local/bin
	install $(RELEASE_TARGET) $(BIN_INSTALL)

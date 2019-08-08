.PHONY: default help build release test clean install

BIN_INSTALL=/usr/local/bin

SOURCE_FILES= $(wildcard src/*.rs src/**/*.rs)

BINARY= mkboggle
DEBUG_TARGET=./target/debug/$(BINARY)
RELEASE_TARGET=./target/release/$(BINARY)

default: help

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

clean:	## Delete all built and intermediate features
	cargo clean

install: $(RELEASE_TARGET)	## Install the release binary into /usr/local/bin
	install $(RELEASE_TARGET) $(BIN_INSTALL)


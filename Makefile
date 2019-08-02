.PHONY: default help build release test clean install

BIN_INSTALL=/usr/local/bin

SOURCE_FILES = $(wildcard src/*.rs src/**/*.rs)

default: help

# Makefiles *are* documentation.  They list what the developer cared
# about in generating his project.  This somewhat magical Perl script
# processes the Makefile twice: first to find the longest command
# we're going to be providing help for, and then to print out the help
# table with the right amount of white-space to make two neat
# columns of command & documentation.  The '@' header makes it silent,
# so you don't have to watch *this* particular bit of sausage being
# made.

help:	## Print this help message
	@M=$$(perl -ne 'm/^((\w|-)*):.*##/ && print length($$1)."\n"' Makefile | \
		sort -nr | head -1) && \
		perl -ne "m/^((\w|-)*):.*##\s*(.*)/ && print(sprintf(\"%s: %s\t%s\n\", \$$1, \" \"x($$M-length(\$$1)), \$$3))" Makefile

target/debug/boggle: $(SOURCE_FILES)
	cargo build

target/release/boggle: $(SOURCE_FILES)
	cargo build --release

build: target/debug/boggle	## Build the debug binary

release: target/release/boggle	## Build the release binary

test: 	## Run all the unit tests available
	cargo test

clean:	## Delete all built and intermediate features
	cargo clean

install: target/release/boggle	## Install the release binary into /usr/local/bin
	install target/release/boggle $BIN_INSTALL


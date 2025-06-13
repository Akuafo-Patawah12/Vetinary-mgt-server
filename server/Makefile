# Makefile for Rust project using Actix Web, Chrono, MongoDB, etc.

# Variables
CARGO = cargo
TARGET = debug
BIN_NAME = server # replace with your actual binary name
PREFIX ?= /usr/local

# Default target
all: build

# Fetch and compile dependencies
install:
	$(CARGO) fetch
	$(CARGO) check

# Build the project
build:
	$(CARGO) build

# Run the project
run:
	$(CARGO) run

# Run tests
test:
	$(CARGO) test

# Clean build artifacts
clean:
	$(CARGO) clean

# Build for release
release:
	$(CARGO) build --release

# Run with release build
run-release:
	$(CARGO) run --release

# Format the code
fmt:
	$(CARGO) fmt

# Lint the code
lint:
	$(CARGO) clippy -- -D warnings

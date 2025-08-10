# Makefile for dupels C FFI example

# Detect OS
UNAME_S := $(shell uname -s)

# Library name and paths
LIB_NAME = dupels_c_ffi
TARGET_DIR = target/release

# Platform-specific settings
ifeq ($(UNAME_S),Linux)
    LIB_EXT = so
    LIB_PREFIX = lib
endif
ifeq ($(UNAME_S),Darwin)
    LIB_EXT = dylib
    LIB_PREFIX = lib
endif
ifeq ($(OS),Windows_NT)
    LIB_EXT = dll
    LIB_PREFIX = 
endif

# Full library path
LIB_FILE = $(TARGET_DIR)/$(LIB_PREFIX)$(LIB_NAME).$(LIB_EXT)
STATIC_LIB = $(TARGET_DIR)/$(LIB_PREFIX)$(LIB_NAME).a

# Compiler settings
CC = gcc
CFLAGS = -Wall -Wextra -std=c99
LDFLAGS = -L$(TARGET_DIR) -l$(LIB_NAME)

# Default target
all: build-rust example-dynamic

# Build the Rust library
build-rust:
	@echo "Building Rust library..."
	cargo build --release
	@echo "Generating C header..."
	@ls -la $(TARGET_DIR)/
	@echo "Library built: $(LIB_FILE)"

# Build example with dynamic linking
example-dynamic: build-rust
	@echo "Building example with dynamic linking..."
	$(CC) $(CFLAGS) -o example example.c $(LDFLAGS)
	@echo "Built example (dynamic)"

# Build example with static linking
example-static: build-rust
	@echo "Building example with static linking..."
	$(CC) $(CFLAGS) -o example-static example.c $(STATIC_LIB) -lpthread -ldl -lm
	@echo "Built example-static"

# Run the example
run: example-dynamic
	@echo "Running example..."
	LD_LIBRARY_PATH=$(TARGET_DIR) ./example

# Run static example
run-static: example-static
	@echo "Running static example..."
	./example-static

# Clean build artifacts
clean:
	rm -f example example-static dupels.h
	cargo clean

# Install library system-wide (Linux/macOS)
install: build-rust
	sudo cp $(LIB_FILE) /usr/local/lib/
	sudo cp dupels.h /usr/local/include/
	sudo ldconfig 2>/dev/null || true

# Show library info
info:
	@echo "Library file: $(LIB_FILE)"
	@echo "Static library: $(STATIC_LIB)"
	@echo "Header file: dupels.h"
	@file $(LIB_FILE) 2>/dev/null || echo "Library not built yet"

# Help
help:
	@echo "Available targets:"
	@echo "  all           - Build Rust library and dynamic example"
	@echo "  build-rust    - Build only the Rust library"
	@echo "  example-dynamic - Build example with dynamic linking"
	@echo "  example-static  - Build example with static linking"  
	@echo "  run           - Run dynamic example"
	@echo "  run-static    - Run static example"
	@echo "  clean         - Clean build artifacts"
	@echo "  install       - Install library system-wide"
	@echo "  info          - Show library information"
	@echo "  help          - Show this help"

.PHONY: all build-rust example-dynamic example-static run run-static clean install info help

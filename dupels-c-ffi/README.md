# Dupels C FFI

This is a C FFI wrapper for the dupels-lib library. This is here so I can attempt making a C++ GUI desktop app for dupels as rust's eGUI library is just not working.

## Building

### Prerequisites

- Rust toolchain
- C compiler (gcc, clang, or MSVC)
- cbindgen (automatically installed as build dependency)

### Build Commands

```bash
# Build everything
make all

# Just build the Rust library
make build-rust

# Build example with dynamic linking
make example-dynamic

# Build example with static linking
make example-static

# Run the example
make run
```

## API Overview

### Core Types

```c
// Configuration structure
typedef struct {
    const char* base_path;      // Directory to search
    unsigned char track_dot_files; // Include hidden files
    unsigned char recursive;    // Search recursively  
    int depth;                 // Maximum depth
    const char* separator;     // Output separator
    unsigned char omit;        // Omit single-file groups
    int max_threads;          // Thread count (0 = auto)
} DupelsConfig;

// Result structure
typedef struct {
    char** output;    // Array of strings
    int count;        // Number of strings
    DupelsError error; // Error code
} DupelsResult;

// Error codes
typedef enum {
    Success = 0,
    NullPointer = 1,
    InvalidPath = 2,
    InvalidUtf8 = 3,
    IoError = 4,
    OutOfMemory = 5
} DupelsError;
```

### Core Functions

```c
// Create a new dupels instance
DupelsHandle* dupels_new(const DupelsConfig* config);

// Free a dupels instance
void dupels_free(DupelsHandle* handle);

// Run the duplicate detection
DupelsError dupels_parse(DupelsHandle* handle);

// Get results as array of strings
DupelsError dupels_get_results(DupelsHandle* handle, DupelsResult* result);

// Get results as single string
char* dupels_get_output_string(DupelsHandle* handle);

// Free string returned by dupels_get_output_string
void dupels_free_string(char* string);

// Free results returned by dupels_get_results
void dupels_free_results(DupelsResult* result);
```

### Convenience Function

```c
// Simple one-shot function
char* dupels_find_duplicates_simple(
    const char* path,
    unsigned char recursive,
    unsigned char include_hidden
);
```

## Usage Examples

### Simple Usage

```c
#include "dupels.h"
#include <stdio.h>

int main() {
    // Find duplicates in current directory
    char* result = dupels_find_duplicates_simple(".", 1, 0);
    
    if (result != NULL) {
        printf("Duplicates:\n%s\n", result);
        dupels_free_string(result);
    }
    
    return 0;
}
```

### Advanced Usage

```c
#include "dupels.h"
#include <stdio.h>

int main() {
    // Create configuration
    DupelsConfig config = {
        .base_path = "/path/to/search",
        .track_dot_files = 0,
        .recursive = 1,
        .depth = 5,
        .separator = "---",
        .omit = 0,
        .max_threads = 4
    };
    
    // Create instance
    DupelsHandle* handle = dupels_new(&config);
    if (handle == NULL) {
        printf("Failed to create dupels instance\n");
        return 1;
    }
    
    // Run analysis
    DupelsError error = dupels_parse(handle);
    if (error != Success) {
        printf("Error during parsing: %d\n", error);
        dupels_free(handle);
        return 1;
    }
    
    // Get results
    DupelsResult result = {0};
    error = dupels_get_results(handle, &result);
    
    if (error == Success) {
        for (int i = 0; i < result.count; i++) {
            printf("Line %d: %s\n", i + 1, result.output[i]);
        }
        dupels_free_results(&result);
    }
    
    dupels_free(handle);
    return 0;
}
```

## Memory Management

**Important**: Always free memory returned by the API:

- Use `dupels_free_string()` for strings from `dupels_get_output_string()`
- Use `dupels_free_results()` for results from `dupels_get_results()`  
- Use `dupels_free()` for handles from `dupels_new()`

## Linking

### Dynamic Linking

```bash
gcc -o myprogram myprogram.c -L./target/release -ldupels_c_ffi
```

### Static Linking  

```bash
gcc -o myprogram myprogram.c ./target/release/libdupels_c_ffi.a -lpthread -ldl -lm
```

## Error Handling

All functions return error codes or NULL on failure. Always check return values:

```c
DupelsHandle* handle = dupels_new(&config);
if (handle == NULL) {
    // Handle error
    return;
}

DupelsError error = dupels_parse(handle);
if (error != Success) {
    // Handle error
    dupels_free(handle);
    return;
}
```

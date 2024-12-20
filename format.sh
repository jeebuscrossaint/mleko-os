#!/bin/bash

# Format all C and header files in src directory
find src -type f \( -name "*.c" -o -name "*.h" \) -exec clang-format -i {} \;

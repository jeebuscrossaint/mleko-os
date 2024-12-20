#!/usr/bin/env python3
import json
import os
import glob

def generate_compile_commands():
    commands = []
    src_dir = "src"

    # Match the flags from Makefile
    flags = [
        "-ffreestanding",
        "-O2",
        "-Wall",
        "-Wextra",
        "-Werror",
        "-Wformat=2",
        "-fstack-protector-strong",
        "-std=c17",
        "-m64",
        "-mno-red-zone",
        f"-I{src_dir}/include"
    ]

    # Find all .c files
    c_files = glob.glob(f"{src_dir}/**/*.c", recursive=True)

    for file in c_files:
        command = {
            "directory": os.getcwd(),
            "command": f"clang {' '.join(flags)} -c {file}",
            "file": file
        }
        commands.append(command)

    with open("compile_commands.json", "w") as f:
        json.dump(commands, f, indent=2)

if __name__ == "__main__":
    generate_compile_commands()

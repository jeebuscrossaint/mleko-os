# runix

*"rust written microkernel soon hopefully"*

ok so far i think you need to switch to nightly rust and then run cargo install bootimage and rustup component add llvm-tools-preview and then you can run `cargo bootimage` and then you can run the script `boot-qemu.sh` requires bash of course and qemu installed and then you can see the kernel booting in qemu. has a base vga driver rn.
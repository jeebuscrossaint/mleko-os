# choqolah-milk-OS

*My first delve into operating system development, as I am interested in low level programming. Simply made for fun and from :heart: *

-----

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [License](#license)

-----

### Installation 

#### Deps
```
grub | git 
```

First thing to know is if you have GRUB installed, if not please do so. 

Once you have installed GRUB (GRand Unified Bootloader), please clone this repository
```
git clone https://github.com/jeebuscrossaint/choqolah-milk-OS
cd choqolah-milk-OS/
```
Next, run `make install`

Once that is completed, add the following lines to your grub.cfg after where it says `### END /etc/grub.d/30_os-prober ###`

```
### BEGIN MY KERNEL ###

menuentry 'choqolah milk OS' {
    multiboot /boot/mykernel.bin
    boot
}

### END MY KERNEL ###
```

### Usage

As for now, there is no real use for this operating system, as I said I've made this for fun as a project, and it probably won't ever do anything that something like the Linux kernel or the Windows NT kernel or any BSD or any macOS cannot do. For now it just prints one line but I plan to make it some what usable like a shell and stuff. 

### License
I use the MIT license for most things because its the one github provides that basically just says idgaf do whatever

```MIT License

Copyright (c) 2023 Amarnath P.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

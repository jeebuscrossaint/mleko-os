# choqolah-milk-OS

*"My first delve into operating system development, as I am interested in low level programming. Simply made for fun and from :heart: "*

-----

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [License](#license)


-----

### Installation 

#### Average person
If you do not know anything about like CS and whatnot, but still want to try this OS, simply go to the [releases](https://github.com/jeebuscrossaint/choqolah-milk-OS/releases) and download the latest release (which will be the .iso file), or any release really for archival purposes. But I reccommend the most recent one so that way your system doesn't break. Next, burn the ISO to a USB stick. A common ISO burner is Balena Etcher, though for windows users I reccommend Rufus because it is just based. For those using Linux distribution's you can use the `xfburn` package from your distributor. For macOS users, figure your stuff out together or just use balena etcher as mentioned earlier. I've never used a mac so thats why I don't know.

#### Virtualization

Currently the make file only supports the VirtualBox virtualizer because I am right now too busy to learn `qemu`, but will eventually do so. If you just want to virtualize it then you figure it out yourself for qemu, but if you want to use virtualbox, simply:

```
cd choqolah-milk-OS/
make run
```

Follow below if you want to actually install it on bare metal.

#### Deps
```
grub | git | xorriso
```
**Building from source**

First thing to know is if you have [GRUB](https://www.gnu.org/software/grub/) installed, if not please do so. 
You probably already have [git](https://git-scm.com/) installed especially if you are on a linux distro. If not do so.
Then install [xorriso](https://www.gnu.org/software/xorriso/) from your package manager.

Once you have installed the required dependencies please clone this repository.
```
git clone https://github.com/jeebuscrossaint/choqolah-milk-OS
cd choqolah-milk-OS/
```
Next, run `make install`

Once that is completed, add the following lines to your grub.cfg after where it says `### END /etc/grub.d/30_os-prober ###`

```
### BEGIN CHOCOLATE MILK KERNEL ###

menuentry 'choqolah milk OS' {
    multiboot /boot/kernel.bin
    boot
}

### END CHOCOLATE MILK KERNEL ###
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


UEFI wouldn't be possible without https://gitlab.com/bztsrc/posix-uefi.git
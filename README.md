# kerokeroOS

Random operating system that doesnt do too much at the moment.
Currently only tested through qemu. Once I get my hands on another system I will try running it on some actual hardware

To create the bootimage, first install bootimage using `cargo install bootimage`, which will give us access to some bootloader code. Afterwards, run `cargo bootimage` to create the ELF file, which contains both the bootloader and the compiled kernel.

To load it into qemu, run `qemu-system-x86_64 -drive format=raw,file=target/path/to/.bin`

# Entry

To compile the kernel we use a linker script specified in the `.cargo.config.toml` file

Inside `linker.ld` (in `src/arch/<arch>/linker.ld`) :
```linkerscript
{{#include ../../../../src/arch/riscv/linker.ld:2}}
```

Which tell the entry point of the BasicOs is the `_entry` function [^note1]

The `_entry` function is defined in `src/arch/<arch>/entry.rs` :
```rust
{{#include ../../../../src/arch/riscv/entry.rs:entry}}
```

> `#[naked]` is used because so that `_entry` entry is not really a function
> 
> > When a normal function is called, the rust compiler will automatically add things, before any
> inside the function is run : it saves some registers (see the calling convention of the architecture),
> it push a return address to the stack, align the stack...

> `#[no_mangle]` : by default, the Rust compiler mangles symbol names, but here we need to call this function inside 
> the `linker.ld` so the name of the function need exactly `_entry` in the binary

> `#[link_section = ".text.entry"]` specify the link section so that we put our entry code in the beginning of the OS in
> the `linker.ld` script 
> 
> ```linkerscript
> .text : {
>   *(.text.entry)
>   [...]
> }
> ```

We setup the `sp` (stack pointer) register [^note2] : the stack is where the local variables and other things of the
kernel running will be

At the end of `_entry` we just call the `start` function (this time it is a real one)
```asm
call start
```

As you can see, the `start` function has two parameters which are the HartId and the Address of the device tree which 
we will explain next

---

[^note1] : in RiscV there are 3 CPU mode :
1. The **M**achine mode for OpenSBI
2. The **S**upervisor mode for our OS
3. The **U**ser mode for user programs

[^note2] : The stack pointer register need to be at the end of the memory range

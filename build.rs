fn main() {
    // Rebuild if the linked script has changed

    #[cfg(target_arch = "riscv64")]
    println!("cargo:rerun-if-changed=src/arch/riscv/linker.ld");

    #[cfg(target_arch = "x86_64")]
    {
        println!("cargo:rerun-if-changed=src/arch/x86/linker.ld");
        println!("cargo:rerun-if-changed=src/arch/x86/multiboot.S");
    }

    #[cfg(target_arch = "aarch64")]
    println!("cargo:rerun-if-changed=src/arch/arm/linker.ld");
}

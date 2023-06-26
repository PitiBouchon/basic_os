#[cfg(target_arch = "aarch64")]
mod arm;
#[cfg(target_arch = "riscv64")]
mod riscv;
#[cfg(target_arch = "x86_64")]
mod x86;

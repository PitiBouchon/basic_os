use bit_field::BitField;
use core::ops::BitOr;

const PTE_BIT_VALID: usize = 0;
const PTE_BIT_READ: usize = 1;
const PTE_BIT_WRITE: usize = 2;
const PTE_BIT_EXECUTE: usize = 3;
const PTE_BIT_USER: usize = 4;

#[derive(Debug, Copy, Clone)]
pub struct PTEPermission(pub u8);

impl PTEPermission {
    pub fn new() -> Self {
        Self(0)
    }

    // See section 4.3.1 Addressing and Memory Protection (RiscV privileged manual)
    pub fn valid() -> Self {
        let mut res = 0;
        res.set_bit(PTE_BIT_VALID, true);
        Self(res)
    }

    pub fn is_valid(&self) -> bool {
        self.0.get_bit(PTE_BIT_VALID)
    }

    pub fn read() -> Self {
        let mut res = 0;
        res.set_bit(PTE_BIT_READ, true);
        Self(res)
    }

    pub fn is_read(&self) -> bool {
        self.0.get_bit(PTE_BIT_READ)
    }

    pub fn write() -> Self {
        let mut res = 0;
        res.set_bit(PTE_BIT_WRITE, true);
        Self(res)
    }

    pub fn is_write(&self) -> bool {
        self.0.get_bit(PTE_BIT_WRITE)
    }

    pub fn execute() -> Self {
        let mut res = 0;
        res.set_bit(PTE_BIT_EXECUTE, true);
        Self(res)
    }

    pub fn is_execute(&self) -> bool {
        self.0.get_bit(PTE_BIT_EXECUTE)
    }

    pub fn user() -> Self {
        let mut res = 0;
        res.set_bit(PTE_BIT_USER, true);
        Self(res)
    }

    pub fn is_user(&self) -> bool {
        self.0.get_bit(PTE_BIT_USER)
    }

    // TODO : set_global (maybe set_dirty and set_access)
}

impl BitOr for PTEPermission {
    type Output = PTEPermission;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

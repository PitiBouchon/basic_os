use crate::entry::addr::{PageOffset, PhysicalAddr, Ppn};
use crate::entry::perm::PTEPermission;
use bit_field::BitField;

pub mod addr;
pub mod perm;

#[derive(Debug, Clone)]
pub struct PageTableEntry(pub usize);

#[derive(Debug, Eq, PartialEq)]
pub enum EntryKind {
    Leaf,
    Branch(PhysicalAddr),
    NotValid,
}

impl PageTableEntry {
    pub fn new(ppn: Ppn, rsw: u8, perm: PTEPermission) -> Self {
        let mut value = 0;
        value.set_bits(0..=7, perm.0 as usize);
        value.set_bits(8..=9, rsw as usize); // These are just 2 bits free of use for the supervisor
        #[cfg(target_pointer_width = "32")]
        value.set_bits(10..=31, ppn.0);
        #[cfg(target_pointer_width = "64")]
        value.set_bits(10..=53, ppn.0);
        Self(value)
    }

    fn ppn(&self) -> Ppn {
        if cfg!(target_pointer_width = "32") {
            Ppn(self.0.get_bits(10..=31))
        } else {
            Ppn(self.0.get_bits(10..=53))
        }
    }

    pub fn perm(&self) -> PTEPermission {
        PTEPermission(self.0.get_bits(0..8) as u8)
    }

    pub fn convert_to_physical_addr(&self, offset: &PageOffset) -> PhysicalAddr {
        let mut res = 0;
        res.set_bits(0..=11, offset.0 as usize);
        #[cfg(target_pointer_width = "32")]
        res.set_bits(12..=33, self.ppn().0);
        #[cfg(target_pointer_width = "64")]
        res.set_bits(12..=55, self.ppn().0);
        PhysicalAddr(res)
    }

    fn addr_zero_offset(&self) -> PhysicalAddr {
        self.convert_to_physical_addr(&PageOffset(0))
    }

    pub fn is_valid(&self) -> bool {
        self.perm().is_valid()
    }

    pub fn is_read(&self) -> bool {
        self.perm().is_read()
    }

    pub fn is_write(&self) -> bool {
        self.perm().is_write()
    }

    pub fn is_execute(&self) -> bool {
        self.perm().is_execute()
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn kind(&self) -> EntryKind {
        if (!self.is_valid()) || (!self.is_valid() && self.is_write()) {
            return EntryKind::NotValid;
        }
        if self.is_read() || self.is_execute() {
            return EntryKind::Leaf;
        }
        let pa = self.addr_zero_offset();
        EntryKind::Branch(pa)
    }
}

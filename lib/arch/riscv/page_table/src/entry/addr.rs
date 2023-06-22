use crate::PAGE_SIZE;
use bit_field::BitField;
use core::marker::PhantomData;

#[cfg(target_pointer_width = "32")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Sv32;

#[cfg(target_pointer_width = "64")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Sv39;

#[cfg(target_pointer_width = "64")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Sv48;

#[cfg(target_pointer_width = "64")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Sv57;

pub trait SatpMode: Sized + Eq + PartialEq + Copy + Clone {
    const VPN_COUNT: usize;
    fn virtual_page_numbers(va: &VirtualAddr<Self>) -> [VirtualPageNumber; Self::VPN_COUNT];
}

#[cfg(target_pointer_width = "32")]
impl SatpMode for Sv32 {
    const VPN_COUNT: usize = 2;

    fn virtual_page_numbers(va: &VirtualAddr<Sv32>) -> [VirtualPageNumber; Self::N] {
        [
            VirtualPageNumber(va.0.get_bits(12..=21)),
            VirtualPageNumber(va.0.get_bits(22..=31)),
        ]
    }
}

#[cfg(target_pointer_width = "64")]
impl SatpMode for Sv39 {
    const VPN_COUNT: usize = 3;

    fn virtual_page_numbers(va: &VirtualAddr<Sv39>) -> [VirtualPageNumber; Self::VPN_COUNT] {
        [
            VirtualPageNumber(va.0.get_bits(12..=20)),
            VirtualPageNumber(va.0.get_bits(21..=29)),
            VirtualPageNumber(va.0.get_bits(30..=38)),
        ]
    }
}
#[cfg(target_pointer_width = "64")]
impl SatpMode for Sv48 {
    const VPN_COUNT: usize = 4;

    fn virtual_page_numbers(va: &VirtualAddr<Sv48>) -> [VirtualPageNumber; Self::VPN_COUNT] {
        [
            VirtualPageNumber(va.0.get_bits(12..=20)),
            VirtualPageNumber(va.0.get_bits(21..=29)),
            VirtualPageNumber(va.0.get_bits(30..=38)),
            VirtualPageNumber(va.0.get_bits(39..=47)),
        ]
    }
}
#[cfg(target_pointer_width = "64")]
impl SatpMode for Sv57 {
    const VPN_COUNT: usize = 5;

    fn virtual_page_numbers(va: &VirtualAddr<Sv57>) -> [VirtualPageNumber; Self::VPN_COUNT] {
        [
            VirtualPageNumber(va.0.get_bits(12..=20)),
            VirtualPageNumber(va.0.get_bits(21..=29)),
            VirtualPageNumber(va.0.get_bits(30..=38)),
            VirtualPageNumber(va.0.get_bits(39..=47)),
            VirtualPageNumber(va.0.get_bits(48..=56)),
        ]
    }
}

// pub trait VirtualAddrRiscv {
//     fn virtual_page_numbers(&self) -> &[VirtualPageNumber];
// }

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct VirtualAddr<MODE: SatpMode>(usize, PhantomData<MODE>);

#[derive(Debug)]
pub struct VirtualPageNumber(pub usize);

#[derive(Debug)]
pub struct PageOffset(pub u16);

pub const fn page_round_down(addr: usize) -> usize {
    addr & !(PAGE_SIZE - 1)
}

pub const fn page_round_up(addr: usize) -> usize {
    page_round_down(addr + PAGE_SIZE - 1)
}

impl<MODE: SatpMode> VirtualAddr<MODE> {
    pub fn virtual_page_numbers(&self) -> [VirtualPageNumber; MODE::VPN_COUNT] {
        MODE::virtual_page_numbers(self)
    }

    pub fn page_offset(&self) -> PageOffset {
        PageOffset((self.0.get_bits(0..=11)) as u16)
    }

    pub fn page_round_down(self) -> Self {
        Self(page_round_down(self.0), PhantomData)
    }

    pub fn page_round_up(self) -> Self {
        Self(page_round_up(self.0), PhantomData)
    }

    pub const fn add_offset(self, offset: usize) -> Self {
        pub const MAX_VIRTUAL_ADDR: usize = 1 << (9 + 9 + 9 + 12 - 1);

        assert!(self.0 + offset <= MAX_VIRTUAL_ADDR);
        Self(self.0 + offset, PhantomData)
    }

    pub const fn sub_offset(self, offset: usize) -> Self {
        assert!(offset <= self.0);
        Self(self.0 - offset, PhantomData)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PhysicalAddr(pub usize);

#[derive(Debug)]
pub struct Ppn(pub usize);

impl PhysicalAddr {
    #[cfg(target_pointer_width = "32")]
    pub fn ppn(&self) -> Ppn {
        Ppn(self.0.get_bits(12..=33))
    }

    #[cfg(target_pointer_width = "64")]
    pub fn ppn(&self) -> Ppn {
        Ppn(self.0.get_bits(12..=55))
    }

    pub fn page_offset(&self) -> PageOffset {
        PageOffset((self.0.get_bits(0..=11)) as u16)
    }
}

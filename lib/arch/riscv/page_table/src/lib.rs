#![no_std]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]

extern crate alloc;

use crate::entry::addr::{PhysicalAddr, SatpMode, VirtualAddr, VirtualPageNumber};
use crate::entry::perm::PTEPermission;
use crate::entry::{EntryKind, PageTableEntry};
use page_alloc::PAGE_ALLOCATOR;

mod entry;

const PAGE_SIZE: usize = 4096;

// 4096 bytes (PAGE_SIZE) / 8 bytes (64 bits) per entry = 512 entries
const ENTRY_COUNT: u16 = 512;

#[derive(Debug)]
#[repr(align(4096))]
pub struct PageTable<MODE: SatpMode>([PageTableEntry; ENTRY_COUNT as usize]);

impl<MODE: SatpMode> PageTable<MODE> {
    pub const fn new() -> Self {
        const ZERO_ENTRY: PageTableEntry = PageTableEntry(0);
        Self([ZERO_ENTRY; ENTRY_COUNT as usize])
    }

    fn get_entry_mut(&mut self, vpn: VirtualPageNumber) -> &mut PageTableEntry {
        &mut self.0[vpn.0]
    }

    fn get_entry(&self, vpn: &VirtualPageNumber) -> &PageTableEntry {
        &self.0[vpn.0]
    }

    pub fn get_phys_addr_perm(&self, va: &VirtualAddr<MODE>) -> (PhysicalAddr, PTEPermission)
    where
        [(); MODE::VPN_COUNT]:,
    {
        let page_numbers = va.virtual_page_numbers().into_iter().rev();
        let mut page_table = self;

        for vpn in page_numbers {
            let entry = page_table.get_entry(&vpn);
            match entry.kind() {
                EntryKind::Leaf => {
                    return (
                        entry.convert_to_physical_addr(&va.page_offset()),
                        entry.perm(),
                    );
                }
                EntryKind::Branch(page_table_addr) => {
                    let new_page_table = unsafe { &*(page_table_addr.0 as *const PageTable<MODE>) };
                    page_table = new_page_table;
                }
                EntryKind::NotValid => panic!("IMPOSSIBLE"),
            }
        }

        panic!("IMPOSSIBLE")
    }

    pub fn map_pages(
        &mut self,
        mut va: VirtualAddr<MODE>,
        mut pa: PhysicalAddr,
        size: usize,
        perm: PTEPermission,
        _rsw: u8,
    ) where
        [(); MODE::VPN_COUNT]:,
    {
        assert!(size > 0);
        let va_end = va.add_offset(size).page_round_up();

        while va != va_end {
            let page_table_entry_leaf = self.walk_alloc(&va);
            // assert!(!page_table_entry_leaf.is_valid());
            // assert!(page_table_entry_leaf.is_zero());
            *page_table_entry_leaf =
                PageTableEntry::new(pa.ppn(), 0, PTEPermission::valid() | perm);
            pa.0 += PAGE_SIZE;
            va = va.add_offset(PAGE_SIZE);
        }
    }

    pub fn walk_alloc(&mut self, va: &VirtualAddr<MODE>) -> &mut PageTableEntry
    where
        [(); MODE::VPN_COUNT]:,
    {
        let mut page_numbers = va.virtual_page_numbers().into_iter().rev();
        let mut page_table = self;
        let mut entry = page_table.get_entry_mut(page_numbers.next().unwrap());

        for vpn in page_numbers {
            match entry.kind() {
                EntryKind::Leaf => break,
                EntryKind::Branch(page_table_addr) => {
                    let new_page_table =
                        unsafe { &mut *(page_table_addr.0 as *mut PageTable<MODE>) };
                    page_table = new_page_table;
                }
                EntryKind::NotValid => {
                    // Allocate a page for a new PageTable
                    let new_page_table_addr = PAGE_ALLOCATOR.kalloc().unwrap().cast().as_ptr();
                    let new_page_table =
                        unsafe { &mut *(new_page_table_addr as *mut PageTable<MODE>) };
                    *entry = PageTableEntry::new(
                        PhysicalAddr(new_page_table_addr as usize).ppn(),
                        0,
                        PTEPermission::valid(),
                    );
                    page_table = new_page_table;
                }
            }
            entry = page_table.get_entry_mut(vpn);
        }

        entry
    }
}

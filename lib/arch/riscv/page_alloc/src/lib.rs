#![feature(allocator_api)]
#![feature(slice_as_chunks)]
#![feature(strict_provenance)]
#![no_std]

use core::alloc::AllocError;
use core::ptr::NonNull;
use spin::Mutex;

#[derive(Debug, Copy, Clone)]
pub struct MyMemoryRegion {
    pub address: u64,
    pub size: u64,
}

pub const PAGE_SIZE: usize = 4096;

pub const fn page_round_down(addr: u64) -> u64 {
    addr & !(PAGE_SIZE as u64 - 1)
}

pub const fn page_round_up(addr: u64) -> u64 {
    page_round_down(addr + PAGE_SIZE as u64 - 1)
}

struct Node {
    next: Option<NonNull<Node>>,
}

pub fn init_page_allocator(free_memory_region: MyMemoryRegion) {
    PAGE_ALLOCATOR.0.lock().init(free_memory_region);
}

// Only used for the PAGE_ALLOCATOR static
struct PageAllocator {
    start: usize,
    end: usize,
    node: Option<NonNull<Node>>,
}
// TODO : It may be a bad idea
unsafe impl Send for PageAllocator {}

pub struct StaticPageAllocator(Mutex<PageAllocator>);

impl PageAllocator {
    fn init(&mut self, free_memory_region: MyMemoryRegion) {
        let start_memory_addr = page_round_up(free_memory_region.address);
        let end_memory_addr = page_round_down(free_memory_region.address + free_memory_region.size);
        self.start = start_memory_addr as usize;
        let mut old_node = NonNull::new(start_memory_addr as *mut Node).unwrap();
        self.node = Some(old_node); // Set the First Node
        unsafe {
            old_node.as_mut().next = None;
        }
        for current_addr in (start_memory_addr..)
            .step_by(PAGE_SIZE)
            .take_while(|&addr| addr + (PAGE_SIZE as u64) < end_memory_addr)
        {
            let mut next_node = NonNull::new(current_addr as *mut Node).unwrap();
            unsafe {
                next_node.as_mut().next = None;
                old_node.as_mut().next = Some(next_node);
            }
            old_node = next_node;
        }
        self.end = usize::from(old_node.addr());
    }
}

pub static PAGE_ALLOCATOR: StaticPageAllocator = StaticPageAllocator(Mutex::new(PageAllocator {
    start: 0,
    end: 0,
    node: None,
}));

impl StaticPageAllocator {
    pub fn start_addr(&self) -> usize {
        self.0.lock().start
    }

    pub fn end_addr(&self) -> usize {
        self.0.lock().end
    }

    pub fn kalloc(&self) -> Result<NonNull<u8>, AllocError> {
        let mut alloc = self.0.lock();
        let first_node_addr = alloc.node.ok_or(AllocError)?;
        unsafe {
            alloc.node = first_node_addr.as_ref().next;
        }

        unsafe {
            memset(first_node_addr.cast(), PAGE_SIZE, 0);
        }
        Ok(first_node_addr.cast())
    }

    pub fn kfree(&self, physical_address: NonNull<u8>) {
        assert_eq!(usize::from(physical_address.addr()) % PAGE_SIZE, 0);
        let mut alloc = self.0.lock();
        assert!(usize::from(physical_address.addr()) >= alloc.start);
        assert!(usize::from(physical_address.addr()) < alloc.end);

        let mut new_node: NonNull<Node> = physical_address.cast();
        unsafe {
            new_node.as_mut().next = alloc.node;
            alloc.node = new_node.as_ref().next;
        }
    }
}

unsafe fn memset(addr: NonNull<usize>, size: usize, value: u8) {
    let start_addr = usize::from(addr.addr());
    let end_addr = usize::from(addr.addr()) + size;
    for addr in start_addr..end_addr {
        unsafe {
            let ptr = addr as *mut u8;
            ptr.write(value);
        }
    }
}

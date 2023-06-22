use crate::interface::mem::MemoryRegion;
use core::mem::size_of;
use fdt::{Fdt, FdtError};
use spin::Once;

static Fdt: Once<Fdt> = Once::new();

pub unsafe fn set_fdt(dtb_ptr: usize) -> Result<(), FdtError> {
    let fdt = Fdt::from_ptr(dtb_ptr as *const u8)?;

    Fdt.call_once(|| fdt);
    Ok(())
}

pub fn get_free_memoy() {}

fn parse_reg(values: &[u8]) -> (u64, u64) {
    let ([addr_values, size_values], reminder) =
        values.as_chunks::<8>() else { panic!("Impossible") };

    assert!(reminder.is_empty());

    let address = unsafe { core::mem::transmute::<[u8; 8], u64>(*addr_values) }.to_be();
    let size = unsafe { core::mem::transmute::<[u8; 8], u64>(*size_values) }.to_be();

    (address, size)
}

fn reserved_memory<'a>(fdt: &'a Fdt) -> impl IntoIterator<Item = MemoryRegion> + 'a {
    fdt.find_node("/reserved-memory")
        .map(|reserved_memory_node| {
            reserved_memory_node.children().map(move |reserved_memory| {
                let reg_prop = reserved_memory
                    .properties()
                    .find(|prop| prop.name == "reg")
                    .unwrap();

                let cell_size = reserved_memory_node.cell_sizes();
                let address_cell = cell_size.address_cells;
                let size_cell = cell_size.size_cells;

                assert_eq!(
                    reg_prop.value.len(),
                    address_cell * size_cell * 4,
                    "Length of reg property invalid"
                ); // Size of u32 = 4 * u8
                assert_eq!(size_cell, 2, "size_cell should equal 2");
                assert_eq!(address_cell, 2, "address_cell should equal 2");

                let (address, size) = parse_reg(reg_prop.value);

                assert_eq!(
                    size_of::<u64>(),
                    size_of::<usize>(),
                    "This is for a 64 system"
                );

                MemoryRegion::new(address as usize, size as usize)
            })
        })
        .unwrap()
        .chain(fdt.memory_reservations().map(|memory_reservation| {
            MemoryRegion::new(
                memory_reservation.address() as usize,
                memory_reservation.size(),
            )
        }))
}

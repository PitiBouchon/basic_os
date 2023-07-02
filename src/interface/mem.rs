#[derive(Debug, Copy, Clone)]
pub struct MemoryRegion {
    pub address: usize,
    pub size: usize,
}

impl MemoryRegion {
    pub fn new(address: usize, size: usize) -> Self {
        Self { address, size }
    }

    pub fn intersect(&self, other: &Self) -> Self {
        if self.address >= other.address {
            return Self {
                address: self.address,
                size: self
                    .size
                    .min(other.size.saturating_sub(self.address - other.address)),
            };
        }
        other.intersect(self)
    }
}

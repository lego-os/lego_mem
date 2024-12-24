/// todo!()
pub struct Region {
    pub mem_start: usize,
    pub total_size: usize,
    pub available_start: usize,
    pub ty: RegionType,
}

/// todo!()
pub enum RegionType {
    Normal,
    Dma,
}

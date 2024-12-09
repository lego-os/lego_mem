use crate::AllocError;

/// 内存映射类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MappingType {
    Private,
    Shared,
}

/// 内存映射的描述
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapAttr {
    // 起始虚拟地址
    pub virt_addr: usize,
    pub phys_addr: usize,
    pub size: usize,
    pub flags: u8,
    pub mapping_type: MappingType,
}

pub trait MemoryMapper: Sync {
    fn map(
        &self,
        virt_addr: usize,
        phys_addr: usize,
        flags: u8,
        mapping_type: MappingType,
    ) -> Result<MapAttr, AllocError>;
    fn unmap(&self, virt_addr: usize, phys_addr: usize, size: usize) -> Result<(), AllocError>;
    fn modify_flags(&self, virt_addr: usize, size: usize, new_flags: u8) -> Result<(), AllocError>;
    fn query(&self, virt_addr: usize) -> Option<MapAttr>;
}

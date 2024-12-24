use core::ptr::NonNull;

use super::{Align, ApFlags};

/// todo!()
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Page {
    pub align: Align,
    pub flags: ApFlags,
    pub addr: usize,
    pub access: usize,
}

impl Page {
    pub fn new(addr: usize, align: Align) -> Self {
        Self {
            align,
            flags: ApFlags::default(),
            addr,
            access: 0,
        }
    }
    pub fn as_ptr<T>(&mut self) -> NonNull<T> {
        unsafe { NonNull::new_unchecked(self.addr as *mut u8 as *mut T) }
    }
}

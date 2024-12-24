use crate::AllocError;

mod flags;
mod layout;
mod page;

pub use flags::ApFlags;
pub use layout::Align;
pub use page::Page;

pub trait PageAllocator: Send + Sync {
    const MIN_PAGE_SIZE: usize;

    fn alloc_pages(&mut self, flags: ApFlags, align: Align) -> Result<Page, AllocError>;

    fn alloc_pages_zero(&mut self, flags: ApFlags, align: Align) -> Result<Page, AllocError> {
        let page = self.alloc_pages(flags, align)?;
        let ptr = page.addr as *mut u8;
        unsafe {
            ptr.write_bytes(0, align.as_size());
        };
        Ok(page)
    }

    fn free_pages(&mut self, page: Page) -> Result<(), AllocError>;
}

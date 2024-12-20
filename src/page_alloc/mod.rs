use crate::AllocError;

mod flags;
mod layout;
mod page;

pub use flags::ApFlags;
pub use layout::{Align, PageLayout};
pub use page::Page;

pub trait PageAllocator: Send + Sync {
    const MIN_PAGE_SIZE: usize;

    fn alloc_pages(&mut self, flags: ApFlags, layout: PageLayout) -> Result<Page, AllocError>;

    fn alloc_pages_zero(&mut self, flags: ApFlags, layout: PageLayout) -> Result<Page, AllocError> {
        let page = self.alloc_pages(flags, layout)?;
        let ptr = page.addr as *mut u8;
        unsafe {
            ptr.write_bytes(0, page.layout.align() as usize);
        };
        Ok(page)
    }

    fn free_pages(&mut self, page: Page) -> Result<(), AllocError>;
}

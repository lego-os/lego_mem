use core::alloc::Layout;

pub trait VMAllocator {
    fn vmalloc(&mut self, layout: Layout) -> *mut u8;
    fn vmfree(&mut self, ptr: *mut u8, layout: Layout);
}

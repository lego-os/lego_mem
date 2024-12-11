#[cfg(any(feature = "vm", feature = "page"))]
use crate::AllocError;
#[cfg(feature = "vm")]
use core::alloc::GlobalAlloc;
#[cfg(feature = "page")]
use core::{fmt::Debug, ptr::NonNull};

/// 物理页式内存分配器接口
#[cfg(feature = "page")]
pub unsafe trait PageAllocator: Debug + Sync {
    /// 初始化物理页式内存分配器，init函数的过程应该是互斥的，OS启动之后，这个函数只能被互斥的执行一次
    /// 参数：
    ///     - alloc_start: 可分配内存的开始地址
    /// 返回值:
    ///     - 初始化成功，返回 ()
    ///     - 初始化失败，可能由于start_addr或end_addr未对齐最小页尺寸或其他逻辑错误
    unsafe fn init(&mut self, alloc_start:usize) -> Result<(), AllocError>;

    /// 总物理内存，单位bytes
    /// 返回值：
    ///     - 该函数不应该出现任何错误，并且返回一个至少大于或等于最小页尺寸的值
    fn total_size(&self) -> usize;

    /// 最小物理页大小，单位bytes
    /// 返回值：
    ///     - 函数不应该出现任何错误，返回非零且对齐到最小页尺寸的值
    fn page_size(&self) -> usize;

    /// 可用内存大小，单位bytes
    /// 返回值：
    ///     - 函数不应该出现任何错误，返回值应该大于等于0，并且对齐到最小页尺寸
    fn available_size(&self) -> usize;

    /// 已经分配的内存大小，单位bytes
    /// 返回值：
    ///     - 函数不应该出现任何错误，返回值应该大于等于0，并且对齐到最小页尺寸
    fn allocated_size(&self) -> usize;

    /// 按页申请连续的物理内存
    /// 参数：
    ///     - num_page: 需要的页数；参数值必须大于0，当为0时，不会出现错误，会返回一个空指针；
    /// 返回值：
    ///     - 申请成功，返回一个以对齐到最小页尺寸地址开始的地址指针；
    ///     - 当系统中没有足够的物理内存时，返回OutOfMemory错误；
    unsafe fn alloc_pages(&mut self, num_page: usize) -> Result<NonNull<u8>, AllocError>;

    /// 按页释放连续的物理内存
    /// 参数：
    ///     - ptr: 一个对齐到最小页尺寸的地址指针
    ///     - num_page: 需要的页数；参数值必须大于0，当为0时，应该报错 IllegalAddr；
    /// 返回值：
    ///     - 释放成功，返回()
    ///     - 如果ptr未对齐到最小页尺寸，返回 Misaligned 错误；
    ///     - 如果num_page为0，返回 IllegalAddr 错误；
    ///     - 如果ptr为空指针，返回 NullPointer 错误；
    ///     - 如果ptr不是通过alloc_pages函数获取的内存，可能会出现破坏性的错误，这是非常危险的，应该避免这样的行为；
    unsafe fn free_pages(&mut self, ptr: NonNull<u8>, num_page: usize) -> Result<(), AllocError>;
}

/// 虚拟内存分配器，继承Rust全局内存分配器
/// 在全局内存分配器中出现错误，应该按照Rust GlobalAlloc API说明处理相应错误
#[cfg(feature = "vm")]
pub unsafe trait VMAllocator: GlobalAlloc {
    /// 初始化物理页式内存分配器，init函数的过程应该是互斥的，OS启动之后，这个函数只能被互斥的执行一次
    /// 参数：
    ///     - alloc_start: 可分配内存的开始地址
    /// 返回值:
    ///     - 初始化成功，返回 ()
    ///     - 初始化失败，可能由于start_addr或end_addr未对齐最小页尺寸或其他逻辑错误
    unsafe fn init(&mut self, alloc_start: usize) -> Result<(), AllocError>;

    /// 总虚拟内存，单位bytes
    /// 返回值：
    ///     - 该函数不应该出现任何错误，并且返回一个至少大于或等于最小页尺寸的值
    fn total_size(&self) -> usize;

    /// 可用内存大小，单位bytes
    /// 返回值：
    ///     - 函数不应该出现任何错误，返回值应该大于等于0，并且对齐到最小页尺寸
    fn available_size(&self) -> usize;

    /// 已经分配的内存大小，单位bytes
    /// 返回值：
    ///     - 函数不应该出现任何错误，返回值应该大于等于0，并且对齐到最小页尺寸
    fn allocated_size(&self) -> usize;
}

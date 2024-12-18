//! # lego_mem
//! lego标准接口组件，内存管理接口
//!
//! ---
//!
//! ## 简介
//!
//! 内存管理分为物理内存管理和虚拟内存管理；物理内存一般以页为单位分配和释放，虚拟内存可以设计为更细粒度的以字节为单位的分配与释放；虚拟内存到物理内存的转换由MMU完成，但需要内核完成虚拟内存到物理内存的映射。
//!
//! ## 设计
//!
//!

#![no_std]
#![feature(error_generic_member_access)]
#![feature(slice_ptr_get)]
#![feature(allocator_api)]

mod piece_alloc;
mod err;
mod mapper;
mod page_alloc;
mod vmalloc;

pub use page_alloc::*;
pub use err::AllocError;
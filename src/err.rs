use core::{alloc::Layout, error::Error, fmt::Display};

/// 内存分配或释放错误类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllocError {
    Misaligned(Layout),
    OutOfMemory(Layout),
    NullPointer(usize),
    IllegalAddr(usize),
}

impl Error for AllocError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

    fn provide<'a>(&'a self, request: &mut core::error::Request<'a>) {
        request.provide_value(self.clone());
    }
}

impl Display for AllocError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AllocError::IllegalAddr(addr) => write!(f, "Illegal address ! [ address: {} ]", addr),
            AllocError::OutOfMemory(layout) => {
                write!(f, "Memory overflow! [ layout size: {} ]", layout.size())
            }
            AllocError::NullPointer(addr) => write!(f, "Null pointer! [ address: {} ]", *addr),
            AllocError::Misaligned(layout) => {
                write!(f, "Layout misaligned! [ layout: {:?} ]", layout)
            }
        }
    }
}

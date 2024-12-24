use core::{error::Error, fmt::Display};

/// 内存分配或释放错误类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllocError {
    Misaligned,
    OutOfMemory,
    NullPointer,
    IllegalAddr,
    Other(core::alloc::AllocError),
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
            AllocError::IllegalAddr => write!(f, "illegal address"),
            AllocError::OutOfMemory => {
                write!(f, "memory overflow")
            }
            AllocError::NullPointer => write!(f, "null pointer"),
            AllocError::Misaligned => {
                write!(f, "access is not aligned")
            }
            AllocError::Other(err) => write!(f, "{}", err),
        }
    }
}

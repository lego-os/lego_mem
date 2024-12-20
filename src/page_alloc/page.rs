use super::{ApFlags, PageLayout};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Page {
    pub layout: PageLayout,
    pub flags: ApFlags,
    pub addr: usize,
    pub access: usize,
}

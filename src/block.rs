use std::ptr::null_mut;

#[repr(C)]
pub struct BlockHeader {
    pub size: usize,              // tamanho do payload
    pub is_free: bool,
    pub next: *mut BlockHeader,   // freelist
}

impl BlockHeader {
    pub fn new(size: usize) -> Self {
        BlockHeader {
            size,
            is_free: true,
            next: null_mut(),
        }
    }

    pub fn as_ptr(&self) -> *const BlockHeader {
        self as *const BlockHeader
    }

    pub fn as_mut_ptr(&mut self) -> *mut BlockHeader {
        self as *mut BlockHeader
    }
}

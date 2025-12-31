#[repr(align(16))]
#[derive(Clone)]
struct AlignedBlock(u8);

pub struct Heap {
    memory: Vec<AlignedBlock>,
}

impl Heap {
    pub fn new(size: usize) -> Self {
        Heap {
            memory: vec![AlignedBlock(0); size],
        }
    }

    pub fn start_ptr(&mut self) -> *mut u8 {
        self.memory.as_mut_ptr() as *mut u8
    }
}
use crate::block::BlockHeader;
use crate::heap::Heap;
use std::ptr::null_mut;

pub const ALIGN: usize = std::mem::align_of::<usize>();

// INVARIANTS:
// - free_list is either null or points to a valid BlockHeader
// - blocks are in free_list iff they are marked as is_free = true
// - blocks do not overlap

pub struct Allocator {
    free_list: *mut BlockHeader,
}

impl Allocator {
    /**
     * Initializes the allocator object
     */

    pub fn new(heap_size: usize) -> Self {
        let mut heap = Heap::new(heap_size);

        let free_list = heap.start_ptr() as *mut BlockHeader;

        unsafe {
            // Initializes a single free block, using all of the heap

            // INVARIANT:
            // free_list starts aligned to the BlockHeader type, because of the AlignedBlock feature of heap.rs
            *free_list = BlockHeader::new(heap_size - std::mem::size_of::<BlockHeader>());
        }

        Allocator { free_list }
    }

    /// Allocates memory of **size** bytes, and returns a pointer to the first byte.
    /// Example:
    ///```
    /// let mut allocator = allocator::Allocator::new(1024);
    /// let foo: *mut u8 = allocator.alloc(16);
    /// unsafe {
    ///     (*foo) = 16;
    ///     assert!(*foo == 16)
    /// }
    ///```

    pub fn alloc(&mut self, size: usize) -> *mut u8 {
        // added size of the header
        let size: usize = size + std::mem::size_of::<BlockHeader>();

        // rounded to align
        let size: usize = Self::round_to_aligned(size, ALIGN);

        let allocated: *mut BlockHeader = self.find_first_fit(size);

        if allocated == null_mut() {
            return null_mut();
        }

        unsafe {
            let payload_size: usize = size - size_of::<BlockHeader>();
            let leftover_size: usize = (*allocated).size - (size + size_of::<BlockHeader>());

            (*allocated).is_free = false;
            (*allocated).size = payload_size;
            self.remove_from_free_list(allocated);

            if leftover_size > size_of::<BlockHeader>() + ALIGN {
                let next_block: *mut BlockHeader =
                    (allocated as *mut u8).add(size) as *mut BlockHeader;

                (*next_block).is_free = true;
                (*next_block).size = leftover_size;

                self.add_to_free_list(next_block);
            }

            return Self::header_to_payload(allocated);
        }
    }

    fn find_first_fit(&mut self, size: usize) -> *mut BlockHeader {
        let mut first_free: *mut BlockHeader = self.free_list;

        unsafe {
            while first_free != null_mut() && (*first_free).size < size {
                first_free = (*first_free).next;
            }

            first_free
        }
    }

    fn remove_from_free_list(&mut self, header: *mut BlockHeader) {
        let mut block_header: *mut BlockHeader = self.free_list;
        let mut prev: *mut BlockHeader = self.free_list;

        unsafe {
            if header == self.free_list {
                self.free_list = (*header).next;
                return;
            }

            while (*block_header).next != null_mut() && block_header != header {
                prev = block_header;
                block_header = (*block_header).next;
            }

            if block_header == header {
                (*prev).next = (*block_header).next;
            }
        }
    }

    fn add_to_free_list(&mut self, header: *mut BlockHeader) {
        let first_node: *mut BlockHeader = self.free_list;

        unsafe {
            if first_node == null_mut() || first_node > header {
                self.free_list = header;
                (*header).next = first_node;
                return;
            }

            let mut prev: *mut BlockHeader = first_node;
            let mut node: *mut BlockHeader = first_node;

            while node != null_mut() && node < header {
                prev = node;
                node = (*node).next;
            }

            (*prev).next = header;
            (*header).next = node;
        }
    }

    fn round_to_aligned(num: usize, align: usize) -> usize {
        (num + align - 1) & !(align - 1)
    }

    unsafe fn header_to_payload(header: *mut BlockHeader) -> *mut u8 {
        unsafe { (header as *mut u8).add(size_of::<BlockHeader>()) }
    }

    unsafe fn payload_to_header(payload: *mut u8) -> *mut BlockHeader {
        unsafe { (payload as *mut u8).sub(size_of::<BlockHeader>()) as *mut BlockHeader }
    }

    /// Deallocates memory previously allocated by **alloc()** and referenced by **ptr**.
    /// Example:
    ///```
    /// let mut allocator = allocator::Allocator::new(1024);
    /// let foo: *mut u8 = allocator.alloc(16);
    /// allocator.free(foo);
    ///```

    pub fn free(&mut self, ptr: *mut u8) {
        unsafe {
            let header: *mut BlockHeader = Self::payload_to_header(ptr);

            (*header).is_free = true;
            self.add_to_free_list(header);
            self.merge_free_list();
        }
    }

    fn merge_free_list(&mut self) {
        let mut free_block: *mut BlockHeader = self.free_list;

        unsafe {
            while free_block != null_mut() {
                let next: *mut BlockHeader = (*free_block).next;

                if next == null_mut() {
                    // Since it is the end of the Linked List, we break out of the loop
                    break;
                }

                let is_adjacent_with_next: bool = (free_block as *mut u8)
                    .add((*free_block).size + size_of::<BlockHeader>())
                    == ((*free_block).next as *mut u8);

                if is_adjacent_with_next {
                    let next_size: usize = (*next).size + size_of::<BlockHeader>();

                    (*free_block).size += next_size;
                    self.remove_from_free_list(next);
                } else {
                    free_block = next;
                }
            }
        }
    }
}

# Memory Allocator

A small project made by me, using Rust, in order to understand a bit better how memory allocators work.

For this one, the memory heap is simulated by a Vector of unsigned bytes (`u8`), as seen in the `Heap` structure at `heap.rs`.

As the caller uses `Allocator::alloc()` to allocate memory, the Heap is segmented into memory blocks, represented by "block headers". An important invariant is that memory blocks never overlap.

When `Allocator::new()` is called, only one memory block is created and fills all the heap. As the caller allocs and frees (with `Allocator::free()`) memory, the blocks can split and merge again. For keeping track of the available memory, the `Allocator` object maintains a linked list with all free memory blocks (the `free_list` attribute).

## Methods

**Allocator::alloc(size: usize)**
Allocates a block of memory of the requested size. If enough free space is available, returns a pointer to the start of the allocated block. Otherwise, returns `None`. Allocation may split a free block if it is larger than requested.

**Allocator::free(ptr: Pointer)**
Frees a previously allocated block of memory, identified by its pointer. After freeing, adjacent free blocks may be merged to reduce fragmentation.

## Testing
To run the tests and verify the allocator's behavior, use:

```
cargo test
```

---
This project is for educational purposes.
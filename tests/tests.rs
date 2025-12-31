use allocator::Allocator;

#[test]
fn allocator_initializes() {
    let alloc = Allocator::new(1024);
    assert!(&alloc as *const _ as usize != 0);
}

#[test]
fn alloc_works() {
    let mut allocator = Allocator::new(1024);
    
    let num: *mut u8 = allocator.alloc(16);    
    
    unsafe {
        *(num) = 16;
        assert!(*(num) == 16);
    }
}
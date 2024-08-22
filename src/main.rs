struct BumpAllocator {
    memory_start: *mut u8,
    memory_end: *mut u8,
    bump_pointer: *mut u8,
}

impl BumpAllocator {
    pub fn new(start: *mut u8, size: usize) -> Self {
        BumpAllocator {
            memory_start: start,
            memory_end: unsafe { start.add(size) },
            bump_pointer: start,
        }
    }

    pub fn alloc(&mut self, size: usize) -> *mut u8 {
        let current_ptr = self.bump_pointer;
        let next_ptr = unsafe { self.bump_pointer.add(size) };

        if next_ptr <= self.memory_end {
            self.bump_pointer = next_ptr;
            current_ptr
        } else {
            // return a null pointer if it runs out of memory
            std::ptr::null_mut()
        }
    }
}

fn main() {
    let mut memory = [0u8; 1024];
    let mut allocator = BumpAllocator::new(memory.as_mut_ptr(), memory.len());

    let ptr1 = allocator.alloc(256);
    assert!(!ptr1.is_null());

    let ptr2 = allocator.alloc(512);
    assert!(!ptr2.is_null());

    let ptr3 = allocator.alloc(512);
    assert!(!ptr3.is_null());
}

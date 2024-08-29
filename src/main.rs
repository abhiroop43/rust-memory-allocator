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

    pub fn alloc_aligned(&mut self, size: usize, align: usize) -> *mut u8 {
        if align > self.memory_end as usize - self.memory_start as usize {
            return std::ptr::null_mut(); // alignment too large
        }

        if size > self.memory_end as usize - self.memory_start as usize {
            return std::ptr::null_mut(); // size too large
        }

        let current = self.bump_pointer as usize;
        let aligned = (current + align - 1) & !(align - 1);
        let offset = aligned - current;

        if current % align == 0 {
            let next_ptr = unsafe { self.bump_pointer.add(size) };

            if next_ptr <= self.memory_end {
                self.bump_pointer = next_ptr;
                return self.bump_pointer;
            } else {
                return std::ptr::null_mut();
            }
        }

        let next_ptr = unsafe { self.bump_pointer.add(size + offset) };

        if next_ptr <= self.memory_end {
            self.bump_pointer = next_ptr;
            aligned as *mut u8
        } else {
            std::ptr::null_mut()
        }
    }

    // deallocation
    pub fn reset(&mut self) {
        self.bump_pointer = self.memory_start;
    }

    pub fn alloc_with_free_list(&mut self, size: usize) -> *mut u8 {
        let current_ptr = self.bump_pointer;
        let next_ptr = unsafe { self.bump_pointer.add(size) };

        if next_ptr <= self.memory_end {
            self.bump_pointer = next_ptr;
            current_ptr
        } else {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_allocation() {
        let mut memory = [0u8; 1024];
        let mut BumpAllocator = BumpAllocator::new(memory.as_mut_ptr(), memory.len());

        let ptr1 = BumpAllocator.alloc(256);
        assert!(!ptr1.is_null());

        let ptr2 = BumpAllocator.alloc(512);
        assert!(!ptr2.is_null());

        let ptr3 = BumpAllocator.alloc(256);
        assert!(!ptr3.is_null());

        // this should fail
        let ptr4 = BumpAllocator.alloc(1);
        assert!(ptr4.is_null());
    }

    #[test]
    fn test_aligned_allocation() {
        let mut memory = [0u8; 1024];
        let mut allocator = BumpAllocator::new(memory.as_mut_ptr(), memory.len());

        // allocate 128 bytes with 8-byte alignment
        let ptr1 = allocator.alloc_aligned(128, 8);
        assert!(!ptr1.is_null());
        assert_eq!(ptr1 as usize % 8, 0); // check alignment

        // allocate 128 bytes with 16-byte alignment
        let ptr2 = allocator.alloc_aligned(128, 16);
        assert!(!ptr2.is_null());
        assert_eq!(ptr2 as usize % 16, 0); // check alignment

        // allocate 256 bytes with 32-byte alignment
        let ptr3 = allocator.alloc_aligned(256, 32);
        assert!(!ptr3.is_null());
        assert_eq!(ptr3 as usize % 32, 0); // check alignment

        // this should fail (out of memory)
        let ptr4 = allocator.alloc_aligned(640, 8);
        assert!(ptr4.is_null());
    }

    #[test]
    fn test_reset_allocator() {
        let mut memory = [0u8; 1024];
        let mut allocator = BumpAllocator::new(memory.as_mut_ptr(), memory.len());

        let ptr1 = allocator.alloc(256);
        assert!(!ptr1.is_null());

        let ptr2 = allocator.alloc(512);
        assert!(!ptr2.is_null());

        // reset the allocator
        allocator.reset();

        // check if allocation is possible again
        let ptr3 = allocator.alloc(1024);
        assert!(!ptr3.is_null());

        // this should fail (out of memory)
        let ptr4 = allocator.alloc(1);
        assert!(ptr4.is_null());
    }

    #[test]
    fn test_large_alignment() {
        let mut memory = [0u8; 1024];
        let mut allocator = BumpAllocator::new(memory.as_mut_ptr(), memory.len());

        // allocate an alignment larger than the memory block
        let ptr1 = allocator.alloc_aligned(128, 2048);
        assert!(ptr1.is_null()); // should return null as alignment is too large
    }

    #[test]
    fn test_large_size() {
        let mut memory = [0u8; 1024];
        let mut allocator = BumpAllocator::new(memory.as_mut_ptr(), memory.len());

        // allocate a size larger than the memory block
        let ptr1 = allocator.alloc(2048);
        assert!(ptr1.is_null()); // should return null as size is too large
    }
}

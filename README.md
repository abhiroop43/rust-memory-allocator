# Memory Allocator in Rust

## Overview

This project implements a simple memory allocator in Rust, specifically a bump
allocator. The bump allocator is a basic form of memory management that
allocates memory by incrementing a pointer. This project is designed to help
understand the concepts of memory management, allocation, and alignment in
systems programming.

## Features

- Bump Allocation: Allocates memory by moving a pointer forward.
- Aligned Allocation: Ensures memory is allocated at addresses that are
  multiples of a specified alignment.
- Memory Reset: Allows the allocator to reset, reclaiming all allocated memory.
- Safety Checks: Handles edge cases like excessive alignment or size requests.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Basic understanding of Rust and systems programming concepts

### Building the Project

#### 1. Clone the Repository:

```sh
git clone https://github.com/abhiroop43/rust-memory-allocator
cd rust-memory-allocator
```

#### 2. Build the Project:

```sh
cargo build
```

#### 3. Run the Project:

```sh
cargo run
```

### Project Structure

```sh
memory_allocator/
├── Cargo.toml      # Cargo configuration file
└── src/
    └── main.rs     # Main source file implementing the bump allocator
```

### Usage

The `main.rs` file demonstrates the use of the `BumpAllocator`:

```rust
fn main() {
    let mut memory = [0u8; 1024]; // 1 KB of memory
    let mut allocator = BumpAllocator::new(memory.as_mut_ptr(), memory.len());

    // Example allocations
    let ptr1 = allocator.alloc(256);
    assert!(!ptr1.is_null());

    let ptr2 = allocator.alloc(512);
    assert!(!ptr2.is_null());

    // Aligned allocation
    let ptr3 = allocator.alloc_aligned(128, 8);
    assert!(!ptr3.is_null());

    // Reset allocator
    allocator.reset();
}
```

### Implemented Functions

- `new(start: *mut u8, size: usize) -> Self`: Initializes the allocator with a
  memory block.
- `alloc(&mut self, size: usize) -> *mut u8`: Allocates a block of memory of the
  specified size.
- `alloc_aligned(&mut self, size: usize, align: usize) -> *mut u8`: Allocates
  memory with a specified alignment.
- `reset(&mut self)`: Resets the allocator, reclaiming all allocated memory.

### Testing

To test the allocator, you can run:

```sh
cargo test
```

This will execute any tests you've written in the `main.rs` file or in a
separate test module.

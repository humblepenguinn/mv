//! Custom heap allocator used to simulate memory allocation and deallocation

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::error::Result;

/// Represents the state of a block of memory in the heap
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum HeapBlockState {
    Unallocated,
    Allocated,
    Free,
    Leaked,
}
/// Represents a block of memory in the heap
///
/// # Fields
/// - `identifier`: The pointer to the block, which acts as a unique identifier for the allocated memory block
/// - `size`: The size of the block in bytes
/// - `metadata`: A string representing additional data associated with the block
/// - `pointer`: The starting position of the block in the heap
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeapBlock {
    pub(crate) block_state: HeapBlockState,
    pub(crate) current_pointer_identifier: Option<String>,
    pub(crate) dangling_pointer_identifiers: Option<Vec<String>>,
    pub(crate) size: usize,
    pub(crate) metadata: String,
    pub(crate) pointer: usize,
}

/// Represents a heap allocator.
///
/// The `HeapAllocator` simulates a heap memory management system, allowing for allocation and deallocation
/// of memory blocks. It maintains a list of free blocks and a heap where these blocks are stored.
///
/// # Fields
/// - `heap`: A `Vec<HeapBlock>` representing the memory blocks within the heap. Each [HeapBlock](crate::analyzer::heap_allocator::HeapBlock) object
///   can be allocated, free or leaked, and the heap keeps track of all these blocks.
/// - `size`: The total size of the heap in bytes. This defines the maximum capacity of the heap.
/// - `free_list`: A `Vec<(usize, usize)>` representing free memory regions in the heap. Each tuple contains
///   the pointer and end positions of a free block, helping to efficiently allocate and deallocate memory.
#[derive(Serialize, Deserialize)]
pub(crate) struct HeapAllocator {
    heap: Vec<HeapBlock>,
    size: usize,
    free_list: Vec<(usize, usize)>,
}

impl HeapAllocator {
    /// Creates a new heap allocator with a given size
    ///
    /// # Arguments
    /// - `size`: The size of the heap
    ///
    /// # Returns
    /// - [HeapAllocator](crate::analyzer::heap_allocator::HeapAllocator): A new heap allocator
    #[allow(dead_code)]
    fn new(size: usize) -> Self {
        HeapAllocator {
            heap: vec![
                HeapBlock {
                    block_state: HeapBlockState::Unallocated,
                    current_pointer_identifier: None,
                    dangling_pointer_identifiers: None,
                    size: 0,
                    metadata: "".to_string(),
                    pointer: usize::MAX,
                };
                size
            ],
            size,
            free_list: vec![(0, size - 1)],
        }
    }

    /// Allocates a block of memory of the specified size
    ///
    /// # Arguments
    /// - `size`: The size of the block to allocate in bytes
    ///
    /// # Returns
    /// - [Result](crate::error::Result): A result containing either:
    ///    - `usize`: The starting position of the allocated block
    ///    - [Error](crate::error::Error): An error if there is insufficient memory
    #[allow(dead_code)]
    fn allocate(&mut self, size: usize) -> Result<usize> {
        for i in 0..self.free_list.len() {
            let (pointer, end) = self.free_list[i];
            let block_size = end - pointer + 1;

            if block_size >= size {
                let allocated_start = pointer;
                let allocated_end = allocated_start + size - 1;

                if block_size > size {
                    self.free_list[i] = (allocated_end + 1, end);
                } else {
                    self.free_list.remove(i);
                }

                return Ok(allocated_start);
            }
        }

        Err("Insufficient memory".into())
    }

    /// Writes a [HeapBlock](crate::analyzer::heap_allocator::HeapBlock) to a allocated memory block in the heap
    ///
    /// # Arguments
    /// - `pointer`: The pointer to the starting position of the block in the heap
    /// - `block_to_write`: The [HeapBlock](crate::analyzer::heap_allocator::HeapBlock) to write to the heap
    ///
    /// # Returns
    /// - `Result<()>`: An error if the write operation is out of bounds or succeeds
    #[allow(dead_code)]
    fn write(&mut self, pointer: usize, block_to_write: HeapBlock) -> Result<()> {
        let end = pointer + block_to_write.size - 1;

        if pointer >= self.size || end >= self.size {
            return Err("Invalid write operation: out of bounds".into());
        }

        for i in pointer..=end {
            self.heap[i] = HeapBlock {
                block_state: HeapBlockState::Allocated,
                current_pointer_identifier: block_to_write.current_pointer_identifier.clone(),
                dangling_pointer_identifiers: self.heap[pointer]
                    .dangling_pointer_identifiers
                    .clone(),
                size: block_to_write.size,
                metadata: block_to_write.metadata.clone(),
                pointer,
            };
        }

        Ok(())
    }

    /// Utility function to allocate memory and write a [HeapBlock](crate::analyzer::heap_allocator::HeapBlock) to the allocated block
    ///
    /// # Arguments
    /// - `identifier`: The identifier for the block
    /// - `value_size`: The size of the block to allocate in bytes
    ///
    ///
    /// # Returns
    /// - [Result](crate::error::Result): A result containing either:
    ///    - `usize`: The starting position of the allocated block
    ///    - [Error](crate::error::Error): An error if there is insufficient memory
    #[allow(dead_code)]
    fn allocate_and_write(
        &mut self,
        current_pointer_identifier: &String,
        value_size: usize,
    ) -> Result<usize> {
        let ptr = self.allocate(value_size)?;
        self.write(
            ptr,
            HeapBlock {
                block_state: HeapBlockState::Allocated,
                current_pointer_identifier: Some(current_pointer_identifier.clone()),
                dangling_pointer_identifiers: None,
                size: value_size,
                metadata: "".to_string(),
                pointer: ptr,
            },
        )?;

        Ok(ptr)
    }

    /// Frees a block of memory starting at the specified position
    ///
    /// This function marks the memory block as free and adds it to the free list. It also merges adjacent
    /// free blocks to mimic real-world heap behavior
    ///
    /// # Arguments
    /// - `pointer`: The starting position of the block to free in the heap
    /// - `size`: The size of the block to free in bytes
    #[allow(dead_code)]
    fn free(&mut self, pointer: usize, size: usize) {
        for i in pointer..pointer + size {
            self.heap[i] = HeapBlock {
                block_state: HeapBlockState::Free,
                current_pointer_identifier: None,
                dangling_pointer_identifiers: self.heap[pointer]
                    .dangling_pointer_identifiers
                    .clone(),
                size,
                metadata: "Free Block".to_string(),
                pointer: pointer,
            };
        }

        self.free_list.push((pointer, pointer + size - 1));
        self.merge_free_blocks();
    }

    /// Updates the metadata of a block of memory starting at the specified position
    ///
    /// # Arguments
    /// - `pointer`: The starting position of the block in the heap
    /// - `metadata`: The new metadata to assign to the block
    ///
    /// # Returns
    /// - `Result<()>`: An error if the update operation is out of bounds or succeeds
    #[allow(dead_code)]
    fn update_metadata(&mut self, pointer: usize, metadata: String) -> Result<()> {
        let end = pointer + self.heap[pointer].size - 1;

        if pointer >= self.size || end >= self.size {
            return Err("Invalid metadata update operation: out of bounds".into());
        }

        for i in pointer..=end {
            self.heap[i].metadata = metadata.clone();
        }

        Ok(())
    }

    /// Updates the dangling pointers of a block of memory starting at the specified position
    /// with the specified dangling pointer identifier
    ///
    /// # Arguments
    /// - `pointer`: The starting position of the block in the heap
    /// - `dangling_pointer_identifier`: The new dangling pointer identifier to assign to the block
    ///
    /// # Returns
    /// - `Result<()>`: An error if the update operation is out of bounds or succeeds
    #[allow(dead_code)]
    fn insert_dangling_pointer(
        &mut self,
        pointer: usize,
        dangling_pointer_identifier: String,
    ) -> Result<()> {
        let end = pointer + self.heap[pointer].size - 1;

        if pointer >= self.size || end >= self.size {
            return Err("Invalid dangling pointers update operation: out of bounds".into());
        }

        for i in pointer..=end {
            if self.heap[i].dangling_pointer_identifiers == None {
                self.heap[i].dangling_pointer_identifiers =
                    Some(vec![dangling_pointer_identifier.clone()]);
            } else {
                self.heap[i]
                    .dangling_pointer_identifiers
                    .as_mut()
                    .unwrap()
                    .push(dangling_pointer_identifier.clone());
            }
        }

        Ok(())
    }

    /// Removes the dangling pointers of a block of memory starting at the specified position
    ///
    /// # Arguments
    /// - `pointer`: The starting position of the block in the heap
    /// - `dangling_pointer_identifier`: The dangling pointer identifier to remove from the block
    ///
    /// # Returns
    /// - `Result<()>`: An error if the update operation is out of bounds or succeeds
    #[allow(dead_code)]
    fn remove_dangling_pointer(
        &mut self,
        pointer: usize,
        dangling_pointer_identifier: String,
    ) -> Result<()> {
        let end = pointer + self.heap[pointer].size - 1;

        if pointer >= self.size || end >= self.size {
            return Err("Invalid dangling pointers update operation: out of bounds".into());
        }

        for i in pointer..=end {
            if self.heap[i].dangling_pointer_identifiers != None {
                let dangling_pointer_identifiers =
                    self.heap[i].dangling_pointer_identifiers.as_mut().unwrap();
                let index = dangling_pointer_identifiers
                    .iter()
                    .position(|x| *x == dangling_pointer_identifier);

                if index != None {
                    dangling_pointer_identifiers.remove(index.unwrap());
                }
            }
        }

        Ok(())
    }

    /// Marks a block of memory as leaked
    ///
    /// # Arguments
    /// - `pointer`: The starting position of the block in the heap
    #[allow(dead_code)]
    fn leak(&mut self, pointer: usize, size: usize) {
        for i in pointer..pointer + size {
            self.heap[i] = HeapBlock {
                block_state: HeapBlockState::Leaked,
                current_pointer_identifier: Some("Leaked Block".to_string()),
                dangling_pointer_identifiers: self.heap[pointer]
                    .dangling_pointer_identifiers
                    .clone(),
                size,
                metadata: "Leaked Block".to_string(),
                pointer: pointer,
            };
        }
    }

    /// Merges adjacent free blocks in the free list
    #[allow(dead_code)]
    fn merge_free_blocks(&mut self) {
        self.free_list.sort_by(|a, b| a.0.cmp(&b.0));

        let mut merged_list = Vec::new();
        let mut current = self.free_list[0];

        for &(next_start, next_end) in &self.free_list[1..] {
            if current.1 >= next_start - 1 {
                current.1 = std::cmp::max(current.1, next_end);
            } else {
                merged_list.push(current);
                current = (next_start, next_end);
            }
        }

        merged_list.push(current);
        self.free_list = merged_list;
    }

    /// Builds a list of all memory blocks in the heap in a format suitable for visualization
    ///
    /// # Returns
    /// - `Vec<HeapBlock>`: A list of memory blocks in the heap
    #[allow(dead_code)]
    fn get_heap(&self) -> Vec<HeapBlock> {
        let mut seen = HashSet::new();

        self.heap
            .iter()
            .filter(|block| {
                if seen.contains(&block.pointer) || block.size == 0 {
                    false
                } else {
                    seen.insert(block.pointer);
                    true
                }
            })
            .cloned()
            .collect()
    }
}

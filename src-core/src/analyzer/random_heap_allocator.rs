//! Custom heap allocator used to simulate memory allocation and deallocation

use log::info;
use rand::{rng, Rng};

use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};

use crate::error::Result;

use super::heap_allocator::{HeapBlock, HeapBlockState};

/// Represents a heap allocator.
///
/// The `HeapAllocator` simulates a heap memory management system, allowing for allocation and deallocation
/// of memory blocks. It maintains a list of free blocks and a heap where these blocks are stored.
///
/// This is the random version of the heap allocator, where memory blocks are allocated randomly in the heap.
/// It supports infinite memory by dynamically resizing the heap when allocation fails.
///
/// # Fields
/// - `heap`: A `Vec<HeapBlock>` representing the memory blocks within the heap. Each [HeapBlock](crate::analyzer::heap_allocator::HeapBlock) object
///   can be allocated, free or leaked, and the heap keeps track of all these blocks.
/// - `size`: The total size of the heap in bytes. This defines the maximum capacity of the heap.
/// - `free_list`: A `Vec<(usize, usize)>` representing free memory regions in the heap. Each tuple contains
///   the pointer and end positions of a free block, helping to efficiently allocate and deallocate memory.
/// - `infinite_memory`: Whether the heap should grow dynamically when allocation fails.
/// - `growth_factor`: The factor by which to multiply the heap size when resizing (default: 2.0).
/// - `max_size`: Optional maximum size limit for the heap (None means unlimited).
#[derive(Serialize, Deserialize)]
pub(crate) struct HeapAllocator {
    heap: Vec<HeapBlock>,
    size: usize,
    free_list: Vec<(usize, usize)>,
    infinite_memory: bool,
    growth_factor: f64,
    max_size: Option<usize>,
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
    pub(crate) fn new(size: usize) -> Self {
        Self::new_with_options(size, false, 2.0, None)
    }

    /// Creates a new heap allocator with infinite memory support
    ///
    /// # Arguments
    /// - `initial_size`: The initial size of the heap
    /// - `growth_factor`: The factor by which to multiply the heap size when resizing (default: 2.0)
    /// - `max_size`: Optional maximum size limit for the heap (None means unlimited)
    ///
    /// # Returns
    /// - [HeapAllocator](crate::analyzer::heap_allocator::HeapAllocator): A new heap allocator with infinite memory
    pub(crate) fn new_infinite(
        initial_size: usize,
        growth_factor: f64,
        max_size: Option<usize>,
    ) -> Self {
        Self::new_with_options(initial_size, true, growth_factor, max_size)
    }

    /// Creates a new heap allocator with custom options
    ///
    /// # Arguments
    /// - `size`: The initial size of the heap
    /// - `infinite_memory`: Whether the heap should grow dynamically when allocation fails
    /// - `growth_factor`: The factor by which to multiply the heap size when resizing
    /// - `max_size`: Optional maximum size limit for the heap (None means unlimited)
    ///
    /// # Returns
    /// - [HeapAllocator](crate::analyzer::heap_allocator::HeapAllocator): A new heap allocator
    pub(crate) fn new_with_options(
        size: usize,
        infinite_memory: bool,
        growth_factor: f64,
        max_size: Option<usize>,
    ) -> Self {
        HeapAllocator {
            heap: vec![
                HeapBlock {
                    block_state: HeapBlockState::Unallocated,
                    current_pointer_identifier: None,
                    dangling_pointer_identifiers: None,
                    size: 0,
                    metadata: "Unallocated Block".to_string(),
                    pointer: usize::MAX,
                };
                size
            ],
            size,
            free_list: vec![(0, size - 1)],
            infinite_memory,
            growth_factor,
            max_size,
        }
    }

    /// Resizes the heap to accommodate more memory
    ///
    /// # Arguments
    /// - `required_size`: The minimum size needed for the allocation
    ///
    /// # Returns
    /// - `Result<()>`: An error if the resize operation fails or succeeds
    fn resize_heap(&mut self, required_size: usize) -> Result<()> {
        if !self.infinite_memory {
            return Err("Infinite memory is disabled".into());
        }

        let new_size = if let Some(max_size) = self.max_size {
            let calculated_size = (self.size as f64 * self.growth_factor) as usize;
            let min_required = self.size + required_size;
            let target_size = std::cmp::max(calculated_size, min_required);
            std::cmp::min(target_size, max_size)
        } else {
            let calculated_size = (self.size as f64 * self.growth_factor) as usize;
            let min_required = self.size + required_size;
            std::cmp::max(calculated_size, min_required)
        };

        if new_size <= self.size {
            return Err("Cannot resize heap: new size is not larger than current size".into());
        }

        if let Some(max_size) = self.max_size {
            if new_size > max_size {
                return Err("Cannot resize heap: would exceed maximum size limit".into());
            }
        }

        // Extend the heap with new unallocated blocks
        let old_size = self.size;
        self.heap.resize(
            new_size,
            HeapBlock {
                block_state: HeapBlockState::Unallocated,
                current_pointer_identifier: None,
                dangling_pointer_identifiers: None,
                size: 0,
                metadata: "Unallocated Block".to_string(),
                pointer: usize::MAX,
            },
        );

        // Add the new memory region to the free list
        self.free_list.push((old_size, new_size - 1));
        self.size = new_size;

        info!("Heap resized from {} to {} bytes", old_size, new_size);
        Ok(())
    }

    /// Allocates a block of memory of the specified size randomly in the heap
    ///
    /// # Arguments
    /// - `size`: The size of the block to allocate in bytes
    ///
    /// # Returns
    /// - [Result](crate::error::Result): A result containing either:
    ///    - `usize`: The starting position of the allocated block
    ///    - [Error](crate::error::Error): An error if there is insufficient memory
    pub(crate) fn allocate(
        &mut self,
        size: usize,
        starting_pointer: Option<usize>,
    ) -> Result<(usize, Option<usize>)> {
        let mut starting_pointer = starting_pointer;

        match starting_pointer {
            Some(pointer) => {
                let mut found = false;
                for (start, end) in self.free_list.iter() {
                    // There is a block in the heap that can be allocated at the starting_pointer
                    if *start <= pointer && (pointer + size - 1) <= *end {
                        found = true;
                        break;
                    }
                }

                // This means that another block might have been allocated at the starting_pointer
                if !found {
                    starting_pointer = None;
                }
            }

            _ => {}
        }

        for i in 0..self.free_list.len() {
            let (block_start_pointer, block_end_pointer) = self.free_list[i];
            let mut pointer;
            let mut is_random_start = true;

            if let Some(value) = starting_pointer {
                if block_end_pointer < value || block_start_pointer > value {
                    continue;
                }

                info!("Starting Pointer: {:?}", value);
                pointer = value;
                is_random_start = false;
            } else {
                if block_start_pointer == block_end_pointer {
                    // Represents one byte blocks in the heap
                    pointer = block_start_pointer;
                } else {
                    pointer = rng().random_range(block_start_pointer..block_end_pointer);
                }

                info!("Random Pointer: {:?}", pointer);
            }

            // Ensure the starting pointer is within the bounds of heap
            // This can happen if the heap is resized
            if pointer >= self.size || block_end_pointer >= self.size {
                continue;
            }

            while (block_end_pointer - block_start_pointer + 1) >= size {
                let block_size = block_end_pointer - pointer + 1;

                if block_size >= size {
                    let allocated_start = pointer;
                    let allocated_end = allocated_start + size - 1;

                    if block_size > size {
                        self.free_list[i] = (allocated_end + 1, block_end_pointer);

                        if pointer != block_start_pointer {
                            self.free_list.insert(i + 1, (block_start_pointer, pointer - 1));
                        }
                    } else {
                        if pointer != block_start_pointer {
                            self.free_list[i] = (block_start_pointer, pointer - 1);
                        } else {
                            self.free_list.remove(i);
                        }
                    }

                    self.free_list.retain(|&(_, end)| end < self.size);

                    info!("Block Point Start: {:?}", pointer);
                    info!("Block End: {:?}", allocated_end);
                    info!("Free list: {:?}", self.free_list);

                    if is_random_start {
                        return Ok((allocated_start, Some(pointer)));
                    }

                    return Ok((allocated_start, None));
                } else {
                    pointer = rng().random_range(block_start_pointer..=block_end_pointer);
                }
            }
        }

        // If infinite memory is enabled, try to resize the heap
        if self.infinite_memory {
            info!("Allocation failed, attempting to resize heap...");
            if let Err(e) = self.resize_heap(size) {
                return Err(format!("Failed to resize heap: {}", e).into());
            }

            // Retry allocation after resizing
            return self.allocate(size, starting_pointer);
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
    pub(crate) fn write(&mut self, pointer: usize, block_to_write: HeapBlock) -> Result<()> {
        let end = pointer + block_to_write.size - 1;

        if pointer >= self.size || end >= self.size {
            // If infinite memory is enabled, try to resize the heap
            if self.infinite_memory {
                info!("Write operation out of bounds, attempting to resize heap...");
                if let Err(e) = self.resize_heap(block_to_write.size) {
                    return Err(format!("Failed to resize heap for write operation: {}", e).into());
                }
            } else {
                return Err("Invalid write operation: out of bounds".into());
            }
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
    pub(crate) fn allocate_and_write(
        &mut self,
        current_pointer_identifier: &String,
        value_size: usize,
        starting_pointers: &mut IndexMap<String, usize>,
    ) -> Result<usize> {
        let starting_pointer =
            if let Some(&pointer) = starting_pointers.get(current_pointer_identifier) {
                Some(pointer)
            } else {
                None
            };

        let (ptr, start_pointer) = self.allocate(value_size, starting_pointer)?;

        if let None = starting_pointer {
            starting_pointers
                .insert(current_pointer_identifier.to_string(), start_pointer.unwrap());
            // This is safe since allocate will always return a start_pointer if no starting_pointer is provided to it
        } else if start_pointer != None {
            starting_pointers
                .insert(current_pointer_identifier.to_string(), start_pointer.unwrap());
        }

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
    /// This function marks the memory block as free and adds it to the free list.
    /// free blocks to mimic real-world heap behavior
    ///
    /// # Arguments
    /// - `pointer`: The starting position of the block to free in the heap
    /// - `size`: The size of the block to free in bytes
    pub(crate) fn free(&mut self, pointer: usize, size: usize) {
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
    }

    /// Updates the metadata of a block of memory starting at the specified position
    ///
    /// # Arguments
    /// - `pointer`: The starting position of the block in the heap
    /// - `metadata`: The new metadata to assign to the block
    ///
    /// # Returns
    /// - `Result<()>`: An error if the update operation is out of bounds or succeeds
    pub(crate) fn update_metadata(&mut self, pointer: usize, metadata: String) -> Result<()> {
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
    pub(crate) fn insert_dangling_pointer(
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
    pub(crate) fn remove_dangling_pointer(
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
    pub(crate) fn leak(&mut self, pointer: usize, size: usize) {
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

    /// Utility function to convert unallocated blocks into proper heap blocks with sizes
    ///
    /// This function will iterate over the heap and group contiguous unallocated blocks into
    /// proper `HeapBlock` structures with the correct size, returning a new heap.
    ///
    /// # Returns
    /// - `Vec<HeapBlock>`: A new heap where unallocated blocks have proper sizes.
    fn convert_unallocated_blocks(&self) -> Vec<HeapBlock> {
        let mut new_heap = Vec::new();
        let mut unallocated_start: Option<usize> = None;
        let mut unallocated_size = 0;

        for (i, block) in self.heap.iter().enumerate() {
            if block.current_pointer_identifier.is_none() && block.metadata == "Unallocated Block" {
                if unallocated_start.is_none() {
                    unallocated_start = Some(i);
                }
                unallocated_size += 1;
            } else {
                if let Some(start) = unallocated_start {
                    new_heap.push(HeapBlock {
                        block_state: HeapBlockState::Unallocated,
                        current_pointer_identifier: None,
                        dangling_pointer_identifiers: None,
                        size: unallocated_size,
                        metadata: "Unallocated Block".to_string(),
                        pointer: start,
                    });

                    unallocated_start = None;
                    unallocated_size = 0;
                }

                new_heap.push(block.clone());
            }
        }

        if let Some(start) = unallocated_start {
            new_heap.push(HeapBlock {
                block_state: HeapBlockState::Unallocated,
                current_pointer_identifier: None,
                dangling_pointer_identifiers: None,
                size: unallocated_size,
                metadata: "Unallocated Block".to_string(),
                pointer: start,
            });
        }

        new_heap
    }

    /// Builds a list of all memory blocks in the heap in a format suitable for visualization
    ///
    /// # Returns
    /// - `Vec<HeapBlock>`: A list of memory blocks in the heap
    pub(crate) fn get_heap(&self) -> Vec<HeapBlock> {
        let mut seen = IndexSet::new();

        self.convert_unallocated_blocks()
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
